use crate::todo_func::TodoApp;
use eframe::egui::{self, Button, Frame, ScrollArea, SidePanel, TextEdit, TopBottomPanel, Vec2};

const PADDING: f32 = 5.0;
const NOTE_PADDING: f32 = 10.0;
const TEMP_PAGE_INPUT_ID_NAME: &str = "temp_page_input";
const TEMP_PAGE_INPUT_WARNING_ID_NAME: &str = "pages_warning_message";

impl TodoApp {
    pub fn render_side_panel(&mut self, ctx: &eframe::egui::Context){
        let window_width = ctx.available_rect().width();
        // Only allow users to drag the side panel within 20% - 60% of the width of the entire window
        let min_width = window_width * 0.2;
        let max_width = window_width * 0.6;

        SidePanel::left("pages_list")
            .resizable(true)
            .width_range(min_width..=max_width)
            .show(ctx, 
        |ui|{
            let mut to_delete_page = false;

            TopBottomPanel::bottom("footer")
            .frame(Frame::default().outer_margin(10.))
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.vertical_centered_justified(|ui|{

                    ui.monospace("...");
                    
                    ui.add_space(NOTE_PADDING);
                    if self.no_page_selected() {
                        ui.monospace("No Page Selected");
                    } else {
                        let delete_btn = ui.button("🗑 Delete Page");
                        if delete_btn.clicked() {
                            to_delete_page = true;
                        }
                    }
                    ui.add_space(PADDING);
                });
            });


            ui.add_space(PADDING);
            ui.vertical_centered_justified(|ui|{
                ui.heading("Your Pages");
            });
            ui.separator();
            let add_button = ui.add_sized(Vec2::new(ui.available_width(), 16.), 
                Button::new("📝 New Page"));
            
            if add_button.clicked() {
                self.panel_manager.add_page_panel_visible = !self.panel_manager.add_page_panel_visible;
                if self.panel_manager.add_page_panel_visible {
                    Self::write_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME, "");
                }
            }

            if self.panel_manager.add_page_panel_visible {
                let mut pending_string = Self::read_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME).unwrap_or_default();
                let mut string_entered = false;

                ui.vertical_centered_justified(|ui|{
                    ui.heading("⬇⬇⬇");
                });

                let response = ui.add_sized(
                    Vec2::new(ui.available_width(), 14.), 
                    TextEdit::singleline(&mut pending_string).hint_text("Enter name of page"));
                
                if response.lost_focus() && Self::enter_key_pressed(ui) {
                    string_entered = true;
                }

                Self::write_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME, &pending_string);

                if string_entered {
                    if pending_string.is_empty() || self.state_list.list.contains_key(&pending_string){
                        Self::write_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME, true);
                    } else {
                        self.state_list.list.insert(pending_string, String::default());
                        self.panel_manager.show_add_page_panel(false);
                        Self::write_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME, false);
                    }
                }

                let show_error = Self::read_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME).unwrap_or_default();

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


            let mut page_title_clicked = false;

            ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui|{
                for page_title in self.state_list.list.keys() {
                    let mut title = page_title.clone();
                    if self.is_current_page(page_title) {
                        title = format!("➡{title}");
                    }
    
                    ui.vertical_centered(|ui|{
                        let page_btn = ui.add_sized(Vec2::new(ui.available_width() - 20., 18.), 
                    Button::new(title).wrap_mode(egui::TextWrapMode::Truncate));
    
                        if page_btn.clicked() {
                            self.state_list.current_app_state = page_title.to_string();
                            page_title_clicked = true;
                        }
                    });
                    
                }
            });
            

            if page_title_clicked {
                self.show_updated_state();
            }

            if to_delete_page {
                self.panel_manager.show_delete_page_popup(true);
            }
        });



    }
}