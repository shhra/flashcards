use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct PathHandler {
    pub file_path: PathBuf,
    pub file_name: String,
}

impl PathHandler {
    pub fn new() -> Self {
        PathHandler {
            file_path: PathBuf::new(),
            file_name: String::new(),
        }
    }

    pub fn load_path(&mut self, path: &Path) {
        // Extract the last element.
        if let Some(name) = path.file_name() {
            match fs::canonicalize(path) {
                Ok(path) => {
                    self.file_path = path;
                    self.file_name = name.to_str().unwrap().to_owned();
                }
                Err(_) => {
                    println!("You pointed to something that doesn't exists.");
                }
            }
        } else {
            println!("Please fill in the path position. You've missed that!");
        }
    }
}
