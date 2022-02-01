use orgize::Org;
use std::path;
use std::fs;

mod content;
mod org;

use org::Document;


fn main() {
    // For now this path is hardcoded
    // TODO: Make it dynamic
    let file_path = path::Path::new("./fc.org");
    let contents = String::from_utf8(fs::read(&file_path).unwrap()).unwrap();
    // Parse the contents.
    let content_data = Org::parse(&contents);
    let arena = content_data.arena();

    let mut org = Document::new(1);
    // Load the data.
    for headline in content_data.headlines() {
        org.handle_context(&headline, arena)
    }

    org.print_content();

    // let mut new_org = Document::new(2);
    // let data = org.get_contents();
    // let new_cd = Org::parse(&data);
    // let new_arena = new_cd.arena();
    // for headline in new_cd.headlines() {
    //     new_org.handle_context(&headline, new_arena)
    // }

    // print!("This is new data.\n\n");
    // new_org.print_content();
    print!("{:#?}", org)
}
