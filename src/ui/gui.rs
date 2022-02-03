use super::files_ui::FileUI;
use crate::database::Database;
use crate::org::FlashCard;
use eframe::{egui, epi};
use egui::Label;
pub(crate) use egui::{Button, CentralPanel, Rect, SidePanel, Slider, TopBottomPanel, Ui};
use rand::prelude::*;
use std::collections::HashMap;

pub struct App {
    db: Database,
    num_cards: i32,
    cards: Vec<FlashCard>,
    start_session: bool,
    rng: ThreadRng,
    active_card: usize,
    reveal: bool,
    done: bool,
    stats: HashMap<usize, bool>,
    files: FileUI,
}

impl Default for App {
    fn default() -> Self {
        let db =  Database::connect().unwrap();
        App {
            num_cards: 25,
            cards: vec![],
            start_session: false,
            rng: thread_rng(),
            active_card: 0,
            reveal: false,
            stats: HashMap::new(),
            done: false,
            files: FileUI::new(&db),
            db,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Flashcard"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        TopBottomPanel::top("").min_height(0.0).show(ctx, |_ui| {});

        let x = 0.4 * ctx.used_size().x;
        SidePanel::right("Menu")
            .resizable(true)
            .min_width(x)
            .show(ctx, |ui| {
                if self.reveal {
                    // Show the relevant content.
                } else if !self.start_session || self.done {
                    self.files.update_files(ui, &mut self.db);
                }
            });

        let y = 0.3 * ctx.used_size().y;
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
            }
        });
    }
}

impl App {
    pub fn init(&mut self) {
        self.fetch_flashcards();
    }

    fn fetch_flashcards(&mut self) {
        // Select 20 flash cards from the
        if let Ok(result) = self.db.get_flashcards(self.num_cards) {
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
            let label = Label::new(format!("{}", card.get_answers()));
            ui.put(widget_rect, label);
            return;
        }
        let label = Label::new(format!("{}", card.get_questions()));
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
        ui.add(Slider::new(&mut self.num_cards, 0..=200));
    }
}
