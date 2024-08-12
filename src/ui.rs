use crate::notepad_app::NotepadApp;
use std::path::PathBuf;
use eframe::egui::{self, menu, Align, Layout, ScrollArea, RichText};

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create a top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New File").clicked() {
                        self.text.clear();
                        self.file_path = None;
                        self.save_history();
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                self.text = content;
                                self.file_path = Some(path);
                                self.save_history();
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        if let Some(path) = &self.file_path {
                            if let Err(err) = std::fs::write(path, &self.text) {
                                println!("Failed to save file: {}", err);
                            }
                        } else if let Some(path) = rfd::FileDialog::new().save_file() {
                            if let Err(err) = std::fs::write(&path, &self.text) {
                                println!("Failed to save file: {}", err);
                            }
                            self.file_path = Some(path);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save As").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            if let Err(err) = std::fs::write(&path, &self.text) {
                                println!("Failed to save file: {}", err);
                            }
                            self.file_path = Some(path);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        if self.current_step > 0 {
                            self.current_step -= 1;
                            self.text = self.history[self.current_step].clone();
                        }
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        if self.current_step < self.history.len() - 1 {
                            self.current_step += 1;
                            self.text = self.history[self.current_step].clone();
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button(if self.sidebar_visible { "Hide Sidebar" } else { "Show Sidebar" }).clicked() {
                        self.sidebar_visible = !self.sidebar_visible;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button("Open Settings").clicked() {
                        self.settings_open = true;  // Trigger the settings window
                        ui.close_menu();
                    }
                });

                ui.menu_button("Folder", |ui| {
                    if ui.button("Open Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.folder_path = path.clone();
                            self.refresh_folder_contents();
                        }
                        ui.close_menu();
                    }
                });
            });
        });

        // Sidebar for folder browsing
        let mut navigate_to_folder: Option<PathBuf> = None;
        let mut open_file: Option<(PathBuf, String)> = None;

        if self.sidebar_visible {
            egui::SidePanel::left("side_panel")
                .min_width(200.0)
                .show(ctx, |ui| {
                    ui.heading("Folder Explorer");
                    ui.label(format!("Opened Folder: {:?}", self.folder_path));

                    ui.separator();

                    // Add navigation to parent directory
                    if self.folder_path.parent().is_some() {
                        if ui.button(".. (Go to Parent Directory)").clicked() {
                            navigate_to_folder = Some(self.folder_path.parent().unwrap().to_path_buf());
                        }
                    }

                    ScrollArea::vertical().show(ui, |ui| {
                        for entry in &self.folder_contents {
                            let entry_name = entry.file_name().unwrap().to_str().unwrap();
                            let entry_icon = if entry.is_dir() {
                                "📁"
                            } else {
                                "📄"
                            };

                            let colored_entry_name = if entry.is_dir() {
                                RichText::new(format!("{} {}", entry_icon, entry_name))
                                    .color(egui::Color32::from_rgb(0, 150, 200))
                            } else {
                                RichText::new(format!("{} {}", entry_icon, entry_name))
                                    .color(egui::Color32::from_rgb(200, 200, 200))
                            };

                            if entry.is_dir() {
                                if ui.button(colored_entry_name).clicked() {
                                    navigate_to_folder = Some(entry.clone());
                                }
                            } else {
                                if ui.button(colored_entry_name).clicked() {
                                    if let Ok(content) = std::fs::read_to_string(entry) {
                                        open_file = Some((entry.clone(), content));
                                    }
                                }
                            }
                        }
                    });
                });
        }

        // Handle navigation outside of the UI update context
        if let Some(folder) = navigate_to_folder {
            self.folder_path = folder;
            self.refresh_folder_contents();
        }

        // Handle file open outside of the UI update context
        if let Some((path, content)) = open_file {
            self.text = content;
            self.file_path = Some(path);
            self.save_history();
        }

        // Create the main body
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.heading("TK Notepad");
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.text)
                            .desired_width(f32::INFINITY)
                            .lock_focus(true)
                            .font(egui::TextStyle::Monospace),
                    );

                    if response.changed() {
                        self.save_history();
                    }
                });
            });
        });

        // Display settings window if open
        if self.settings_open {
            let mut open = self.settings_open;
            egui::Window::new("Settings")
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.checkbox(&mut self.show_hidden_files, "Show Hidden Files");
                    if ui.button("Toggle Dark/Light Mode").clicked() {
                        self.dark_mode = !self.dark_mode;
                        self.apply_theme(ctx);
                    }
                    if ui.button("Close").clicked() {
                        self.settings_open = false;
                    }
                });
            self.settings_open = open;
        }
    }
}
