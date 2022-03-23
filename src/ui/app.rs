use super::{cards_ui::CardsUI, content_ui::DocumentUI, files_ui::FileUI, settings_ui::SettingsUI};
use crate::database::Database;
use eframe::{egui, epi};
use egui::{
    Button, CentralPanel, FontData, FontDefinitions, FontFamily, Rect, SidePanel, TopBottomPanel,
    Ui,
};
use rand::prelude::*;

pub struct App {
    db: Database,
    rng: ThreadRng,
    files: FileUI,
    document: DocumentUI,
    settings: SettingsUI,
    cards: CardsUI,
    start_session: bool,
    fonts: FontDefinitions,
}

impl Default for App {
    fn default() -> Self {
        let db = Database::connect().unwrap();
        App {
            start_session: false,
            rng: thread_rng(),
            files: FileUI::new(&db),
            document: DocumentUI::new(),
            settings: SettingsUI::default(),
            cards: CardsUI::new(),
            fonts: FontDefinitions::default(),
            db,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Flashcard"
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            let settings: SettingsUI = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
            self.settings = settings;
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if self.cards.is_done() {
            self.cards.save_to_database(&mut self.db);
            self.cards.reset();
        }
        TopBottomPanel::top("").min_height(0.0).show(ctx, |_ui| {});
        let x = 0.4 * ctx.used_size().x;
        SidePanel::right("Menu")
            .resizable(true)
            .min_width(x)
            .show(ctx, |ui| {
                if !self.start_session || self.cards.is_done() {
                    self.files.update_files(ui, &mut self.db);
                    if self.files.should_import {
                        // TODO: This can probably lead to some hard cases.
                        // This should be handled later on.
                        self.cards.fetch(&self.db, self.settings.num_cards);
                        self.files.should_import = false;
                    }
                } else {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            self.cards
                                .show_content(&self.db, &mut self.document, ui, ctx);
                        });
                }
            });

        if &self.cards.len() < &1 {
            // TODO: Create a rectangle and draw it and the center of the screen.
            // This will now call different sql commands to print the stats of the
            // current deck.
            // It can show the practice habit, and other things.
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.settings.ui(ctx, ui, &mut self.fonts);
                    ui.separator();
                    ui.label("You are done. Enjoy!");
                });
            });
            return;
        }

        let y = 0.3 * ctx.used_size().y;
        TopBottomPanel::bottom("").min_height(y).show(ctx, |ui| {
            if self.start_session && !self.cards.is_done() {
                self.lower_buttons(ui);
            } else {
                self.initial_buttons(ui, frame);
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.start_session && !self.cards.is_done() {
                self.cards.show(ui, ctx);
            } else {
                self.settings.ui(ctx, ui, &mut self.fonts);
            }
        });
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, &self.settings);
    }
}

impl App {
    pub fn init(&mut self) {
        // Setup fonts here.
        self.fonts.font_data.insert(
            "garamond_normal".to_owned(),
            FontData::from_static(include_bytes!(
                "../../fonts/EBGaramond-VariableFont_wght.ttf"
            )),
        );
        self.fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "garamond_normal".to_owned());

        self.cards.fetch(&self.db, self.settings.num_cards);
    }

    fn lower_buttons(&mut self, ui: &mut Ui) {
        let mut widget_size = ui.max_rect().size();
        widget_size.x *= 0.20;
        widget_size.y *= 0.40;

        let mut offset = ui.min_rect().size();
        offset.x *= 0.10;
        offset.y *= 0.30;
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if !self.cards.is_repeat() {
            if ui.put(widget_rect, Button::new("Repeat")).clicked() {
                self.cards.repeat();
            }
        } else {
            if ui.put(widget_rect, Button::new("Question")).clicked() {
                self.cards.unset_repeat();
            }
        }

        let mut offset = ui.min_rect().size();
        offset.x *= 0.40;
        offset.y *= 0.30;
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if ui.put(widget_rect, Button::new("Good")).clicked() {
            // Update stats and grades.
            self.cards.update_and_next(&mut self.rng);
        }

        let mut offset = ui.min_rect().size();
        offset.x *= 0.70;
        offset.y *= 0.30;
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if !self.cards.is_reveal() && !self.cards.is_repeat() {
            if ui.put(widget_rect, Button::new("Reveal")).clicked() {
                self.cards.reveal();
            }
        } else {
            if ui.put(widget_rect, Button::new("Next")).clicked() {
                self.cards.next(&mut self.rng);
            }
        }
    }

    fn initial_buttons(&mut self, ui: &mut Ui, frame: &epi::Frame) {
        let mut widget_size = ui.max_rect().size();
        widget_size.x *= 0.20;
        widget_size.y *= 0.40;

        let mut offset = ui.min_rect().size();
        offset.x *= 0.40;
        offset.y *= 0.30;
        let mut widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if ui.put(widget_rect, Button::new("Quit")).clicked() {
            frame.quit();
        }

        offset = ui.min_rect().size();
        offset.x *= 0.70;
        offset.y *= 0.30;
        widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if ui.put(widget_rect, Button::new("Start")).clicked() {
            self.start_session = true;
            self.cards.reset();
        }
    }
}
