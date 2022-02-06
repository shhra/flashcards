use super::{content_ui::DocumentUI, files_ui::FileUI, settings_ui::SettingsUI};
use crate::database::Database;
use crate::org::FlashCard;
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

    cards: Vec<FlashCard>,
    active_card: usize,

    start_session: bool,
    reveal: bool,
    repeat: bool,
    done: bool,
    stats: HashMap<usize, bool>,
}

impl Default for App {
    fn default() -> Self {
        let db = Database::connect().unwrap();
        App {
            cards: vec![],
            start_session: false,
            rng: thread_rng(),
            active_card: 0,
            reveal: false,
            repeat: false,
            stats: HashMap::new(),
            done: false,
            files: FileUI::new(&db),
            document: DocumentUI::new(),
            settings: SettingsUI::new(),
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
                    if self.reveal || self.repeat {
                        self.document.load_item(
                            &self.db,
                            self.cards[self.active_card].get_id(),
                            ui,
                        );
                    } else if !self.start_session || self.done {
                        self.files.update_files(ui, &mut self.db);
                        if self.files.should_import {
                            self.fetch_flashcards();
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
            if self.start_session && !self.done {
                self.lower_buttons(ui);
            } else {
                self.initial_buttons(ui);
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.start_session && !self.done {
                self.show_card(ui);
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
        self.fetch_flashcards();
    }

    fn fetch_flashcards(&mut self) {
        // Select 20 flash cards from the
        if let Ok(result) = self.db.get_flashcards(self.settings.num_cards) {
            self.cards = result;
        }
        for id in 0..self.cards.len() {
            self.stats.entry(id).or_insert(false);
        }
    }

    fn reset(&mut self) {
        for (_, value) in self.stats.iter_mut() {
            *value = false;
        }
        self.done = false;
        self.active_card = 0;
        self.reveal = false;
    }

    fn show_card(&mut self, ui: &mut Ui) {
        let widget_size = 0.8 * ui.max_rect().size();
        // println!("Max Rect: {:?}", ui.max_rect().size());
        let widget_offset = 0.1 * ui.min_rect().size();
        // println!("Min Rect: {:?}", ui.min_rect().size());
        let widget_rect = Rect::from_min_size(ui.min_rect().min + widget_offset, widget_size);

        let card = &self.cards[self.active_card];
        if self.reveal {
            let mut job = LayoutJob::default();
            job.append(
                card.get_answers(),
                0.0,
                TextFormat {
                    style: TextStyle::Heading, // TODO: update font id later.
                    ..Default::default()
                },
            );
            let label = Label::new(job);
            ui.put(widget_rect, label);
            return;
        }
        let mut job = LayoutJob::default();
        job.append(
            card.get_questions(),
            0.0,
            TextFormat {
                style: TextStyle::Heading, // TODO: update font id later.
                ..Default::default()
            },
        );
        let label = Label::new(job);
        ui.put(widget_rect, label);
    }

    fn next_question(&mut self) {
        let vec: Vec<usize> = self
            .stats
            .iter()
            .filter(|(_, y)| !*y)
            .map(|(x, _)| *x)
            .collect();

        if vec.len() <= 0 {
            self.done = true;
            return;
        }

        let dist = rand::distributions::Uniform::new_inclusive(0, vec.len() - 1);
        self.active_card = vec[self.rng.sample(dist)];
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
            self.unset_reveal();
            self.repeat = true;
        }

        let mut offset = ui.min_rect().size();
        offset.x *= 0.40;
        offset.y *= 0.30;
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if ui.put(widget_rect, Button::new("Good")).clicked() {
            self.unset_reveal();
            self.stats.entry(self.active_card).and_modify(|x| *x = true);
            self.next_question();
        }

        let mut offset = ui.min_rect().size();
        offset.x *= 0.70;
        offset.y *= 0.30;
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, widget_size);
        if !self.reveal {
            if ui.put(widget_rect, Button::new("Reveal")).clicked() {
                self.reveal = true;
            }
        } else {
            if ui.put(widget_rect, Button::new("Next")).clicked() {
                self.unset_reveal();
                self.next_question();
            }
        }
    }

    fn unset_reveal(&mut self) {
        self.reveal = false;
        self.repeat = false;
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
            self.reset();
        }
    }
}
