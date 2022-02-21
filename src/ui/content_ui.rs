use crate::database::Database;
use eframe::egui::{text::LayoutJob, TextFormat, TextStyle, Ui, Visuals};
use egui::Color32;
use orgize::{
    elements::List,
    indextree::{Arena, NodeId},
    Element, Headline, Org,
};
use std::{borrow::Cow, collections::HashMap, default};

enum Style {
    List { indent: i32, bullet: String },
    Bold,
    Italics,
    Default,
}

pub struct DocumentUI {
    cached_content: HashMap<i64, LayoutJob>,
}

impl DocumentUI {
    pub fn new() -> Self {
        Self {
            cached_content: HashMap::new(),
        }
    }

    fn fill_list(indent: &i32, bullet: &str, job: &mut LayoutJob) {
        let bullet: String = match bullet {
            "* " => String::from("▫ "),
            "- " => String::from("◊ "),
            "+ " => String::from("◾ "),
            _ => {
                let data = bullet.split(".").next().unwrap();
                if let Ok(num) = data.parse::<i32>() {
                    let mut data = num.to_string();
                    data += ". ";
                    data
                } else {
                    "• ".to_owned()
                }
            }
        };
        job.append(
            &bullet,
            3.0 * *indent as f32,
            TextFormat {
                style: TextStyle::Body,
                ..Default::default()
            },
        );
    }

    fn handle_section<'a>(section_id: NodeId, arena: &Arena<Element<'a>>, job: &mut LayoutJob) {
        // We fetch the relevant data here.
        let data = arena.get(section_id).unwrap().get();
        match data {
            Element::Section => {
                for child in section_id.children(arena) {
                    DocumentUI::handle_section(child, arena, job);
                }
            }
            Element::Paragraph { post_blank: blank } => {
                DocumentUI::handle_paragraph(
                    *blank as i32,
                    section_id,
                    arena,
                    &Style::Default,
                    job,
                );
            }
            Element::List(list) => {
                DocumentUI::handle_list(list, section_id, arena, &Style::Default, job)
            }
            Element::SourceBlock(_source) => println!("TODO: Insert code block"),
            _ => {}
        }
    }

    fn handle_headline<'a>(id: NodeId, arena: &Arena<Element<'a>>, job: &mut LayoutJob) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            // Check if this is a some other element like code block.
            if let Element::Title(title) = data {
                DocumentUI::handle_normal_headline(id, arena, &2, job);
            }
        }
    }

    fn handle_normal_headline<'a>(
        id: NodeId,
        arena: &Arena<Element<'a>>,
        level: &usize,
        job: &mut LayoutJob,
    ) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::Section => DocumentUI::handle_section(child, arena, job),
                Element::Headline { level: size } => {
                    DocumentUI::handle_normal_headline(child, arena, size, job)
                }
                Element::Title(title) => {
                    // TODO: Handle size.
                    let mut data = String::new();
                    data += "\n";
                    data += &title.raw;
                    data += "\n";
                    job.append(
                        &data,
                        0.0,
                        TextFormat {
                            style: TextStyle::Heading,
                            ..Default::default()
                        },
                    );
                }
                _ => {}
            }
        }
    }

    fn handle_paragraph<'a>(
        blank: i32,
        id: NodeId,
        arena: &Arena<Element<'a>>,
        style: &Style,
        job: &mut LayoutJob,
    ) {
        let mut i = 0;
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::Text { value: text } => match style {
                    Style::List { indent, bullet } => {
                        if i < 1 {
                            DocumentUI::fill_list(indent, bullet, job);
                        }
                        job.append(&**text, 0.0, TextFormat::default());
                    }
                    Style::Bold => {
                        job.append(
                            &**text,
                            0.0,
                            TextFormat {
                                color: Visuals::default().strong_text_color(),
                                ..Default::default()
                            },
                        );
                    }
                    Style::Italics => {
                        job.append(&**text, 0.0, TextFormat::default());
                    }
                    Style::Default => {
                        job.append(&**text, 0.0, TextFormat::default());
                    }
                },
                Element::Bold => {
                    DocumentUI::handle_paragraph(-1, child, arena, &Style::Bold, job);
                }
                _ => println!("{:?}", data),
            }
            i += 1;
        }
        let mut data = String::new();
        for _ in 0..(blank + 1) {
            data += "\n";
        }
        job.append(&data, 0.0, TextFormat::default());
    }

    fn handle_list<'a>(
        list: &List,
        id: NodeId,
        arena: &Arena<Element<'a>>,
        style: &Style,
        job: &mut LayoutJob,
    ) {
        for child in id.children(arena) {
            let data = arena.get(child).unwrap().get();
            match data {
                Element::List(nested_list) => {
                    DocumentUI::handle_list(nested_list, child, arena, &Style::Default, job)
                }
                Element::ListItem(item) => {
                    DocumentUI::handle_list(
                        list,
                        child,
                        arena,
                        &Style::List {
                            indent: item.indent as i32,
                            bullet: item.bullet.to_string(),
                        },
                        job,
                    );
                }
                Element::Paragraph { post_blank: blank } => {
                    DocumentUI::handle_paragraph(*blank as i32, child, arena, style, job);
                }
                _ => println!("ListItem: {:?}", data),
            }
        }
        let mut data = String::new();
        for _ in 0..list.post_blank as i32 {
            data += "\n";
        }
        job.append(&data, 0.0, TextFormat::default());
    }

    fn handle_context<'a>(headline: &'a Headline, arena: &Arena<Element<'a>>, job: &mut LayoutJob) {
        let node_id = headline.title_node();
        let node = arena.get(node_id).unwrap().get();
        // We make sure that we are only accessing the context title.
        match node {
            Element::Title(title) => {
                // We yeet out if this is not a context.
                if !(title.tags.contains(&Cow::Borrowed("context"))) {
                    return;
                }

                let mut data = String::new();
                data += &title.raw;
                data += "\n";
                job.append(
                    &data,
                    0.0,
                    TextFormat {
                        style: TextStyle::Heading,
                        ..Default::default()
                    },
                );
                if let Some(section_id) = headline.section_node() {
                    DocumentUI::handle_section(section_id, arena, job);
                }
                for headline in headline.headline_node().children(arena) {
                    DocumentUI::handle_headline(headline, arena, job);
                }
            }
            _ => (),
        }
    }

    /// If the function is called for the first time, it fetches the content
    /// from the database and caches.
    /// By default function displays the content in the main content section.
    pub fn load_item(&mut self, db: &Database, id: i64, ui: &mut Ui) {
        if !self.cached_content.contains_key(&id) {
            self.cached_content.entry(id).or_insert({
                let data = db.load_data(id).ok().unwrap()[0].clone();
                let mut job = LayoutJob::default();
                let string = &data.get_contents().to_owned();
                let content_data = Org::parse(&string);
                let arena = content_data.arena();
                for headline in content_data.headlines() {
                    DocumentUI::handle_context(&headline, &arena, &mut job)
                }
                job
            });
        }
        if let Some(job) = self.cached_content.get(&id) {
            ui.label(job.clone());
        }
    }
}
