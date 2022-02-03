use orgize::Org;
use std::fs;
use std::path;

mod database;
mod org;
mod ui;
mod files;

use database::Database;
use org::Document;
use ui::App;

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

    db.insert_flashcards(org.get_cards()).ok();

    print!("{:#?}\n", org);


    let mut app = App::default();
    app.init();
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);


}
