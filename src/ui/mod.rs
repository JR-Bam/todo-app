use eframe::egui::{
    self, Button, CentralPanel, Frame, Label, Layout, RichText, ScrollArea, SidePanel, TextEdit,
    TopBottomPanel, Ui, Vec2, Window,
};

use crate::todo_func::{Content, TodoApp};

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
    pub fn show(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.show_sidepanel {
            self.show_side_panel(ctx);
        }

        CentralPanel::default().show(ctx, |ui| {
            self.show_header(ctx);
            ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    ui.add_space(HEADER_TO_BODY_PADDING);
                    if self.show_addpanel {
                        self.show_add_panel(ui, ctx);
                    }
                    self.show_notes(ui);
                });
        });
    }

    #[allow(clippy::too_many_lines)]
    fn show_side_panel(&mut self, ctx: &eframe::egui::Context) {
        let window_width = ctx.available_rect().width();
        // Only allow users to drag the side panel within 20% - 60% of the width of the entire window
        let min_width = window_width * 0.2;
        let max_width = window_width * 0.6;

        SidePanel::left("pages_list")
            .resizable(true)
            .width_range(min_width..=max_width)
            .show(ctx, |ui| {
                let mut to_delete_page = false;

                TopBottomPanel::bottom("footer")
                    .frame(Frame::default().outer_margin(10.))
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.monospace("...");

                            ui.add_space(NOTE_PADDING);
                            if self.no_page_selected() {
                                ui.monospace("No Page Selected");
                            } else if ui.button("🗑 Delete Page").clicked() {
                                to_delete_page = true;
                            }
                            ui.add_space(PADDING);
                        });
                    });

                ui.add_space(PADDING);
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Your Pages");
                });
                ui.separator();
                let add_button = ui.add_sized(
                    Vec2::new(ui.available_width(), 16.),
                    Button::new("📝 New Page"),
                );

                if add_button.clicked() {
                    self.show_sideaddpagepanel = !self.show_sideaddpagepanel;
                    if self.show_sideaddpagepanel {
                        Self::write_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME, String::new());
                    }
                }

                if self.show_sideaddpagepanel {
                    let mut pending_string =
                        Self::read_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME).unwrap_or_default();
                    let mut string_entered = false;

                    ui.vertical_centered_justified(|ui| {
                        ui.heading("⬇⬇⬇");
                    });

                    let response = ui.add_sized(
                        Vec2::new(ui.available_width(), 14.),
                        TextEdit::singleline(&mut pending_string).hint_text("Enter name of page"),
                    );

                    if response.lost_focus() && Self::enter_key_pressed(ui) {
                        string_entered = true;
                    }

                    Self::write_temp_mem(ctx, TEMP_PAGE_INPUT_ID_NAME, pending_string.clone());

                    if string_entered {
                        if pending_string.is_empty()
                            || self.state_list.list.contains_key(&pending_string)
                        {
                            Self::write_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME, true);
                        } else {
                            self.state_list
                                .list
                                .insert(pending_string, String::default());
                            self.show_sideaddpagepanel = false;
                            Self::write_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME, false);
                        }
                    }

                    let show_error = Self::read_persist_state(ctx, TEMP_PAGE_INPUT_WARNING_ID_NAME)
                        .unwrap_or_default();

                    if show_error {
                        ui.vertical_centered_justified(|ui| {
                            ui.label("⚠ Page title empty or already exists. ⚠");
                        });
                    }
                }

                // Separator using label
                ui.add_space(PADDING);
                ui.vertical_centered_justified(|ui| {
                    ui.monospace("...");
                });
                ui.add_space(PADDING);

                let mut page_title_clicked = false;

                ScrollArea::vertical()
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        for page_title in self.state_list.list.keys() {
                            let mut title = page_title.clone();
                            if self.is_current_page(page_title) {
                                title = format!("➡{title}");
                            }

                            ui.vertical_centered(|ui| {
                                let page_btn = ui.add_sized(
                                    Vec2::new(ui.available_width() - 20., 18.),
                                    Button::new(title).wrap_mode(egui::TextWrapMode::Truncate),
                                );

                                if page_btn.clicked() {
                                    self.state_list.current_app_state.clone_from(page_title);
                                    page_title_clicked = true;
                                }
                            });
                        }
                    });

                if page_title_clicked {
                    self.show_updated_state();
                }

                if to_delete_page {
                    self.show_delete_page_popup = true;
                }
            });
    }

    fn show_header(&mut self, ctx: &eframe::egui::Context) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // * Sidebar button container
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui.button("☰").on_hover_text_at_pointer("Menu").clicked() {
                        self.show_sidepanel = !self.show_sidepanel;
                    }

                    ui.add_space(PADDING);

                    if ui
                        .button("⚙")
                        .on_hover_text_at_pointer("Settings")
                        .clicked()
                    {
                        self.show_settings = true;
                    }
                });

                if self.show_settings {
                    self.show_settings(ctx);
                }

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    let add_button = ui.button("➕ New Note");
                    if add_button.clicked() && !self.no_page_selected() {
                        self.show_addpanel = !self.show_addpanel;
                        if self.show_addpanel {
                            Self::write_temp_mem(ctx, TEMP_INPUT_ID_NAME, String::new());
                        }
                    }
                });
            });
        });
    }

    fn show_settings(&mut self, ctx: &eframe::egui::Context) {
        Window::new("Settings")
            .open(&mut self.show_settings)
            .fade_in(true)
            .fade_out(true)
            .min_width(200.)
            .show(ctx, |ui| {
                ui.add_space(PADDING);
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.add_space(PADDING);
                    ui.vertical(|ui| {
                        ui.label("Theme: ");
                        ui.label("Clear Data: ");
                    });
                    ui.vertical_centered_justified(|ui| {
                        let theme_btn = ui.button(if self.dark_mode.is_dark_mode {
                            "🌙 Dark"
                        } else {
                            "🌞 Light"
                        });
                        let reset_btn = ui.button("🔁Reset");

                        if theme_btn.clicked() {
                            self.dark_mode.is_dark_mode = !self.dark_mode.is_dark_mode;
                        }

                        if reset_btn.clicked() {
                            self.show_reset_popup = true;
                        }
                    });
                });

                ui.add_space(30.);
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.small("Made with Eframe/Egui in Rust!");
                    ui.hyperlink_to(
                        RichText::new("Visit the Source Code!").small(),
                        "https://github.com/JR-Bam/todo-app",
                    );
                });
            });

        self.update_theme(ctx);
    }

    fn display_empty_content_prompt(ui: &mut Ui, to_print: &str) {
        ui.centered_and_justified(|ui| {
            ui.heading(to_print)
                .on_hover_cursor(eframe::egui::CursorIcon::Default);
        });
    }

    fn show_notes(&mut self, ui: &mut Ui) {
        if self.no_page_selected() {
            Self::display_empty_content_prompt(
                ui,
                "No page selected. Press ☰ to select/add a page.",
            );
            return;
        }

        if self.state.list.is_empty() {
            Self::display_empty_content_prompt(ui, "🍃 Page is empty.");
            return;
        }

        let mut content_to_delete = Vec::<usize>::new();

        for (index, content) in self.state.list.iter_mut().enumerate() {
            ui.add_space(NOTE_PADDING);

            ui.horizontal(|ui| {
                // * Content
                ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui| {
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
                ui.with_layout(Layout::right_to_left(eframe::egui::Align::Min), |ui| {
                    if ui
                        .button("❌")
                        .on_hover_text_at_pointer("Delete Note")
                        .clicked()
                    {
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

    fn show_add_panel(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context) {
        let mut pending_string = Self::read_temp_mem(ctx, TEMP_INPUT_ID_NAME).unwrap_or_default();
        let mut string_entered = false;

        ui.add_space(NOTE_PADDING);
        ui.with_layout(Layout::left_to_right(eframe::egui::Align::Min), |ui| {
            ui.label("Enter content: ");
            let response = ui.add_sized(
                Vec2::new(ui.available_width(), 14.),
                TextEdit::singleline(&mut pending_string),
            );

            if response.lost_focus() && Self::enter_key_pressed(ui) {
                string_entered = true;
            }
        });
        ui.add_space(NOTE_PADDING);

        Self::write_temp_mem(ctx, TEMP_INPUT_ID_NAME, pending_string.clone());

        if string_entered {
            if pending_string.is_empty() {
                Self::write_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME, true);
            } else {
                self.state.list.push(Content {
                    text: pending_string,
                    is_checked: false,
                });
                self.update_state();

                self.show_addpanel = false;
                Self::write_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME, false);
            }
        }

        let show_error =
            Self::read_persist_state(ctx, TEMP_INPUT_WARNING_ID_NAME).unwrap_or_default();

        if show_error {
            ui.vertical_centered(|ui| {
                ui.label("⚠ Invalid. Content is empty or already exists within this page. ⚠")
                    .highlight();
                ui.add_space(PADDING);
            });
        }

        ui.separator();
    }

    pub fn show_popups(&mut self, ctx: &eframe::egui::Context) {
        if self.show_reset_popup {
            let mut temp_show_popup = self.show_reset_popup;
            Window::new("Confirm Clearing of Data.").title_bar(false).open(&mut temp_show_popup).resizable(false).movable(true).show(ctx, |ui|{
                ui.monospace("Clearing data includes all notes and pages and cannot be reversed. Are you sure you want to delete your data?");
                ui.add_space(PADDING);
                ui.with_layout( Layout::left_to_right(egui::Align::Min),|ui|{
                    let yes = ui.button("Yes");
                    let no = ui.button("No");

                    if no.clicked() {
                        self.show_reset_popup = false;
                    }

                    if yes.clicked() {
                        self.delete_data();
                        self.show_reset_popup = false;
                    }
                });

            });
        }

        if self.show_delete_page_popup {
            let mut temp_show_popup = self.show_delete_page_popup;
            Window::new("Confirm Deleting Page.")
                .title_bar(false)
                .open(&mut temp_show_popup)
                .resizable(false)
                .movable(true)
                .show(ctx, |ui| {
                    ui.monospace("You are attempting to delete the page entitled:");
                    ui.add_space(PADDING);
                    ui.vertical_centered(|ui| {
                        ui.monospace(
                            RichText::new(self.state_list.current_app_state.to_string()).strong(),
                        );
                    });
                    ui.add_space(PADDING);
                    ui.monospace(
                        "Doing so will also delete every note within it. Are you sure of this?",
                    );
                    ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                        let yes = ui.button("Yes");
                        let no = ui.button("No");

                        if no.clicked() {
                            self.show_delete_page_popup = false;
                        }

                        if yes.clicked() {
                            self.delete_page();
                            self.show_delete_page_popup = false;
                        }
                    });
                });
        }
    }
}
