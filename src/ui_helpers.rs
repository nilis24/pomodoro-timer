use eframe::egui;

pub fn centered_row<R>(
    ui: &mut egui::Ui,
    width: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    ui.horizontal(|ui| {
        let side_space = ((ui.available_width() - width) / 2.0).max(0.0);
        ui.add_space(side_space);
        add_contents(ui)
    })
    .inner
}
