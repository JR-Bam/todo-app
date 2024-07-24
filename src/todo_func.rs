/*
  TODO: using the persistent storage of App, try to store a value of AppState.list depending on which page to use. 
 */
use std::collections::HashMap;

use eframe::{egui::{self, FontFamily, FontId, Id, Key, TextStyle, Ui}, App};
use serde::{Deserialize, Serialize};

use crate::json_parser;

#[derive(Serialize, Deserialize, Default)]
pub struct AppState {
    pub list: Vec<Content>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Content{
    pub text: String,
    pub is_checked: bool
}


#[derive(Serialize, Deserialize, Default)]
pub struct StateList {
    pub list: HashMap<String, String>,
    pub current_app_state: String
}

#[derive(Default)]
pub struct TodoApp{
    pub state: AppState,
    pub state_list: StateList,
    pub show_sidepanel: bool,
    pub show_addpanel: bool,
    pub show_sideaddpagepanel: bool
}

impl App for TodoApp {

    fn raw_input_hook(&mut self, _ctx: &eframe::egui::Context, _raw_input: &mut eframe::egui::RawInput) {} // TODO
    
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.render(ctx, frame);
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        if let Err(_) = json_parser::save_state_list(&self.state_list, _storage){
            println!("Error while saving state_list.");
        }
    }


}

impl TodoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);

        let state_list = json_parser::read_state_list(cc).unwrap_or_default();
        let mut state = AppState::default();

        if !state_list.list.is_empty() && !state_list.current_app_state.is_empty() {
            state = json_parser::json_string_to_state(&state_list.list.get(&state_list.current_app_state)).unwrap_or_default();
        }


        Self {
            state_list,
            state,
            show_sidepanel: false,
            show_addpanel: false,
            show_sideaddpagepanel: false
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



impl TodoApp {
    pub fn read_temp_mem(ctx: &eframe::egui::Context, id: &'static str) -> Option<String> {
        ctx.memory(|mem| {
            mem.data.get_temp(Id::new(id))
        })
    }

    pub fn write_temp_mem(ctx: &eframe::egui::Context, id: &'static str, to_write: &String) {
        ctx.memory_mut(|mem| {
            mem.data.insert_temp(Id::new(id), to_write.clone());
        });
    }

    pub fn read_persist_state(ctx: &eframe::egui::Context, id: &'static str) -> Option<bool> {
        ctx.memory(|mem| {
            mem.data.get_temp(Id::new(id))
        })
    }

    pub fn write_persist_state(ctx: &eframe::egui::Context, id: &'static str, to_write: &bool) {
        ctx.memory_mut(|mem| {
            mem.data.insert_temp(Id::new(id), to_write.clone());
        });
    }

    pub fn delete_content(&mut self, arr: &mut Vec<usize>){
        if arr.len() > 1 {
            arr.sort();
            arr.reverse();
        }

        for index in arr {
            self.state.list.remove(*index);
        }
    }

    pub fn update_state(&mut self) {
        let state_as_json = json_parser::state_to_json_string(&self.state);
        self.state_list.list.insert(self.state_list.current_app_state.clone(), state_as_json);
    }

    pub fn is_state_list_empty(&self) -> bool {
        self.state_list.list.len() == 0
    }

    pub fn enter_key_pressed(ui: &Ui) -> bool {
        ui.input(|i| i.key_pressed(Key::Enter))
    }
    
}
