mod ui;
mod notepad_app;

use notepad_app::NotepadApp;

pub fn run() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TK Notepad",
        options,
        Box::new(|_cc| Ok(Box::new(NotepadApp::default()))),
    ).expect("Failed to run the application");
}
