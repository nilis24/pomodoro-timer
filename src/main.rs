use eframe::egui;

mod app;
mod business;
mod screens;
mod ui_helpers;

use app::PomodoroApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([560.0, 360.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Pomodoro",
        options,
        Box::new(|_cc| Ok(Box::new(PomodoroApp::default()))),
    )
}
