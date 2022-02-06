//! It holds the necessary org structure like Content, Questions and answers
//! In unformatted structure.

use orgize::{
    elements::List,
    indextree::{Arena, NodeId},
};
use orgize::{Element, Headline};
use rusqlite::Row;

use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct FlashCard {
    doc_id: i64,
    questions: String,
    answers: String,
    confidence: f64,
}

#[derive(Debug, Clone)]
pub struct Document {
    id: i64,
    title: String,
    content: String,
    cards: Vec<FlashCard>,
}

impl From<&Row<'_>> for Document {
    fn from(row: &Row) -> Self {
        Document {
            id: row.get_unwrap("id"),
            title: row.get_unwrap("title"),
            content: row.get_unwrap("content"),
            cards: vec![],
        }
    }
}

impl FlashCard {
    pub fn new() -> Self {
        FlashCard {
            doc_id: 0,
            questions: String::new(),
            answers: String::new(),
            confidence: 0.0,
        }
    }

    pub fn from_db(questions: &str, answers: &str, id: i64, confidence: f64) -> Self {
        FlashCard {
            doc_id: id,
            questions: questions.to_string(),
            answers: answers.to_string(),
            confidence,
        }
    }

    pub fn add_question(&mut self, question: &str) {
        self.questions += question;
    }

    pub fn add_answer(&mut self, answer: &str) {
        self.answers += answer;
        self.answers += ", ";
    }

    pub fn get_id(&self) -> i64 {
        self.doc_id
    }

    pub fn get_questions(&self) -> &str {
        &self.questions
    }

    pub fn get_answers(&self) -> &str {
        &self.answers
    }

    pub fn get_confidence(&self) -> f64 {
        self.confidence
    }
}

impl Document {
    pub fn new() -> Self {
        Document {
            id: 0,
            title: String::new(),
            content: String::new(),
            cards: vec![],
        }
    }

    pub fn handle_headline<'a>(&mut self, id: NodeId, arena: &Arena<Element<'a>>) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            // Check if this is a some other element like code block.
            if let Element::Title(title) = data {
                if !(title.tags.contains(&Cow::Borrowed("card"))) {
                    self.handle_normal_headline(id, arena, &2, usize::MAX);
                } else {
                    self.handle_flashcards(id, arena)
                }
            }
        }
    }

    pub fn handle_normal_headline<'a>(
        &mut self,
        id: NodeId,
        arena: &Arena<Element<'a>>,
        level: &usize,
        idx: usize,
    ) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::Section => self.handle_section(child, arena),
                Element::Headline { level: size } => {
                    self.handle_normal_headline(child, arena, size, idx)
                }
                Element::Title(title) => {
                    if title.tags.contains(&Cow::Borrowed("card")) {
                        self.handle_flashcards(id, arena);
                        return;
                    }
                    if usize::MAX > idx {
                        // Do not touch this. It is deep in recursion.
                        if let Some(flashcard) = self.cards.get_mut(idx as usize) {
                            flashcard.add_answer(&title.raw);
                        }
                    } else {
                        for _ in 0..*level {
                            self.content += "*";
                        }
                        self.content += " ";
                        self.content += &title.raw;
                        self.content += "\n";
                    }
                }
                _ => {}
            }
        }
    }
    pub fn handle_paragraph<'a>(&mut self, blank: i32, id: NodeId, arena: &Arena<Element<'a>>) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::Text { value: text } => {
                    self.content += &text;
                }
                Element::Bold => {
                    self.content += "*";
                    self.handle_paragraph(-1, child, arena);
                    self.content += "*";
                }
                _ => println!("{:?}", data),
            }
        }
        for _ in 0..(blank + 1) {
            self.content += "\n";
        }
    }

    pub fn handle_list<'a>(&mut self, list: &List, id: NodeId, arena: &Arena<Element<'a>>) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::List(nested_list) => self.handle_list(nested_list, child, arena),
                Element::ListItem(item) => {
                    for _ in 0..item.indent {
                        self.content += " ";
                    }
                    self.content += &item.bullet;
                    self.handle_list(list, child, arena);
                }
                Element::Paragraph { post_blank: blank } => {
                    self.handle_paragraph(*blank as i32, child, arena)
                }
                _ => println!("ListItem: {:?}", data),
            }
        }
        for _ in 0..list.post_blank as i32 {
            self.content += "\n";
        }
    }

    /// Since section doesn't contain other headlines all the elements within
    /// The section are present here.
    pub fn handle_section<'a>(&mut self, section_id: NodeId, arena: &Arena<Element<'a>>) {
        // We fetch the relevant data here.
        let data = arena.get(section_id).unwrap().get();
        match data {
            Element::Section => {
                for child in section_id.children(arena) {
                    self.handle_section(child, arena);
                }
            }
            Element::Paragraph { post_blank: blank } => {
                self.handle_paragraph(*blank as i32, section_id, arena)
            }
            Element::List(list) => self.handle_list(list, section_id, arena),
            Element::SourceBlock(_source) => println!("TODO: Insert code block"),
            _ => {}
        }
    }

    pub fn handle_flashcards<'a>(&mut self, id: NodeId, arena: &Arena<Element<'a>>) {
        for child_id in id.children(arena) {
            let data = arena.get(child_id).unwrap().get();
            match data {
                Element::Title(title) => {
                    let mut flash_card = FlashCard::new();
                    flash_card.add_question(&title.raw);
                    self.cards.push(flash_card);
                }
                Element::Headline { level } => {
                    self.handle_normal_headline(child_id, arena, level, self.cards.len() - 1);
                }
                _ => {}
            }
        }
    }

    /// This function is responsible for fetching the context. It is the source of
    /// truth.
    pub fn handle_context<'a>(&mut self, headline: &'a Headline, arena: &Arena<Element<'a>>) {
        let node_id = headline.title_node();
        let node = arena.get(node_id).unwrap().get();
        // We make sure that we are only accessing the context title.
        if let Element::Title(title) = node {
            // We yeet out if this is not a context.
            if !(title.tags.contains(&Cow::Borrowed("context"))) {
                return;
            }
            self.content += "* ";
            self.content += &title.raw;
            self.title += &title.raw;
            self.content += " :context:";
            self.content += "\n";
            // We will access the section data for this context. The actual data lies
            // Inside the root node. Therefore we will call the handle section with
            // given section id.
            if let Some(section_id) = headline.section_node() {
                self.handle_section(section_id, arena);
            }
            for headline in headline.headline_node().children(arena) {
                self.handle_headline(headline, arena);
            }
        }
    }

    pub fn get_contents(&self) -> &str {
        &self.content
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_cards(&self) -> &Vec<FlashCard> {
        &self.cards
    }

    pub fn update_id(&mut self, id: i64) {
        self.id = id;
        for card in self.cards.iter_mut() {
            card.doc_id = id;
        }
    }
}
