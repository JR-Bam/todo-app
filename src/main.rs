use eframe::egui::{self, CentralPanel, FontFamily, FontId, Layout, SidePanel, TextStyle, TopBottomPanel, ViewportBuilder};
use todo_func::{json_parser, TodoApp};

mod todo_func;

const PADDING: f32 = 5.0;

impl TodoApp {

    // * All UI declarations here
    fn render(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame){
        if self.show_sidepanel {
            SidePanel::left("pages_list")
                .resizable(false)
                .show(ctx, 
            |ui|{
                ui.add_space(PADDING);
                ui.vertical_centered_justified(|ui|{
                    ui.heading("List of Pages");
                });
                ui.separator();
                ui.monospace("WIP: You can put different pages here that each houses a set of notes");
            });
        }

        CentralPanel::default().show(ctx, |_|{
            self.render_header(ctx);
        });
    }
    fn render_header(&mut self, ctx: &eframe::egui::Context) {
        TopBottomPanel::top("header").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {
                // * Sidebar button container
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui|{

                    if ui.button("☰")
                    .on_hover_text_at_pointer("Menu")
                    .clicked() {
                        self.show_sidepanel = !self.show_sidepanel;
                    }

                    ui.add_space(PADDING);

                    if ui.button("⚙")
                    .on_hover_text_at_pointer("Settings")
                    .clicked() {
                        // TODO: Settings Functionality
                    }
                    
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui|{
                    if ui.button("➕ Add Note")
                    .clicked() {
                        // TODO: Add Note Functionality 
                    }
                });
                
            });

        });
    }

    // * Setup stuff here like fonts, etc.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        Self {
            state: json_parser::read_state_from_file().unwrap_or_default(),
            show_sidepanel: false
        }
    }
}

fn configure_fonts(ctx: &egui::Context){
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(20.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();

    ctx.set_style(style);
}

fn main() -> eframe::Result {
    let default_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800.0, 500.0]),
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
