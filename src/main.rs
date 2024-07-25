use eframe::egui::{self, Button, CentralPanel, Layout, RichText, ScrollArea, SidePanel, TextEdit, TopBottomPanel, Ui, Vec2, ViewportBuilder};
use todo_func::{Content, TodoApp};

mod todo_func;
mod json_parser;

const PADDING: f32 = 5.0;
const HEADER_TO_BODY_PADDING: f32 = 14.0;
// * The body's hitbox has a possibility to overlap the header's, resulting in weird focusing behaviors. This is a remedy.
const NOTE_PADDING: f32 = 10.0;

const TEMP_INPUT_ID_NAME: &str = "temp_input";
const TEMP_INPUT_WARNING_ID_NAME: &str = "notes_warning_message";
const TEMP_PAGE_INPUT_ID_NAME: &str = "temp_page_input";
const TEMP_PAGE_INPUT_WARNING_ID_NAME: &str = "pages_warning_message";

impl TodoApp {

    // * All UI declarations here
    fn render(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame){
        if self.show_sidepanel {
            self.render_side_panel(ctx)
        }

        CentralPanel::default().show(ctx, |ui|{
            self.render_header(ctx);
            ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui| {

                ui.add_space(HEADER_TO_BODY_PADDING);
                if self.show_addpanel{
                    self.render_add_panel(ui, ctx);
                }
                self.render_notes(ui);
                
            });
        });
    }

    fn render_side_panel(&mut self, ctx: &eframe::egui::Context){
        SidePanel::left("pages_list")
            .resizable(false)
            .show(ctx, 
        |ui|{
            ui.add_space(PADDING);
            ui.vertical_centered_justified(|ui|{
                ui.heading("Your Pages");
            });
            ui.separator();
            let add_button = ui.add_sized(Vec2::new(ui.available_width(), 16.), 
                Button::new("📝 New Page"));
            
            if add_button.clicked() {
                self.show_sideaddpagepanel = !self.show_sideaddpagepanel;
                if self.show_sideaddpagepanel {
                    TodoApp::write_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME, &String::new());
                }
            }

            if self.show_sideaddpagepanel {
                let mut pending_string = TodoApp::read_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME).unwrap_or_default();
                let mut string_entered = false;

                ui.vertical_centered_justified(|ui|{
                    ui.heading("⬇⬇⬇");
                });

                let response = ui.add_sized(
                    Vec2::new(ui.available_width(), 14.), 
                    TextEdit::singleline(&mut pending_string).hint_text("Enter name of page"));
                
                if response.lost_focus() && TodoApp::enter_key_pressed(ui) {
                    string_entered = true;
                }

                TodoApp::write_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME, &pending_string);

                if string_entered {
                    if pending_string.is_empty() || self.state_list.list.contains_key(&pending_string){
                        TodoApp::write_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME, &true);
                    } else {
                        self.state_list.list.insert(pending_string, String::default());
                        self.show_sideaddpagepanel = false;
                        TodoApp::write_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME, &false);
                    }
                }

                let show_error = TodoApp::read_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME).unwrap_or_default();

                if show_error {
                    ui.vertical_centered_justified(|ui|{
                        ui.label("⚠ Page title empty or already exists. ⚠");
                    });
                }
            }
            
            // Separator using label
            ui.add_space(PADDING);
            ui.vertical_centered_justified(|ui|{
                ui.monospace("...");
            });
            ui.add_space(PADDING);

            for page_title in self.state_list.list.keys() {
                let mut title = page_title.clone();
                if self.is_current_page(&page_title) {
                    title = format!("➡{}", title);
                }

                let response = ui.vertical_centered(|ui| {
                    ui.add_sized(Vec2::new(ui.available_width() - 10., 16.), 
                        Button::new(title).wrap_mode(egui::TextWrapMode::Truncate))
                });

                if response.inner.clicked() {
                    self.state_list.current_app_state = page_title.clone();
                }
                
            }

            self.show_updated_state(); // Maybe this can be optimized by calling only if there's a click
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
                    let add_button = ui.button("➕ Add Note");
                    if add_button.clicked() {
                        self.show_addpanel = !self.show_addpanel;
                        if self.show_addpanel {
                            TodoApp::write_temp_mem(ctx, TEMP_INPUT_ID_NAME, &String::new());
                        }
                    }
                });
                
            });

        });
    }

    fn display_empty_content_prompt(&self, ui: &mut Ui, to_print: &str){
        ui.centered_and_justified(|ui|{

            ui.heading(to_print).on_hover_cursor(eframe::egui::CursorIcon::Default);

        });
    }

    pub fn render_notes(&mut self, ui: &mut Ui){

        if self.no_page_selected() {
            self.display_empty_content_prompt(ui, "Page not selected. Press ☰ to select/add a page.");
            return;
        }

        if self.state.list.is_empty() {
            self.display_empty_content_prompt(ui, "🍃 Page is empty.");
            return;
        }

        let mut content_to_delete = Vec::<usize>::new();

        for (index, content) in self.state.list.iter_mut().enumerate() {
            ui.add_space(NOTE_PADDING);
            
            ui.horizontal(|ui|{
                // * Content
                ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui|{
                    ui.add_space(2.);
                    ui.checkbox(&mut content.is_checked, String::new());

                    if content.is_checked {
                        ui.label(RichText::new(&content.text).strikethrough());
                    } else {
                        ui.label(&content.text);
                    }
                });

                ui.add_space(20.);

                // * Buttons
                ui.with_layout(Layout::right_to_left(eframe::egui::Align::Min), |ui|{
                    if ui.button("❌").on_hover_text_at_pointer("Delete Note").clicked() {
                        content_to_delete.push(index);
                    }
                    ui.add_space(2.);
                });
            });


            ui.add_space(NOTE_PADDING);
            ui.separator();
        }

        self.delete_content(&mut content_to_delete);
        self.update_state();
        
    }

    pub fn render_add_panel(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context){
        let mut pending_string = TodoApp::read_temp_mem(ctx, TEMP_INPUT_ID_NAME).unwrap_or_default();
        let mut string_entered = false;

        ui.add_space(NOTE_PADDING);
        ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui| {
            ui.label("Enter content: ");
            let response = ui.add_sized(
                Vec2::new(ui.available_width(), 14.), 
                TextEdit::singleline(&mut pending_string));
            
            if response.lost_focus() && TodoApp::enter_key_pressed(ui) {
                string_entered = true;
            }
        });
        ui.add_space(NOTE_PADDING);

        TodoApp::write_temp_mem(ctx, TEMP_INPUT_ID_NAME, &pending_string);

        if string_entered {
            if pending_string.is_empty() {
                TodoApp::write_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME, &true);
            } else {
                self.state.list.push(Content {text: pending_string, is_checked: false });
                self.update_state();

                self.show_addpanel = false;
                TodoApp::write_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME, &false);
            }
        }

        let show_error = TodoApp::read_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME).unwrap_or_default();

        if show_error {
            ui.vertical_centered(|ui|{
                ui.label("⚠ Invalid. Content is empty or already exists within this page. ⚠").highlight();
                ui.add_space(PADDING); 
            });

        }

        ui.separator();
    }
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
