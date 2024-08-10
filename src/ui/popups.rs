use eframe::egui::{self, Layout, RichText, Window};
use crate::todo_func::TodoApp;

const PADDING: f32 = 5.0;

impl TodoApp {
    pub fn render_popups(&mut self, ctx: &eframe::egui::Context){
        if self.panel_manager.reset_popup_visible {
            let mut temp_show_popup = self.panel_manager.reset_popup_visible;
            Window::new("Confirm Clearing of Data.").title_bar(false).open(&mut temp_show_popup).resizable(false).movable(true).show(ctx, |ui|{
                ui.monospace("Clearing data includes all notes and pages and cannot be reversed. Are you sure you want to delete your data?");
                ui.add_space(PADDING);
                ui.with_layout( Layout::left_to_right(egui::Align::Min),|ui|{
                    let yes = ui.button("Yes");
                    let no = ui.button("No");

                    if no.clicked() {
                        self.panel_manager.show_reset_popup(false);
                    }

                    if yes.clicked() {
                        self.delete_data();
                        self.panel_manager.show_reset_popup(false);
                    }
                });

            });
        }

        if self.panel_manager.delete_page_popup_visible {
            let mut temp_show_popup = self.panel_manager.delete_page_popup_visible;
            Window::new("Confirm Deleting Page.").title_bar(false).open(&mut temp_show_popup).resizable(false).movable(true).show(ctx, |ui|{
                ui.monospace("You are attempting to delete the page entitled:");
                ui.add_space(PADDING);
                ui.vertical_centered(|ui|{
                    ui.monospace(RichText::new(self.state_list.current_app_state.to_string()).strong());
                });
                ui.add_space(PADDING);
                ui.monospace("Doing so will also delete every note within it. Are you sure of this?");
                ui.with_layout( Layout::left_to_right(egui::Align::Min),|ui|{
                    let yes = ui.button("Yes");
                    let no = ui.button("No");

                    if no.clicked() {
                        self.panel_manager.show_delete_page_popup(false);
                    }

                    if yes.clicked() {
                        self.delete_page();
                        self.panel_manager.show_delete_page_popup(false);
                    }
                });

            });
        }
    }
}