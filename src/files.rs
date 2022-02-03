use std::{
    fs,
    path::Path,
};

use rusqlite::Row;

#[derive(Debug)]
pub struct File {
    id: i64,
    file_path: String,
    file_name: String,
}

impl File {
    pub fn new() -> Self {
        File {
            file_path: String::new(),
            file_name: String::new(),
            id: 0,
        }
    }

    pub fn load_path(&mut self, path: &Path) {
        // Extract the last element.
        if let Some(name) = path.file_name() {
            match fs::canonicalize(path) {
                Ok(path) => {
                    self.file_path = path.to_str().unwrap().to_owned();
                    self.file_name =
                        name.to_str().unwrap().split(".").collect::<Vec<&str>>()[0].to_owned();
                }
                Err(_) => {
                    println!("You pointed to something that doesn't exists.");
                }
            }
        } else {
            println!("Please fill in the path position. You've missed that!");
        }
    }

    pub fn get_path(&self) -> &str {
        &self.file_path
    }

    pub fn get_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<&Row<'_>> for File {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            file_path: row.get_unwrap("path"),
            file_name: row.get_unwrap("name"),
        }
    }
}
