use std::collections::HashMap;

use eframe::App;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct AppState {
    pub list: HashMap<String, bool>
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
    pub show_sidepanel: bool
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
}