use std::collections::HashMap;

use eframe::egui::{text::LayoutJob, Label, Rect, TextFormat, TextStyle, Ui};
use rand::prelude::*;

use crate::{database::Database, org::FlashCard};

use super::content_ui::DocumentUI;

pub struct CardsUI {
    cards: Vec<FlashCard>,
    active_card: usize,
    repeat: bool,
    reveal: bool,
    done: bool,
    stats: HashMap<usize, bool>,
    grades: HashMap<usize, i8>,
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
            grades: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn show_content(&mut self, db: &Database, document: &mut DocumentUI, ui: &mut Ui) {
        if self.reveal || self.repeat {
            document.load_item(db, self.cards[self.active_card].get_doc_id(), ui);
        }
    }

    pub fn fetch(&mut self, db: &Database, num_cards: i32) {
        // Select 20 flash cards from the
        if let Ok(result) = db.get_flashcards(num_cards) {
            self.cards = result;
        }
        for id in 0..self.cards.len() {
            self.stats.entry(id).or_insert(false);
            self.grades.entry(id).or_insert(4);
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let widget_size = 0.8 * ui.max_rect().size();
        let widget_offset = 0.1 * ui.min_rect().size();
        let widget_rect = Rect::from_min_size(ui.min_rect().min + widget_offset, widget_size);

        let card = &self.cards[self.active_card];
        if self.reveal || self.repeat {
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
        println!("{:#?}", self.cards);
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
        if self.done {
            for (_, value) in self.stats.iter_mut() {
                *value = false;
            }
            self.cards = vec![];
            self.done = false;
            self.active_card = 0;
            self.reveal = false;
            self.repeat = false;
        }
    }

    pub fn update_and_next(&mut self, rng: &mut ThreadRng) {
        if let Some(grade) = self.grades.get(&self.active_card) {
            if grade >= &2 || grade <= &-4 {
                self.stats.entry(self.active_card).and_modify(|x| *x = true);
                let stats = &mut self.cards[self.active_card].get_stats_mut();
                stats.repeat(*grade);
            }
        }
        self.grades
            .entry(self.active_card)
            .and_modify(|x| *x = *x + 1);
        self.next(rng);
    }

    pub fn reveal(&mut self) {
        self.reveal = true;
        self.grades
            .entry(self.active_card)
            .and_modify(|x| *x = *x - 1);
    }

    pub fn repeat(&mut self) {
        self.reveal = false;
        self.repeat = true;
        self.grades
            .entry(self.active_card)
            .and_modify(|x| *x = *x - 1);
    }

    pub fn is_reveal(&self) -> bool {
        self.reveal
    }

    pub fn is_repeat(&self) -> bool {
        self.repeat
    }

    pub fn unset_repeat(&mut self) {
        self.repeat = false;
    }

    pub fn save_to_database(&self, db: &mut Database) {
        db.update_flashcards(&self.cards);
    }
}
