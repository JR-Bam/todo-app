#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::{egui::ViewportBuilder, Theme};
use todo_func::TodoApp;

mod json_parser;
mod todo_func;
mod ui;

fn main() -> eframe::Result {
    let theme = if json_parser::read_theme().unwrap_or_default().is_dark_mode {
        Theme::Dark
    } else {
        Theme::Light
    };

    let default_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800.0, 500.0]),
        centered: true,
        follow_system_theme: false,
        default_theme: theme,
        ..Default::default()
    };

    eframe::run_native(
        "TODO App",
        default_options,
        Box::new(|cc| Ok(Box::new(TodoApp::new(cc)))),
    )
}
