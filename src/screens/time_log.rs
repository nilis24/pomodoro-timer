use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Registre de temps");
    });

    ui.add_space(18.0);

    let column_spacing = 16.0;
    let column_width = (ui.available_width() - column_spacing * 2.0) / 3.0;

    egui::Grid::new("time_log_table")
        .num_columns(3)
        .striped(true)
        .spacing([column_spacing, 10.0])
        .min_col_width(column_width)
        .show(ui, |ui| {
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Dia i hora").strong()),
            );
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Temps treballat").strong()),
            );
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Temps de descans").strong()),
            );
            ui.end_row();

            ui.add_sized([column_width, 20.0], egui::Label::new("06/09/2026 09:00"));
            ui.add_sized([column_width, 20.0], egui::Label::new("1 h 40 min"));
            ui.add_sized([column_width, 20.0], egui::Label::new("20 min"));
            ui.end_row();

            ui.add_sized([column_width, 20.0], egui::Label::new("06/09/2026 12:15"));
            ui.add_sized([column_width, 20.0], egui::Label::new("50 min"));
            ui.add_sized([column_width, 20.0], egui::Label::new("10 min"));
            ui.end_row();

            ui.add_sized([column_width, 20.0], egui::Label::new("05/09/2026 17:30"));
            ui.add_sized([column_width, 20.0], egui::Label::new("2 h 05 min"));
            ui.add_sized([column_width, 20.0], egui::Label::new("25 min"));
            ui.end_row();
        });
}
