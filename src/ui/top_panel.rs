use eframe::egui::{self, Layout, TextEdit, TopBottomPanel, Ui, Vec2};

use crate::todo_func::{TodoApp, Content};

const PADDING: f32 = 5.0;
const NOTE_PADDING: f32 = 10.0;
const TEMP_INPUT_ID_NAME: &str = "temp_input";
const TEMP_INPUT_WARNING_ID_NAME: &str = "notes_warning_message";


impl TodoApp {
    pub fn render_header(&mut self, ctx: &eframe::egui::Context) {
        TopBottomPanel::top("header").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {
                // * Sidebar button container
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui|{

                    if ui.button("☰")
                    .on_hover_text_at_pointer("Menu")
                    .clicked() {
                        self.panel_manager.show_side_panel(!self.panel_manager.side_panel_visible);
                    }

                    ui.add_space(PADDING);

                    if ui.button("⚙")
                    .on_hover_text_at_pointer("Settings")
                    .clicked() {
                        self.panel_manager.show_settings(true);
                    }
                    
                });

                if self.panel_manager.settings_visible {
                    self.render_settings(ctx);
                }

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui|{
                    let add_button = ui.button("➕ New Note");
                    if add_button.clicked() && !self.no_page_selected(){
                        self.panel_manager.add_panel_visible = !self.panel_manager.add_panel_visible;
                        if self.panel_manager.add_panel_visible {
                            Self::write_temp_mem(ctx, TEMP_INPUT_ID_NAME, "");
                        }
                    }
                });
                
            });

        });
    }

    pub fn render_add_panel(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context){
        let mut pending_string = Self::read_temp_mem(ctx, TEMP_INPUT_ID_NAME).unwrap_or_default();
        let mut string_entered = false;

        ui.add_space(NOTE_PADDING);
        ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui| {
            ui.label("Enter content: ");
            let response = ui.add_sized(
                Vec2::new(ui.available_width(), 14.), 
                TextEdit::singleline(&mut pending_string));
            
            if response.lost_focus() && Self::enter_key_pressed(ui) {
                string_entered = true;
            }
        });
        ui.add_space(NOTE_PADDING);

        Self::write_temp_mem(ctx, TEMP_INPUT_ID_NAME, &pending_string);

        if string_entered {
            if pending_string.is_empty() {
                Self::write_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME, true);
            } else {
                self.state.list.push(Content {text: pending_string, is_checked: false });
                self.update_state();

                self.panel_manager.show_add_panel(false);
                Self::write_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME, false);
            }
        }

        let show_error = Self::read_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME).unwrap_or_default();

        if show_error {
            ui.vertical_centered(|ui|{
                ui.label("⚠ Invalid. Content is empty or already exists within this page. ⚠").highlight();
                ui.add_space(PADDING); 
            });

        }

        ui.separator();
    }
}