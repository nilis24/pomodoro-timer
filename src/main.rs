use eframe::egui::{self, FontId, TextStyle};

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

#[derive(PartialEq)]
enum Tab {
    AvailableTime,
    Cycles,
    Settings,
    TimeLog,
}

struct PomodoroApp {
    tab: Tab,

    remaining_seconds: u32,

    available_hours: u32,
    available_minutes: u32,
    cycle_count: u32,

    work_minutes: u32,
    short_break_minutes: u32,
    long_break_minutes: u32,
}

impl Default for PomodoroApp {
    fn default() -> Self {
        Self {
            tab: Tab::AvailableTime,

            remaining_seconds: 25 * 60,

            available_hours: 2,
            available_minutes: 0,
            cycle_count: 4,

            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
        }
    }
}

impl eframe::App for PomodoroApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.style_mut().text_styles = [
            (TextStyle::Heading, FontId::proportional(26.0)),
            (TextStyle::Body, FontId::proportional(18.0)),
            (TextStyle::Button, FontId::proportional(18.0)),
            (TextStyle::Monospace, FontId::monospace(17.0)),
            (TextStyle::Small, FontId::proportional(15.0)),
        ]
        .into();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

                centered_row(ui, 515.0, |ui| {
                    ui.selectable_value(&mut self.tab, Tab::AvailableTime, "Temps disponible");

                    ui.selectable_value(&mut self.tab, Tab::Cycles, "Cicles");

                    ui.selectable_value(&mut self.tab, Tab::Settings, "Configuració");

                    ui.selectable_value(&mut self.tab, Tab::TimeLog, "Registre de temps");
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                match self.tab {
                    Tab::AvailableTime => self.available_time_ui(ui),
                    Tab::Cycles => self.cycles_ui(ui),
                    Tab::Settings => self.settings_ui(ui),
                    Tab::TimeLog => self.time_log_ui(ui),
                }
            });
        });
    }
}

impl PomodoroApp {
    fn available_time_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Pomodoro per temps disponible");

            ui.add_space(18.0);

            centered_row(ui, 270.0, |ui| {
                ui.label("Tinc disponibles:");

                ui.add(
                    egui::DragValue::new(&mut self.available_hours)
                        .range(0..=24)
                        .suffix(" h"),
                );

                ui.add(
                    egui::DragValue::new(&mut self.available_minutes)
                        .range(0..=59)
                        .suffix(" min"),
                );
            });

            ui.add_space(18.0);

            if ui.button("Iniciar pomodoro").clicked() {
                println!(
                    "Inicia amb temps disponible: {} h {} min",
                    self.available_hours, self.available_minutes
                );
            }
        });
    }

    fn cycles_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Pomodoro per cicles");

            ui.add_space(18.0);

            centered_row(ui, 175.0, |ui| {
                ui.label("Vull fer:");

                ui.add(
                    egui::DragValue::new(&mut self.cycle_count)
                        .range(1..=24)
                        .suffix(" cicles"),
                );
            });

            ui.add_space(18.0);

            if ui.button("Iniciar pomodoro").clicked() {
                println!("Inicia amb nombre de cicles: {}", self.cycle_count);
            }
        });
    }

    fn settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Configuració");

            ui.add_space(18.0);

            centered_row(ui, 135.0, |ui| {
                ui.label("Treball:");

                ui.add(
                    egui::DragValue::new(&mut self.work_minutes)
                        .range(1..=180)
                        .suffix(" min"),
                );
            });

            ui.add_space(8.0);

            centered_row(ui, 190.0, |ui| {
                ui.label("Descans curt:");

                ui.add(
                    egui::DragValue::new(&mut self.short_break_minutes)
                        .range(1..=60)
                        .suffix(" min"),
                );
            });

            ui.add_space(8.0);

            centered_row(ui, 195.0, |ui| {
                ui.label("Descans llarg:");

                ui.add(
                    egui::DragValue::new(&mut self.long_break_minutes)
                        .range(1..=120)
                        .suffix(" min"),
                );
            });

            ui.add_space(18.0);

            if ui.button("Aplica").clicked() {
                self.remaining_seconds = self.work_minutes * 60;

                self.tab = Tab::AvailableTime;
            }
        });
    }

    fn time_log_ui(&mut self, ui: &mut egui::Ui) {
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
                ui.add_sized([column_width, 20.0], egui::Label::new(egui::RichText::new("Dia i hora").strong()));
                ui.add_sized([column_width, 20.0], egui::Label::new(egui::RichText::new("Temps treballat").strong()));
                ui.add_sized([column_width, 20.0], egui::Label::new(egui::RichText::new("Temps de descans").strong()));
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
}

fn centered_row<R>(
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
