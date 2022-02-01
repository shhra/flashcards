use orgize::Org;
use std::fs;
use std::path;

mod content;
mod db;
mod org;

use db::Database;
use org::Document;

fn main() {
    // For now this path is hardcoded
    // TODO: Make it dynamic
    let file_path = path::Path::new("./fc.org");
    let contents = String::from_utf8(fs::read(&file_path).unwrap()).unwrap();
    // Parse the contents.
    let content_data = Org::parse(&contents);
    let arena = content_data.arena();

    let db = Database::connect().expect("Failed to connect");

    let mut org = Document::new();
    // Load the data.
    for headline in content_data.headlines() {
        org.handle_context(&headline, arena)
    }

    match db.insert_documents(&org.get_contents(), &org.get_title()) {
        Ok(id) => { org.update_id(id);
                    println!("id updated");}
        Err(_) => {
            org.update_id(db.get_last_id());
        }
    }

    // db.insert_flashcards(org.get_cards());
    print!("{:#?}", org);

    // let mut new_org = Document::new(2);
    // let data = org.get_contents();
    // let new_cd = Org::parse(&data);
    // let new_arena = new_cd.arena();
    // for headline in new_cd.headlines() {
    //     new_org.handle_context(&headline, new_arena)
    // }

    // print!("This is new data.\n\n");
    // new_org.print_content();
}
