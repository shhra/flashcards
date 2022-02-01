use orgize::{
    elements::List,
    indextree::{Arena, NodeId},
};
use orgize::{Element, Headline, Org};
use std::io::Error as IOError;
use std::path;
use std::string::FromUtf8Error;
use std::{borrow::Cow, fs};

#[derive(Debug)]
enum Errors {
    IO(IOError),
    Utf8(FromUtf8Error),
}

// Since we are defining custom error, we have to impl From<IOError>
impl From<IOError> for Errors {
    fn from(err: IOError) -> Self {
        Errors::IO(err)
    }
}

impl From<FromUtf8Error> for Errors {
    fn from(err: FromUtf8Error) -> Self {
        Errors::Utf8(err)
    }
}

fn handle_headline<'a>(id: NodeId, arena: &Arena<Element<'a>>) {
    for child in id.children(arena) {
        let data = arena.get(child).unwrap().get();
        // Check if this is a some other element like code block.
        if let Element::Title(title) = data {
            if !(title.tags.contains(&Cow::Borrowed("card"))) {
                handle_normal_headline(id, arena, &2);
            } else {
                handle_flashcards(id, arena)
            }
        }
    }
}

fn handle_normal_headline<'a>(id: NodeId, arena: &Arena<Element<'a>>, level: &usize) {
    for child in id.children(arena) {
        let data = arena.get(child).unwrap().get();
        match data {
            Element::Section => handle_section(child, arena),
            Element::Headline { level: size } => handle_normal_headline(child, arena, size),
            Element::Title(title) => println!("{:?} {:?}", level, title.raw),
            _ => {}
        }
    }
}
fn handle_paragraph<'a>(blank: i32, id: NodeId, arena: &Arena<Element<'a>>) {
    for child in id.children(arena) {
        let data = arena.get(child).unwrap().get();
        match data {
            Element::Text { value: text } => print!("{:?}", text),
            Element::Bold => {
                print!("*");
                handle_paragraph(-1, child, arena);
                print!("*");
            }
            _ => println!("{:?}", data),
        }
    }
    // for _ in 0..*blank {
    for _ in 0..(blank + 1) {
        print!("\n");
    }
}

fn handle_list<'a>(list: &List, id: NodeId, arena: &Arena<Element<'a>>) {
    for child in id.children(arena) {
        let data = arena.get(child).unwrap().get();
        match data {
            Element::List(nested_list) => handle_list(nested_list, child, arena),
            Element::ListItem(item) => {
                for _ in 0..item.indent {
                    print!(" ");
                }
                print!("{:?}", item.bullet);
                handle_list(list, child, arena);
            }
            Element::Paragraph { post_blank: blank } => {
                handle_paragraph(*blank as i32, child, arena)
            }
            _ => println!("ListItem: {:?}", data),
        }
    }
    for _ in 0..list.post_blank as i32 {
        print!("\n")
    }
}

/// Since section doesn't contain other headlines all the elements within
/// The section are present here.
fn handle_section<'a>(section_id: NodeId, arena: &Arena<Element<'a>>) {
    // We fetch the relevant data here.
    let data = arena.get(section_id).unwrap().get();
    match data {
        Element::Section => {
            for child in section_id.children(arena) {
                handle_section(child, arena);
            }
        }
        Element::Paragraph { post_blank: blank } => {
            handle_paragraph(*blank as i32, section_id, arena)
        }
        Element::List(list) => handle_list(list, section_id, arena),
        Element::SourceBlock(_source) => println!("TODO: Insert code block"),
        _ => {}
    }
}

fn handle_flashcards<'a>(id: NodeId, arena: &Arena<Element<'a>>) {
    // Here is how we create different flashcards.
    for child_id in id.children(arena) {
        let data = arena.get(child_id).unwrap().get();
        match data {
            Element::Title(title) => println!("Q: {:?}", title.raw),
            Element::Headline { level } => handle_normal_headline(child_id, arena, level),
            _ => {}
        }
    }
}

/// This function is responsible for fetching the context. It is the source of

/// truth.
fn handle_context<'a>(headline: &'a Headline, arena: &Arena<Element<'a>>) {
    let node_id = headline.title_node();
    let node = arena.get(node_id).unwrap().get();
    // We make sure that we are only accessing the context title.
    if let Element::Title(title) = node {
        // We yeet out if this is not a context.
        if !(title.tags.contains(&Cow::Borrowed("context"))) {
            return;
        }
        // TODO: Create actual content
        println!("{:#?}", title.raw);
        // We will access the section data for this context. The actual data lies
        // Inside the root node. Therefore we will call the handle section with
        // given section id.
        let section_id = headline.section_node().unwrap();
        handle_section(section_id, arena);
        for headline in headline.headline_node().children(arena) {
            handle_headline(headline, arena);
        }
    }
}

fn main() {
    // For now this path is hardcoded
    // TODO: Make it dynamic
    let file_path = path::Path::new("./fc.org");
    let contents = String::from_utf8(fs::read(&file_path).unwrap()).unwrap();

    // Parse the contents.
    let content_data = Org::parse(&contents);
    let arena = content_data.arena();

    // Load the data.
    for headline in content_data.headlines() {
        handle_context(&headline, arena)
    }
}
