/*
  TODO: using the persistent storage of App, try to store a value of AppState.list depending on which page to use. 
 */
use std::collections::HashMap;

use eframe::{egui::{Id, Key, Layout, RichText, TextEdit, Ui, Vec2}, App};
use serde::{Deserialize, Serialize};

use crate::{json_parser, PADDING};
const TEMP_INPUT_ID_NAME: &str = "temp_input";
const TEMP_INPUT_WARNING_ID_NAME: &str = "notes_warning_message";

// * The body's hitbox has a possibility to overlap the header's, resulting in weird focusing behaviors. This is a remedy.
const NOTE_PADDING: f32 = 10.0;

#[derive(Serialize, Deserialize, Default)]
pub struct AppState {
    pub list: HashMap<String, NoteFlags>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct NoteFlags {
    pub is_checked: bool
}


#[derive(Default)]
pub struct TodoApp{
    pub state: AppState,
    pub show_sidepanel: bool,
    pub show_addpanel: bool
}

impl App for TodoApp {

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        json_parser::save_state_to_file(&self.state)
            .expect("Failed to save state.");
    }

    fn raw_input_hook(&mut self, _ctx: &eframe::egui::Context, _raw_input: &mut eframe::egui::RawInput) {}
    
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.render(ctx, frame);
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }


}


impl TodoApp {
    pub fn render_notes(&mut self, ui: &mut Ui){

        if self.state.list.is_empty() {
            ui.centered_and_justified(|ui|{
                ui.heading("🍃 Page is empty.").on_hover_cursor(eframe::egui::CursorIcon::Default);
            });
        } else {

            let mut keys_to_delete = Vec::<String>::new();

            for (key, value) in &mut self.state.list {
                ui.add_space(NOTE_PADDING);
                
                ui.horizontal(|ui|{
                    // * Content
                    ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui|{
                        ui.add_space(2.);
                        ui.checkbox(&mut value.is_checked, String::new());

                        if value.is_checked {
                            ui.label(RichText::new(key).strikethrough());
                        } else {
                            ui.label(key);
                        }
                    });

                    ui.add_space(20.);

                    // * Buttons
                    ui.with_layout(Layout::right_to_left(eframe::egui::Align::Min), |ui|{
                        if ui.button("❌").on_hover_text_at_pointer("Delete Note").clicked() {
                            keys_to_delete.push(key.clone());
                        }
                        ui.add_space(2.);
                    });
                });

                ui.add_space(NOTE_PADDING);
                ui.separator();
            }

            for key in keys_to_delete {
                self.state.list.remove(&key);
            }
        }
    }

    pub fn render_add_panel(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context){
        let mut pending_string = String::new();

            // ! So uhm.. found out you can NOT declare ui elements inside memory_mut closures. App crashes a lot.
            ctx.memory_mut(|mem| {
                pending_string = mem.data.get_temp(Id::new(TEMP_INPUT_ID_NAME)).unwrap_or_default();
            });

            let mut string_entered = false;

            ui.add_space(NOTE_PADDING);
            ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui| {
                ui.label("Enter content: ");
                let response = ui.add_sized(
                    Vec2::new(ui.available_width(), 14.), 
                    TextEdit::singleline(&mut pending_string));
                
                if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                    string_entered = true;
                }
            });
            ui.add_space(NOTE_PADDING);

            ctx.memory_mut(|mem| {
                mem.data.insert_temp(Id::new(TEMP_INPUT_ID_NAME), pending_string.clone());
            });

            if string_entered {
                if pending_string.is_empty() || self.state.list.contains_key(&pending_string) {
                    ctx.memory_mut(|mem|{
                        mem.data.insert_temp(Id::new(TEMP_INPUT_WARNING_ID_NAME), true);
                    });
                } else {
                    self.state.list.insert(pending_string, NoteFlags { is_checked: false });
                    self.show_addpanel = false;
                    ctx.memory_mut(|mem|{
                        mem.data.insert_temp(Id::new(TEMP_INPUT_WARNING_ID_NAME), false);
                    });
                }
            }

            let mut show_error = false;
            ctx.memory(|mem|{
                if let Some(show_error_flag) = mem.data.get_temp::<bool>(Id::new(TEMP_INPUT_WARNING_ID_NAME)){
                    show_error = show_error_flag;
                } // TODO: Custom Error
            });

            if show_error {
                ui.vertical_centered(|ui|{
                    ui.label("⚠ Invalid. Content is empty or already exists within this page. ⚠").highlight();
                    ui.add_space(PADDING); 
                });

            }

            ui.separator();
    }
}
