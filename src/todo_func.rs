/*
  TODO: using the persistent storage of App, try to store a value of AppState.list depending on which page to use. 
 */
use std::collections::HashMap;

use eframe::{egui::{Id, Key, Layout, TextEdit, Ui, Vec2}, App};
use serde::{Deserialize, Serialize};

use crate::TEMP_INPUT_ID_NAME;

// * The body's hitbox has a possibility to overlap the header's, resulting in weird focusing behaviors. This is a remedy.
const HEADER_TO_BODY_PADDING: f32 = 14.0;
const NOTE_PADDING: f32 = 10.0;

#[derive(Serialize, Deserialize, Default)]
pub struct AppState {
    pub list: HashMap<String, NoteFlags>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct NoteFlags {
    pub is_checked: bool,
    pub is_editable: bool
}

pub mod json_parser {
    use std::{fs::File, io::{self, Read, Write}, path::Path};
    use super::AppState;

    const FILE_PATH: &str = "entries.json";

    pub fn read_state_from_file() -> io::Result<AppState> {
        if Path::new(FILE_PATH).exists() {
            let mut file = File::open(FILE_PATH)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            Ok(AppState::default())
        }
    }

    pub fn save_state_to_file(state: &AppState) -> io::Result<()> {
        let json = serde_json::to_string_pretty(state)?;
        let mut file = File::create(FILE_PATH)?;
        file.write_all(json.as_bytes())
    }
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
    pub fn render_notes(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context){
        ui.add_space(HEADER_TO_BODY_PADDING);

        if self.show_addpanel {
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
                    ui.label("⚠️ Invalid. Content is empty or already exists. ⚠️"); // TODO: Make this persist one way or another
                } else {
                    self.state.list.insert(pending_string, NoteFlags { is_checked: false, is_editable: false });
                    self.show_addpanel = false;
                }
            }
            ui.separator();
        }

        if self.state.list.is_empty() {
            ui.centered_and_justified(|ui|{
                ui.heading("🍃 Page is empty.").on_hover_cursor(eframe::egui::CursorIcon::Default);
            });
        } else {

            for (key, value) in &mut self.state.list {
                ui.add_space(NOTE_PADDING);
                ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui| {
                    ui.checkbox(&mut value.is_checked, key);
                });
                ui.add_space(NOTE_PADDING);
                ui.separator();
            }
        }
    }
}
