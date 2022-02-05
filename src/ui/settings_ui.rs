use std::path::PathBuf;
use walkdir::WalkDir;

use eframe::egui::{text::FontDefinitions, CtxRef, Slider, Style, TextStyle, Ui};

pub struct SettingsUI {
    spacing: f32,
    body_size: f32,
    heading_size: f32,
    button_size: f32,
    fonts: FontDefinitions,
    pub style: Style,
    pub num_cards: i32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
impl SettingsUI {
    pub fn new() -> Self {
        Self {
            spacing: 5.0,
            body_size: 22.0,
            heading_size: 26.0,
            button_size: 27.0,
            fonts: FontDefinitions::default(),
            style: Style::default(),
            num_cards: 25,
        }
    }

    pub fn ui(&mut self, ctx: &CtxRef, ui: &mut Ui) {
        // self.style.visuals.light_dark_radio_buttons(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Spacing");
                ui.add(Slider::new(&mut self.spacing, 0.0..=8.0));
            });

            ui.horizontal(|ui| {
                ui.label("Body Size");
                ui.add(Slider::new(&mut self.body_size, 15.0..=30.0));
            });
            ui.horizontal(|ui| {
                ui.label("Button Size");
                ui.add(Slider::new(&mut self.button_size, 15.0..=40.0));
            });
            ui.horizontal(|ui| {
                ui.label("Heading Size");
                ui.add(Slider::new(&mut self.heading_size, 20.0..=33.0));
            });

            ui.horizontal(|ui| {
                ui.label("Number of cards");
                ui.add(Slider::new(&mut self.num_cards, 10..=200));
            });

        });

        self.set_style();
        ctx.set_style(self.style.clone());
        ctx.set_fonts(self.fonts.clone());
    }

    fn set_style(&mut self) {
        self.style.spacing.item_spacing.y = self.spacing;
        if let Some((_, size)) = self.fonts.family_and_size.get_mut(&TextStyle::Body) {
            *size = self.body_size;
        };
        if let Some((_, size)) = self.fonts.family_and_size.get_mut(&TextStyle::Heading) {
            *size = self.heading_size;
        };
        if let Some((_, size)) = self.fonts.family_and_size.get_mut(&TextStyle::Button) {
            *size = self.button_size;
        };
    }
}
