use eframe::egui;
use egui_fancy_knob::{Knob, KnobStyle, LabelPosition};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Knob Example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(KnobExample::default()))),
    )
}

struct KnobExample {
    basic_value: f32,
    purple_value: f32,
    large_value: f32,
    thick_value: f32,
    red_value: f32,
    green_value: f32,
    blue_value: f32,
}

impl Default for KnobExample {
    fn default() -> Self {
        Self {
            basic_value: 0.0,
            purple_value: 0.0,
            large_value: 0.0,
            thick_value: 0.0,
            red_value: 0.0,
            green_value: 0.0,
            blue_value: 0.0,
        }
    }
}

impl eframe::App for KnobExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(
                        Knob::new(
                            self.basic_value,
                            |value| self.basic_value = value,
                            0.0..=100.0,
                            KnobStyle::Dot,
                        )
                        .with_label("Basic", LabelPosition::Right)
                        .with_size(40.0)
                        .with_font_size(10.0)
                        .with_colors(
                            egui::Color32::from_rgb(60, 60, 60),
                            egui::Color32::from_rgb(150, 150, 150),
                            egui::Color32::from_rgb(150, 150, 150),
                            egui::Color32::from_rgb(200, 200, 200),
                        )
                        .with_neutral(50.0),
                    )
                    .changed()
                {
                    println!("Basic value changed: {}", self.basic_value);
                }

                ui.add(
                    Knob::new(
                        self.purple_value,
                        |value| self.purple_value = value,
                        0.0..=100.0,
                        KnobStyle::Wiper,
                    )
                    .with_label("Purple", LabelPosition::Bottom)
                    .with_colors(
                        egui::Color32::from_rgb(60, 30, 80),
                        egui::Color32::from_rgb(200, 100, 255),
                        egui::Color32::from_rgb(200, 100, 255),
                        egui::Color32::from_rgb(230, 150, 255),
                    )
                    .with_size(50.0)
                    .with_font_size(14.0)
                    .with_stroke_width(3.0)
                    .with_step(0.1)
                    .with_neutral(50.0),
                );

                ui.add(
                    Knob::new(
                        self.large_value,
                        |value| self.large_value = value,
                        0.0..=100.0,
                        KnobStyle::Dot,
                    )
                    .with_label("Large", LabelPosition::Bottom)
                    .with_size(60.0)
                    .with_font_size(16.0),
                );

                ui.add(
                    Knob::new(
                        self.thick_value,
                        |value| self.thick_value = value,
                        0.0..=100.0,
                        KnobStyle::Wiper,
                    )
                    .with_label("Thick", LabelPosition::Bottom)
                    .with_size(50.0)
                    .with_stroke_width(4.0)
                    .with_neutral(50.0),
                );

                ui.add(
                    Knob::new(
                        self.red_value,
                        |value| self.red_value = value,
                        0.0..=100.0,
                        KnobStyle::Dot,
                    )
                    .with_label("Red", LabelPosition::Bottom)
                    .with_colors(
                        egui::Color32::from_rgb(80, 30, 30),
                        egui::Color32::from_rgb(220, 50, 50),
                        egui::Color32::from_rgb(220, 50, 50),
                        egui::Color32::from_rgb(255, 100, 100),
                    )
                    .with_size(50.0)
                    .with_neutral(50.0),
                );

                ui.add(
                    Knob::new(
                        self.green_value,
                        |value| self.green_value = value,
                        0.0..=100.0,
                        KnobStyle::Wiper,
                    )
                    .with_label("Leftandlongtext", LabelPosition::Left)
                    .with_colors(
                        egui::Color32::from_rgb(30, 80, 30),
                        egui::Color32::from_rgb(50, 220, 50),
                        egui::Color32::from_rgb(50, 220, 50),
                        egui::Color32::from_rgb(100, 255, 100),
                    )
                    .with_size(50.0)
                    .with_label_format(|v| format!("{:.2}%", v))
                    .with_neutral(50.0),
                );

                ui.add(
                    Knob::new(
                        self.blue_value,
                        |value| self.blue_value = value,
                        0.0..=100.0,
                        KnobStyle::Dot,
                    )
                    .with_label("Top", LabelPosition::Top)
                    .with_colors(
                        egui::Color32::from_rgb(30, 30, 80),
                        egui::Color32::from_rgb(50, 50, 220),
                        egui::Color32::from_rgb(50, 50, 220),
                        egui::Color32::from_rgb(100, 100, 255),
                    )
                    .with_size(50.0)
                    .with_neutral(50.0),
                );
            });
        });
    }
}
