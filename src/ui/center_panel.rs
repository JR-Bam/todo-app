use crate::todo_func::TodoApp;

use eframe::egui::{Label, Layout, RichText, Ui};

// * The body's hitbox has a possibility to overlap the header's, resulting in weird focusing behaviors. This is a remedy.
const NOTE_PADDING: f32 = 10.0;

impl TodoApp {
    pub fn display_empty_content_prompt(ui: &mut Ui, to_print: &str){
        ui.centered_and_justified(|ui|{

            ui.heading(to_print).on_hover_cursor(eframe::egui::CursorIcon::Default);

        });
    }

    pub fn render_notes(&mut self, ui: &mut Ui){

        if self.no_page_selected() {
            Self::display_empty_content_prompt(ui, "No page selected. Press ☰ to select/add a page.");
            return;
        }

        if self.state.list.is_empty() {
            Self::display_empty_content_prompt(ui, "🍃 Page is empty.");
            return;
        }

        let mut content_to_delete = Vec::<usize>::new();

        for (index, content) in self.state.list.iter_mut().enumerate() {
            ui.add_space(NOTE_PADDING);
            
            ui.horizontal(|ui|{
                // * Content
                ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui|{
                    ui.set_width(ui.available_width() * 0.9); // Takes up only 90% of the available width
                    ui.add_space(2.);
                    ui.checkbox(&mut content.is_checked, String::new());

                    if content.is_checked {
                        ui.add(Label::new(RichText::new(&content.text).strikethrough()).wrap());
                    } else {
                        ui.add(Label::new(&content.text).wrap());
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
}