use std::fs;
use std::path::{PathBuf};
use dirs::home_dir;
use eframe::egui::{self, Color32, Context};

pub struct NotepadApp {
    pub text: String,
    pub file_path: Option<PathBuf>,
    pub history: Vec<String>,
    pub current_step: usize,
    pub folder_path: PathBuf,
    pub folder_contents: Vec<PathBuf>,
    pub sidebar_visible: bool,
    pub show_hidden_files: bool, // New field for hidden files
    pub settings_open: bool,     // New field to control settings window visibility
    pub dark_mode: bool,         // New field for theme toggle
}

impl Default for NotepadApp {
    fn default() -> Self {
        let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
        let mut app = Self {
            text: String::new(),
            file_path: None,
            history: vec![String::new()],
            current_step: 0,
            folder_path: home_dir.clone(),
            folder_contents: vec![],
            sidebar_visible: true,
            show_hidden_files: false, // Default to not showing hidden files
            settings_open: false,     // Settings window closed by default
            dark_mode: false,         // Start with light mode
        };
        app.refresh_folder_contents();  // Ensure folder contents are populated
        app
    }
}

impl NotepadApp {
    pub fn save_history(&mut self) {
        if self.current_step < self.history.len() - 1 {
            self.history.truncate(self.current_step + 1);
        }
        self.history.push(self.text.clone());
        self.current_step += 1;
    }

    pub fn refresh_folder_contents(&mut self) {
        self.folder_contents = fs::read_dir(&self.folder_path)
            .unwrap()
            .filter_map(|res| res.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                // Filter out hidden files if `show_hidden_files` is false
                if !self.show_hidden_files {
                    !path.file_name().unwrap().to_str().unwrap().starts_with('.')
                } else {
                    true
                }
            })
            .collect();
    }

    pub fn show_folder_contents(ui: &mut egui::Ui, path: &PathBuf, opened_file: &mut Option<(PathBuf, String)>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            let entry_name = entry.file_name().unwrap().to_str().unwrap();
            let entry_icon = if entry.is_dir() { "📁" } else { "📄" };
            if entry.is_dir() {
                egui::CollapsingHeader::new(format!("{} {}", entry_icon, entry_name))
                    .show(ui, |ui| {
                        NotepadApp::show_folder_contents(ui, &entry, opened_file);
                    });
            } else {
                if ui.button(format!("{} {}", entry_icon, entry_name)).clicked() {
                    if let Ok(content) = fs::read_to_string(&entry) {
                        *opened_file = Some((entry.clone(), content));
                    }
                }
            }
        }
    }

    pub fn apply_theme(&self, ctx: &Context) {
        let visuals = if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        ctx.set_visuals(visuals);
    }
}
