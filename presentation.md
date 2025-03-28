---
title: Cooking up TUIs with Ratatui
sub_title: RustAsia 2025 🐲
author: Orhun Parmaksız
theme:
  path: assets/theme.yml
---

<!-- new_lines: 2 -->

![image:width:50%](assets/rustasia.png)

![image:width:20%](assets/rat-chef.gif)

<!-- column_layout: [1, 2, 1] -->

<!-- column: 1 -->

#### <span style="color: #ffffff">**Welcome to the workshop!**</span>

<!-- reset_layout -->

<!-- column_layout: [1, 9] -->

<!-- column: 1 -->

[](https://github.com/orhun/rustasia2025-ratatui-workshop)

<!-- end_slide -->

<!-- column_layout: [4, 5] -->

<!-- column: 0 -->

<!-- new_lines: 1 -->

![](assets/orhun.jpg)

<!-- column: 1 -->

<!-- new_lines: 1 -->

<!-- pause -->

# **Orhun Parmaksız**

🦀 Open source, Rust and terminals!

🐭 **Ratatui**, **Ratzilla**, **git-cliff**, **binsider**

📦 **Arch Linux** (btw)

---

https://orhun.dev

https://github.com/orhun

https://youtube.com/@orhundev

<!-- end_slide -->

# About the workshop

<!-- pause -->

<!-- column_layout: [3, 2] -->

<!-- column: 1 -->

![image:width:70%](assets/ratatui-spin.gif)

<!-- column: 0 -->

## Goals

- Get started with Rust & Ratatui
- Grasp the terminal UI concepts
- Build a real-world application

<!-- pause -->

## Structure

- 2 hours in total
- Split up into 5 chapters
- Hands-on coding

<!-- end_slide -->

# What are we gonna cook?

```bash +exec +acquire_terminal
cargo run
```

<!-- end_slide -->

# Schedule

| Duration   | Chapter       | Topic                                                      |
| ---------- | ------------- | ---------------------------------------------------------- |
| **10 min** | **Chapter 1** | **Setup** - Install Rust, cargo-generate, create project   |
| **15 min** | **Chapter 2** | **Manage State** - Use `sysinfo`, refresh data             |
| **30 min** | **Chapter 3** | **Layout & Styling** - Define UI structure, apply styles   |
| **45 min** | **Chapter 4** | **Widgets** - Render charts, bar charts, sparklines, table |
| **20 min** | **Chapter 5** | **Interactivity** - Handle inputs, scroll table, search    |
| **-**      | 🧀            | **Q&A / Extras**                                           |

<!-- end_slide -->

# Chapter 1 - Setup 🧀 (10 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Install Rust using `rustup.rs` or any other way

• Install `cargo-generate`

• Generate a new Ratatui project

```bash
$ cargo generate ratatui/templates
```

<!-- column: 1 -->

## Bonus track

• Change the text

• Change the colors

<!-- end_slide -->

# Chapter 2 - Manage state 🧀 (15 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Add `sysinfo` dependency

• Store `System` struct

• Refresh the data every 16ms

<!-- column: 1 -->

## Bonus track

• Create `App::refresh` method

<!-- end_slide -->

# Chapter 3 - Layout & styling 🧀 (30 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Lay out the main application blocks

• CPU, Disks, Memory, Network, Processes

<!-- column: 1 -->

## Bonus track

• Add a one line header to the top with your app name

• Refactor rendering each block into its own function

• Render pretty borders and header

<!-- end_slide -->

# Chapter 4 - Widgets 🧀 (45 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Render CPU via `Chart` widget

• Customize the `Chart` widget

• Render processes via `Table` widget

<!-- column: 1 -->

## Bonus track

• Render memory (`Chart`)

• Render disks (`BarChart`)

• Render network (`Sparkline`)

<!-- end_slide -->

# Chapter 5 - Interactivity 🧀 (20 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Scroll the `Table` when J/K pressed

• Add text input via `tui-input`

<!-- column: 1 -->

## Bonus track

• Search processes

• Kill the selected process

<!-- end_slide -->

# Show and tell!

![](assets/rat-cheese.gif)
