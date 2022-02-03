use eframe::egui;

use crate::database::Database;
use walkdir::WalkDir;
use crate::files::File;

#[derive(Debug)]
pub struct FileUI {
    picked_path: Option<String>,
    loaded_files: Vec<File>,
    imported: bool,
    display: bool,
}

impl FileUI {
    pub fn new(db: &Database) -> FileUI {
        let mut files :Vec<File>= vec![];
        if let Ok(fetched_files) = db.load_file_names(){
            files = fetched_files;
        };
        FileUI {
            picked_path: None,
            imported: false,
            display: false,
            loaded_files: files,
        }
    }

    pub fn update_files(&mut self, ui: &mut egui::Ui, db: &mut Database) {
        if ui.button("Import folder").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                self.picked_path = Some(path.display().to_string());
            }
            self.imported = true;
        }
        if self.imported {
            self.import_and_fill(db);
            if let Ok(fetched_files) = db.load_file_names(){
                self.loaded_files = fetched_files;
            }
            self.imported = false;
        }
        // Fetch from the db and load the contents.
        self.loaded_files.iter().for_each(|x| {
            ui.label(x.get_name());
        })
    }

    fn is_hidden(&self, entry: &walkdir::DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    fn import_and_fill(&mut self, db: &mut Database) {
        let picked_path = match &self.picked_path {
            Some(it) => it,
            _ => return, // TODO: Handle error here.
        };
        // Iterate through this picked path and then add them to database
        WalkDir::new(picked_path)
            .into_iter()
            .filter_entry(|e| !self.is_hidden(e))
            .filter_map(|file| file.ok())
            .for_each(|entry| {
                let name = entry.file_name();
                if !name.to_string_lossy().ends_with(".org") {
                    return;
                }
                db.insert_file(entry.path()).ok();
                // Perform the main import here.

            });
        self.imported = true;
    }
}
