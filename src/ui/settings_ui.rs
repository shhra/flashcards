use eframe::egui::{text::FontDefinitions, CtxRef, Slider, Style, TextStyle, Ui};

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
    pub fn ui(&mut self, ctx: &CtxRef, ui: &mut Ui, fonts: &mut FontDefinitions) {
        self.style.visuals.light_dark_radio_buttons(ui);

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

        self.set_style(fonts);
        ctx.set_style(self.style.clone());
        ctx.set_fonts(fonts.clone());
    }

    fn set_style(&mut self, fonts: &mut FontDefinitions) {
        self.style.spacing.item_spacing.y = self.spacing;
        if let Some((_, size)) = fonts.family_and_size.get_mut(&TextStyle::Body) {
            *size = self.body_size;
        };
        if let Some((_, size)) = fonts.family_and_size.get_mut(&TextStyle::Heading) {
            *size = self.heading_size;
        };
        if let Some((_, size)) = fonts.family_and_size.get_mut(&TextStyle::Button) {
            *size = self.button_size;
        };
    }
}
