#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::{egui::{self, CentralPanel, ScrollArea, ViewportBuilder}, Theme};
use todo_func::TodoApp;

mod todo_func;
mod json_parser;

mod ui {
    pub mod center_panel;
    pub mod side_panel;
    pub mod top_panel;
    pub mod settings;
    pub mod popups;
}

const HEADER_TO_BODY_PADDING: f32 = 14.0;

impl TodoApp {

    // * All UI declarations here
    fn render(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame){
        if self.panel_manager.side_panel_visible {
            self.render_side_panel(ctx);
        }

        CentralPanel::default().show(ctx, |ui|{
            self.render_header(ctx);
            ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui| {

                ui.add_space(HEADER_TO_BODY_PADDING);
                if self.panel_manager.add_panel_visible{
                    self.render_add_panel(ui, ctx);
                }
                self.render_notes(ui);
                
            });
        });
    }
}

fn main() -> eframe::Result {
    let theme = if json_parser::read_theme().unwrap_or_default().is_dark_mode {Theme::Dark} else {Theme::Light};

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
        Box::new(|cc|{
            Ok(Box::new(TodoApp::new(cc)))
        })
    )
}
