use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint::*, Layout, Rect},
    style::{Style, Stylize, palette::tailwind},
    text::Line,
    widgets::{Block, BorderType},
};
use sysinfo::System;

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
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            system: System::new_all(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.refresh();
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Refresh the system information.
    pub fn refresh(&mut self) {
        self.system.refresh_all();
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

    fn render_cpu(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Self::create_pane("CPU"), area);
    }

    fn render_disks(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Self::create_pane("Disks"), area);
    }

    fn render_memory(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Self::create_pane("Memory"), area);
    }

    fn render_networks(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Self::create_pane("Network"), area);
    }

    fn render_processes(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Self::create_pane("Processes"), area);
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
