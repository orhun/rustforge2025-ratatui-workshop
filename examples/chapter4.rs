use std::{collections::HashMap, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint::*, Direction, Layout, Rect},
    style::{palette::tailwind, Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, BorderType, Chart, Dataset, GraphType,
        RenderDirection, Row, Sparkline, Table,
    },
    DefaultTerminal, Frame,
};
use sysinfo::{Disks, Networks, ProcessesToUpdate, System};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,

    /// System information.
    system: System,
    networks: Networks,

    /// Data collected from the system.
    cpu_data: Vec<CpuData>,
    memory_data: Vec<MemoryData>,
    disk_data: Vec<DiskData>,
    network_data: HashMap<String, Vec<NetworkData>>,
}

#[derive(Clone, Debug)]
struct CpuData {
    usage: f64,
    point: f64,
}

#[derive(Clone, Debug)]
struct MemoryData {
    usage: f64,
    point: f64,
}

#[derive(Clone, Debug)]
struct DiskData {
    name: String,
    available: u64,
    total: u64,
}

#[derive(Clone, Debug)]
struct NetworkData {
    total_packets: u64,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            system: System::new_all(),
            networks: Networks::new(),
            cpu_data: Vec::new(),
            memory_data: Vec::new(),
            disk_data: Vec::new(),
            network_data: HashMap::new(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.refresh_disks();
        while self.running {
            terminal.draw(|frame| {
                self.render(frame);
                self.refresh(frame.count());
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    pub fn refresh_disks(&mut self) {
        let disks = Disks::new_with_refreshed_list();
        self.disk_data = disks
            .list()
            .iter()
            .map(|disk| DiskData {
                name: disk.name().to_string_lossy().to_string(),
                available: disk.available_space(),
                total: disk.total_space(),
            })
            .collect();
    }

    /// Refresh the system information.
    pub fn refresh(&mut self, frame_count: usize) {
        self.system.refresh_cpu_all();
        let cpu_usage = self.system.global_cpu_usage();
        self.cpu_data.push(CpuData {
            usage: cpu_usage as f64,
            point: frame_count as f64,
        });

        self.system.refresh_memory();
        let memory_usage = self.system.used_memory();
        self.memory_data.push(MemoryData {
            usage: memory_usage as f64,
            point: frame_count as f64,
        });

        self.networks.refresh(true);
        for (interface_name, network) in &self.networks {
            self.network_data
                .entry(interface_name.clone())
                .or_default()
                .push(NetworkData {
                    total_packets: network.packets_received() + network.packets_transmitted(),
                });
        }

        if frame_count % 30 == 0 {
            self.system.refresh_processes(ProcessesToUpdate::All, true);
        }
    }

    /// Renders the user interface.
    ///
    /// Split the area into 6 parts: header, cpu, disk, memory, network, and process.
    ///
    /// The resulting layout is as follows:
    ///
    /// ```
    /// ──────────────────────────Header──────────────────────────
    // ┌CPU───────────────────────────────────────────────────────┐
    // │                                                          │
    // │                                                          │
    // └──────────────────────────────────────────────────────────┘
    // ┌Disks───────────┐┌Memory──────────────────────────────────┐
    // │                ││                                        │
    // │                ││                                        │
    // │                ││                                        │
    // └────────────────┘└────────────────────────────────────────┘
    // ┌Network─────────────────────┐┌Processes───────────────────┐
    // │                            ││                            │
    // │                            ││                            │
    // │                            ││                            │
    // └────────────────────────────┘└────────────────────────────┘
    // ```
    fn render(&mut self, frame: &mut Frame) {
        let [header_area, main_area] = Layout::vertical([Length(1), Min(0)]).areas(frame.area());

        let [cpu_area, middle, bottom] =
            Layout::vertical([Percentage(25), Fill(1), Fill(1)]).areas(main_area);

        let [disk_area, memory_area] = Layout::horizontal([Percentage(30), Fill(1)]).areas(middle);

        let [network_area, process_area] = Layout::horizontal([Fill(1); 2]).areas(bottom);

        self.render_header(frame, header_area);
        self.render_cpu(frame, cpu_area);
        self.render_disks(frame, disk_area);
        self.render_memory(frame, memory_area);
        self.render_networks(frame, network_area);
        self.render_processes(frame, process_area);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Line::from("Ratatop")
                .alignment(Alignment::Center)
                .fg(tailwind::BLUE.c200)
                .bg(tailwind::GRAY.c800)
                .bold(),
            area,
        );
    }

    /// Renders a chart of CPU usage.
    fn render_cpu(&self, frame: &mut Frame, area: Rect) {
        let data = self
            .cpu_data
            .clone()
            .into_iter()
            .map(|v| (v.point, v.usage))
            .collect::<Vec<_>>();

        let datasets = vec![Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(tailwind::GREEN.c400)
            .data(&data)];

        let current_percentage = self.cpu_data.last().map(|v| v.usage).unwrap_or_default();
        let current_percentage_line =
            format!("{:.2}%", current_percentage).fg(match current_percentage {
                0.0..=50.0 => tailwind::GREEN.c400,
                50.0..=80.0 => tailwind::YELLOW.c300,
                _ => tailwind::RED.c600,
            });

        let x_axis = Axis::default()
            .title(current_percentage_line)
            .bounds([0.0, self.cpu_data.len() as f64]);

        let y_axis = Axis::default()
            .bounds([0.0, 100.0])
            .labels(vec![
                "0%".fg(tailwind::GREEN.c400),
                "50%".fg(tailwind::YELLOW.c300),
                "100%".fg(tailwind::RED.c600),
            ])
            .style(tailwind::GRAY.c600);

        let chart = Chart::new(datasets)
            .block(Self::create_pane("CPU").title_alignment(Alignment::Right))
            .style(Style::new().bg(tailwind::GRAY.c900))
            .x_axis(x_axis)
            .y_axis(y_axis);

        frame.render_widget(chart, area);
    }

    /// Renders a bar chart of disk usage.
    fn render_disks(&self, frame: &mut Frame, area: Rect) {
        let bars = self
            .disk_data
            .iter()
            .map(|disk| {
                let name = disk.name.rsplit('/').next().unwrap().to_string();
                let percent = (disk.available as f64 / disk.total as f64 * 100.0) as u64;
                let style = match percent {
                    0..=50 => tailwind::GREEN.c400,
                    51..=80 => tailwind::YELLOW.c300,
                    _ => tailwind::RED.c600,
                };
                Bar::default()
                    .label(name.fg(tailwind::BLUE.c100).into())
                    .value(percent)
                    .style(style)
            })
            .collect::<Vec<_>>();

        let data = BarGroup::default().bars(&bars);
        let bar_chart = BarChart::default()
            .block(Self::create_pane("Disks"))
            .style(Style::new().bg(tailwind::GRAY.c900))
            .direction(Direction::Horizontal)
            .data(data)
            .bar_gap(1)
            .bar_width(1)
            .bar_style(Style::new().on_black());

        frame.render_widget(bar_chart, area);
    }

    /// Renders a chart of memory usage.
    fn render_memory(&self, frame: &mut Frame, area: Rect) {
        let current_percentage =
            self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0;

        let current_percentage_line =
            format!("{:.2}%", current_percentage).fg(match current_percentage {
                0.0..=50.0 => tailwind::GREEN.c400,
                50.0..=80.0 => tailwind::YELLOW.c300,
                _ => tailwind::RED.c600,
            });

        let data = self
            .memory_data
            .clone()
            .into_iter()
            .map(|v| (v.point, v.usage))
            .collect::<Vec<_>>();

        let datasets = vec![Dataset::default()
            .name(current_percentage_line)
            .marker(Marker::Bar)
            .graph_type(GraphType::Line)
            .style(tailwind::BLUE.c400)
            .data(&data)];

        let x_axis = Axis::default().bounds([0.0, self.memory_data.len() as f64]);
        let y_axis = Axis::default().bounds([0.0, self.system.total_memory() as f64]);

        let chart = Chart::new(datasets)
            .style(Style::new().bg(tailwind::GRAY.c900))
            .block(Self::create_pane("Memory"))
            .x_axis(x_axis)
            .y_axis(y_axis);

        frame.render_widget(chart, area);
    }

    /// Renders a sparkline for each network interface.
    fn render_networks(&self, frame: &mut Frame, area: Rect) {
        let block = Self::create_pane("Network");
        let inner_block = block.inner(area);
        frame.render_widget(block, area);

        let mut network_data = self.network_data.iter().collect::<Vec<_>>();
        network_data.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

        let longest_name = network_data
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);

        let [name_area, data_area] = Layout::horizontal([Length(longest_name as u16), Fill(1)])
            .spacing(1)
            .areas(inner_block);

        for (((name, data), name_area), data_area) in network_data
            .into_iter()
            .zip(name_area.rows())
            .zip(data_area.rows())
        {
            let line = Line::from(name.clone()).fg(tailwind::BLUE.c200);
            frame.render_widget(line, name_area);

            let data = data
                .iter()
                .rev()
                .take(data_area.width as usize)
                .rev()
                .map(|v| v.total_packets)
                .collect::<Vec<_>>();

            let sparkline = Sparkline::default()
                .data(data)
                .direction(RenderDirection::LeftToRight)
                .style(tailwind::GREEN.c400);

            frame.render_widget(sparkline, data_area);
        }
    }

    /// Renders a table of processes.
    fn render_processes(&self, frame: &mut Frame, area: Rect) {
        let header = Row::new(vec!["Pid", "Cmd", "CPU%", "Mem%"]).style(tailwind::YELLOW.c200);

        let widths = [Length(10), Fill(2), Fill(1), Fill(1)];
        let mut rows = Vec::new();
        for (pid, process) in self.system.processes() {
            let row = vec![
                pid.to_string(),
                process.name().to_string_lossy().to_string(),
                format!("{:.2}", process.cpu_usage()),
                format!(
                    "{:.2}",
                    process.memory() as f64 / self.system.total_memory() as f64 * 100.0
                ),
            ];
            rows.push(row);
        }
        rows.sort_by(|a, b| {
            a[2].parse::<f64>()
                .unwrap_or_default()
                .partial_cmp(&b[2].parse::<f64>().unwrap_or_default())
                .unwrap()
                .reverse()
        });

        let table = Table::new(
            rows.into_iter()
                .map(|v| Row::new(v).fg(tailwind::GRAY.c400))
                .collect::<Vec<_>>(),
            widths,
        )
        .header(header)
        .style(tailwind::GRAY.c900)
        .row_highlight_style(Style::new().bg(tailwind::GRAY.c800).fg(tailwind::BLUE.c200))
        .highlight_symbol("> ")
        .block(Self::create_pane("Processes"));

        frame.render_widget(table, area);
    }

    /// Creates a bordered block with a title.
    fn create_pane(title: &str) -> Block {
        let title = Line::from_iter([
            "┤ ".fg(tailwind::GRAY.c700),
            title.fg(tailwind::BLUE.c200),
            " ├".fg(tailwind::GRAY.c700),
        ]);
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .title_alignment(Alignment::Left)
            .border_style(tailwind::GRAY.c700)
            .style(Style::new().bg(tailwind::GRAY.c900))
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> Result<()> {
        // Poll every 1/60th of a second in non-blocking mode.
        let timeout = Duration::from_secs_f32(1.0 / 60.0);
        if event::poll(timeout)? {
            match event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
