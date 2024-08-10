use eframe::egui::{self, Layout, RichText, Window};
use crate::todo_func::TodoApp;

const PADDING: f32 = 5.0;

impl TodoApp {
    pub fn render_settings(&mut self, ctx: &eframe::egui::Context) {

        Window::new("Settings").open(&mut self.panel_manager.settings_visible).fade_in(true).fade_out(true).min_width(200.)
        .show(ctx, |ui|{
            ui.add_space(PADDING);
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui|{
                ui.add_space(PADDING);
                ui.vertical(|ui|{
                    ui.label("Theme: ");
                    ui.label("Clear Data: ");
                });
                ui.vertical_centered_justified(|ui| {
                    let theme_btn = ui.button( if self.dark_mode.is_dark_mode {"🌙 Dark"} else {"🌞 Light"});
                    let reset_btn = ui.button("🔁Reset");
    
                    if theme_btn.clicked() {
                        self.dark_mode.is_dark_mode = !self.dark_mode.is_dark_mode;
                    }

                    if reset_btn.clicked() {
                        self.panel_manager.reset_popup_visible = true;
                    }
    
                });
            }); 

            ui.add_space(30.);
            ui.separator();
            ui.vertical_centered(|ui|{
                ui.small("Made with Eframe/Egui in Rust!");
                ui.hyperlink_to(RichText::new("Visit the Source Code!").small(), "https://github.com/JR-Bam/todo-app");
            });
            
        });

        self.update_theme(ctx);
        
    }
}