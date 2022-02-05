use crate::{database::Database, org::Document};
use eframe::egui::{RichText, Ui};
use orgize::{
    elements::List,
    indextree::{Arena, NodeId},
    Element, Headline, Org,
};
use std::{borrow::Cow, collections::HashMap};

pub enum Style {
    List { indent: i32, bullet: String },
    Bold,
    Italics,
    Default,
}

pub struct DocumentUI {
    cached_content: HashMap<i64, Document>,
}

impl DocumentUI {
    pub fn new() -> Self {
        Self {
            cached_content: HashMap::new(),
        }
    }

    pub fn handle_section<'a>(
        &mut self,
        section_id: NodeId,
        arena: &Arena<Element<'a>>,
        ui: &mut Ui,
        
    ) {
        // We fetch the relevant data here.
        let data = arena.get(section_id).unwrap().get();
        match data {
            Element::Section => {
                for child in section_id.children(arena) {
                    self.handle_section(child, arena, ui);
                }
            }
            Element::Paragraph { post_blank: blank } => {
                self.handle_paragraph(*blank as i32, section_id, arena, &Style::Default, ui);
            }
            Element::List(list) => self.handle_list(list, section_id, arena, &Style::Default, ui),
            Element::SourceBlock(_source) => println!("TODO: Insert code block"),
            _ => {}
        }
    }

    fn handle_headline<'a>(&mut self, id: NodeId, arena: &Arena<Element<'a>>, ui: &mut Ui) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            // Check if this is a some other element like code block.
            if let Element::Title(title) = data {
                self.handle_normal_headline(id, arena, &2, ui);
            }
        }
    }

    fn handle_normal_headline<'a>(
        &mut self,
        id: NodeId,
        arena: &Arena<Element<'a>>,
        level: &usize,
        ui: &mut Ui,
    ) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::Section => self.handle_section(child, arena, ui),
                Element::Headline { level: size } => {
                    self.handle_normal_headline(child, arena, size, ui)
                }
                Element::Title(title) => {
                    // TODO: Handle size.
                    ui.label(RichText::new(&*title.raw).heading());
                }
                _ => {}
            }
        }
    }

    pub fn handle_paragraph<'a>(
        &mut self,
        blank: i32,
        id: NodeId,
        arena: &Arena<Element<'a>>,
        style: &Style,
        ui: &mut Ui,
    ) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::Text { value: text } => {
                    let text = match style {
                        Style::List { indent, bullet } => RichText::new(&**text),
                        Style::Bold => RichText::new(&**text).strong(),
                        Style::Italics => RichText::new(&**text).italics(),
                        Style::Default => RichText::new(&**text),
                    };
                    ui.label(text);
                }
                Element::Bold => {
                    self.handle_paragraph(-1, child, arena, &Style::Bold, ui);
                }
                _ => println!("{:?}", data),
            }
        }
        for _ in 0..(blank + 1) {
            ui.label("\n");
        }
    }

    fn handle_list<'a>(
        &mut self,
        list: &List,
        id: NodeId,
        arena: &Arena<Element<'a>>,
        style: &Style,
        ui: &mut Ui,
    ) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::List(nested_list) => {
                    self.handle_list(nested_list, child, arena, &Style::Default, ui)
                }
                Element::ListItem(item) => {
                    self.handle_list(
                        list,
                        child,
                        arena,
                        &Style::List {
                            indent: item.indent as i32,
                            bullet: item.bullet.to_string(),
                        },
                        ui,
                    );
                }
                Element::Paragraph { post_blank: blank } => {
                    self.handle_paragraph(*blank as i32, child, arena, &Style::Default, ui);
                }
                _ => println!("ListItem: {:?}", data),
            }
        }
        for _ in 0..list.post_blank as i32 {
            ui.label("\n");
        }
    }

    fn handle_context<'a>(
        &mut self,
        headline: &'a Headline,
        arena: &Arena<Element<'a>>,
        ui: &mut Ui,
    ) {
        let node_id = headline.title_node();
        let node = arena.get(node_id).unwrap().get();
        // We make sure that we are only accessing the context title.
        if let Element::Title(title) = node {
            // We yeet out if this is not a context.
            if !(title.tags.contains(&Cow::Borrowed("context"))) {
                return;
            }
            ui.label(RichText::new(&*title.raw).heading());

            if let Some(section_id) = headline.section_node() {
                self.handle_section(section_id, arena, ui);
            }
            for headline in headline.headline_node().children(arena) {
                self.handle_headline(headline, arena, ui);
            }
        }
    }

    pub fn load_item(&mut self, db: &Database, id: i64, ui: &mut Ui) {
        if !self.cached_content.contains_key(&id) {
            self.cached_content
                .entry(id)
                .or_insert(db.load_data(id).ok().unwrap()[0].clone());
        }
        if let Some(data) = self.cached_content.get(&id) {
            let string = &data.get_contents().to_owned();
            let content_data = Org::parse(&string);
            let arena = content_data.arena();
            for headline in content_data.headlines() {
                self.handle_context(&headline, &arena, ui)
            }
        } else {
            println!("{:#?}", self.cached_content);
        }
    }
}
