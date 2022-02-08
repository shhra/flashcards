use std::collections::HashMap;

use eframe::egui::Ui;
use egui::{text::LayoutJob, Label, Rect, TextFormat, TextStyle};
use rand::prelude::*;

use crate::{database::Database, org::FlashCard};

use super::content_ui::DocumentUI;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CardsUI {
    cards: Vec<FlashCard>,
    active_card: usize,
    repeat: bool,
    reveal: bool,
    done: bool,
    stats: HashMap<usize, bool>,
}

impl CardsUI {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            active_card: 0,
            repeat: false,
            reveal: false,
            done: false,
            stats: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn show_content(&self, db: &Database, document: &mut DocumentUI, ui: &mut Ui) {
        if self.reveal || self.repeat {
            document.load_item(db, self.cards[self.active_card].get_id(), ui);
        }
    }

    pub fn fetch(&mut self, db: &Database, num_cards: i32) {
        // Select 20 flash cards from the
        if let Ok(result) = db.get_flashcards(num_cards) {
            self.cards = result;
        }
        for id in 0..self.cards.len() {
            self.stats.entry(id).or_insert(false);
        }
    }

    pub fn show(&self, ui: &mut Ui) {
        let widget_size = 0.8 * ui.max_rect().size();
        let widget_offset = 0.1 * ui.min_rect().size();
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

    pub fn next(&mut self, rng: &mut ThreadRng) {
        self.reveal = false;
        self.repeat = false;
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
        self.active_card = vec[rng.sample(dist)];
    }

    pub fn reset(&mut self) {
        for (_, value) in self.stats.iter_mut() {
            *value = false;
        }
        self.done = false;
        self.active_card = 0;
        self.reveal = false;
    }

    pub fn update_and_next(&mut self, rng: &mut ThreadRng) {
        self.stats.entry(self.active_card).and_modify(|x| *x = true);
        self.next(rng);
    }

    pub fn reveal(&mut self) {
        self.reveal = true;
   }

    pub fn repeat(&mut self) {
        self.reveal = false;
        self.repeat = true;
    }

    pub fn is_reveal(&self) -> bool{
        self.reveal
    }
}
