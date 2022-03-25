use eframe::egui::{Context, FontDefinitions, Slider, Style, TextStyle, Ui};
use egui::{Color32, FontFamily, FontId, Visuals};

/// Contains different vari that allows to handle different settings.
/// This maintains the exclusive state of the different variables that can be
/// tuned.
///
/// It also provides handler's to update these values.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct SettingsUI {
    spacing: f32,
    body_size: f32,
    heading_size: f32,
    button_size: f32,
    pub style: Style,
    pub num_cards: i32,
}

impl Default for SettingsUI {
    fn default() -> Self {
        Self {
            spacing: 5.0,
            body_size: 22.0,
            heading_size: 26.0,
            button_size: 27.0,
            style: Style::default(),
            num_cards: 25,
        }
    }
}

impl SettingsUI {
    pub fn ui(&mut self, ctx: &Context, ui: &mut Ui, fonts: &mut FontDefinitions) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.style.visuals, Self::light(), "☀ Light");
            ui.selectable_value(&mut self.style.visuals, Visuals::dark(), "🌙 Dark");
        });

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Spacing");
                ui.add(Slider::new(&mut self.spacing, 1.0..=5.0));
            });

            ui.horizontal(|ui| {
                ui.label("Body Size");
                ui.add(Slider::new(&mut self.body_size, 1.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Button Size");
                ui.add(Slider::new(&mut self.button_size, 1.0..=5.0));
            });
            ui.horizontal(|ui| {
                ui.label("Heading Size");
                ui.add(Slider::new(&mut self.heading_size, 1.0..=10.0));
            });

            ui.horizontal(|ui| {
                ui.label("Number of cards");
                ui.add(Slider::new(&mut self.num_cards, 10..=200));
            });
        });

        self.set_style();
        ctx.set_style(self.style.clone());
        ctx.set_fonts(fonts.clone());
    }

    fn set_style(&mut self) {
        self.style.spacing.item_spacing.y = self.spacing;
        self.style.text_styles.insert(
            TextStyle::Body,
            FontId::new(
                20.0 * (1.0 + self.body_size / 10.0),
                FontFamily::Proportional,
            ),
        );

        self.style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(
                25.0 * (1.0 + self.heading_size / 10.0),
                FontFamily::Proportional,
            ),
        );

        self.style.text_styles.insert(
            TextStyle::Button,
            FontId::new(
                25.0 * (1.0 + self.button_size / 5.0),
                FontFamily::Proportional,
            ),
        );
    }

    fn light() -> Visuals {
        Visuals {
            dark_mode: false,
            override_text_color: Some(Color32::from_rgb(10, 10, 10)),
            ..Visuals::light()
        }
    }
}
