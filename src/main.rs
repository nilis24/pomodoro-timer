use eframe::egui;

mod app;
mod business;
mod screens;
mod ui_helpers;

use app::PomodoroApp;

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default().with_inner_size([560.0, 360.0]);

    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Pomodoro",
        options,
        Box::new(|_cc| Ok(Box::new(PomodoroApp::default()))),
    )
}

fn load_window_icon() -> Option<egui::IconData> {
    let icon_bytes = std::fs::read("assets/icon.png").ok()?;
    eframe::icon_data::from_png_bytes(&icon_bytes).ok()
}
