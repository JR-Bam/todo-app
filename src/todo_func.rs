use std::collections::HashMap;

use eframe::{egui::{self, FontFamily, FontId, Id, Key, TextStyle, Ui, Visuals}, App};
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
    pub panel_manager: PanelManager,
    pub dark_mode: Theme
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct PanelManager {
    pub side_panel_visible: bool,
    pub add_panel_visible: bool,
    pub add_page_panel_visible: bool,
    pub settings_visible: bool,
    pub reset_popup_visible: bool,
    pub delete_page_popup_visible: bool,
}

impl PanelManager {
    pub fn show_side_panel(&mut self, visible: bool) {
        self.side_panel_visible = visible;
    }

    pub fn show_add_panel(&mut self, visible: bool) {
        self.add_panel_visible = visible;
    }

    pub fn show_settings(&mut self, visible: bool) {
        self.settings_visible = visible;
    }

    pub fn show_reset_popup(&mut self, visible: bool) {
        self.reset_popup_visible = visible;
    }

    pub fn show_delete_page_popup(&mut self, visible: bool) {
        self.delete_page_popup_visible = visible;
    }

    pub fn show_add_page_panel(&mut self, visible: bool) {
        self.add_page_panel_visible = visible;
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Theme {
    pub is_dark_mode: bool
}

impl App for TodoApp {

    fn raw_input_hook(&mut self, _ctx: &eframe::egui::Context, _raw_input: &mut eframe::egui::RawInput) {} // TODO
    
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.render(ctx, frame);
        self.render_popups(ctx);
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Err(e) = json_parser::save_state_list(&self.state_list, storage){
            eprintln!("Error while saving state_listL {e}");
        }

        if let Err(e) = json_parser::save_theme(&self.dark_mode) {
            eprintln!("Failed to save theme: {e}");
        } 
    }


}

impl TodoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);

        let mut state_list = json_parser::read_state_list(cc).unwrap_or_default();
        let state = AppState::default();
        state_list.current_app_state = String::new();

        let dark_mode = json_parser::read_theme().unwrap_or(Theme {is_dark_mode: true});

        Self { state, state_list, dark_mode, ..Default::default() }
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

    pub fn write_temp_mem(ctx: &eframe::egui::Context, id: &'static str, to_write: &str) {
        ctx.memory_mut(|mem| {
            mem.data.insert_temp(Id::new(id), String::from(to_write));
        });
    }

    pub fn read_persist_state(ctx: &eframe::egui::Context, id: &'static str) -> Option<bool> {
        ctx.memory(|mem| {
            mem.data.get_temp(Id::new(id))
        })
    }

    pub fn write_persist_state(ctx: &eframe::egui::Context, id: &'static str, to_write: bool) {
        ctx.memory_mut(|mem| {
            mem.data.insert_temp(Id::new(id), to_write);
        });
    }

    pub fn delete_content(&mut self, arr: &mut Vec<usize>){
        if arr.len() > 1 {
            arr.sort_unstable();
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

    pub fn no_page_selected(&self) -> bool {
        self.state_list.current_app_state.is_empty()
    }

    pub fn enter_key_pressed(ui: &Ui) -> bool {
        ui.input(|i| i.key_pressed(Key::Enter))
    }

    pub fn is_current_page(&self, title: &str) -> bool {
        title == self.state_list.current_app_state.as_str()
    }

    pub fn show_updated_state(&mut self) {
        self.state = json_parser::json_string_to_state(
            self.state_list.list.get(&self.state_list.current_app_state))
            .unwrap_or_default();
    }

    pub fn update_theme(&self, ctx: &eframe::egui::Context){
        if self.dark_mode.is_dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }
    }

    pub fn delete_data(&mut self){
        self.state = AppState::default();
        self.state_list = StateList::default();
        self.state_list.current_app_state = String::new();
    }

    pub fn delete_page(&mut self){
        self.state = AppState::default();
        self.state_list.list.remove(&self.state_list.current_app_state);
        self.state_list.current_app_state = String::new();
    }
    
}
