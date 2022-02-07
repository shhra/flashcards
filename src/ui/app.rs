use super::{cards_ui::CardsUI, content_ui::DocumentUI, files_ui::FileUI, settings_ui::SettingsUI};
use crate::database::Database;
use ::egui::{text::LayoutJob, TextFormat, TextStyle};
use eframe::{
    egui::{self, Vec2},
    epi,
};
use egui::Label;
use egui::{Button, CentralPanel, Rect, SidePanel, TopBottomPanel, Ui};
use rand::prelude::*;
use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct App {
    db: Database,
    rng: ThreadRng,
    files: FileUI,
    document: DocumentUI,
    settings: SettingsUI,
    cards: CardsUI,
    start_session: bool,
}

impl Default for App {
    fn default() -> Self {
        let db = Database::connect().unwrap();
        App {
            start_session: false,
            rng: thread_rng(),
            files: FileUI::new(&db),
            document: DocumentUI::new(),
            settings: SettingsUI::new(),
            cards: CardsUI::new(),
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
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        TopBottomPanel::top("").min_height(0.0).show(ctx, |_ui| {});
        let x = 0.4 * ctx.used_size().x;
        SidePanel::right("Menu")
            .resizable(true)
            .min_width(x)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.cards.show_content(&self.db, &mut self.document, ui);
                    if !self.start_session || self.cards.is_done() {
                        self.files.update_files(ui, &mut self.db);
                        if self.files.should_import {
                            self.cards.fetch(&self.db, self.settings.num_cards);
                            self.files.should_import = false;
                        }
                    }
                })
            });

        let y = 0.3 * ctx.used_size().y;
        if &self.cards.len() < &1 {
            egui::Window::new("Warning")
                .default_size(Vec2::new(x, y))
                .vscroll(false)
                .show(ctx, |ui| {
                    ui.label("No cards in the deck.");
                });
            return;
        }

        TopBottomPanel::bottom("").min_height(y).show(ctx, |ui| {
            if self.start_session && !self.cards.is_done() {
                self.lower_buttons(ui);
            } else {
                self.initial_buttons(ui);
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.start_session && !self.cards.is_done() {
                self.cards.show(ui);
            } else {
                self.settings.ui(ctx, ui);
            }
        });
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }
}

impl App {
    pub fn init(&mut self) {
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
        if ui.put(widget_rect, Button::new("Repeat")).clicked() {
            self.cards.repeat();
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
        if !self.cards.is_reveal() {
            if ui.put(widget_rect, Button::new("Reveal")).clicked() {
                self.cards.reveal();
            }
        } else {
            if ui.put(widget_rect, Button::new("Next")).clicked() {
                self.cards.next(&mut self.rng);
            }
        }
    }

    fn initial_buttons(&mut self, ui: &mut Ui) {
        let mut widget_size = ui.max_rect().size();
        widget_size.x *= 0.20;
        widget_size.y *= 0.40;

        let mut offset = ui.min_rect().size();
        offset.x *= 0.70;
        offset.y *= 0.30;
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if ui.put(widget_rect, Button::new("Start")).clicked() {
            self.start_session = true;
            self.cards.reset();
        }
    }
}
