use std::io::Error;

use rusqlite::{params, Connection, Result};

use crate::org::{Document, FlashCard};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn connect() -> Result<Self> {
        if let Err(err) = Database::create_tables() {
            println!("Error: {:?}", err);
        }
        Ok(Database {
            conn: Connection::open("./database.db3")?,
        })
    }

    pub fn create_tables() -> Result<()> {
        let conn = Connection::open("./database.db3")?;
        // If table doesn't exists create table.
        let doc = "CREATE TABLE IF NOT EXISTS documents(id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE, title TEXT UNIQUE, content TEXT)";
        conn.execute(doc, params![])?;

        let question = "CREATE TABLE IF NOT EXISTS flashcards(id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE, questions TEXT, answers TEXT, document INTEGER, FOREIGN KEY(document) REFERENCES documents(id)
            )";
        conn.execute(question, params![])?;
        if let Ok(_) = conn.close() {
            println!("Connection closed")
        }
        Ok(())
    }

    // pub fn get_last_id(self) -> i32 {
    //     self.conn.execute("SELECT * FROM documents ORDER BY id DESC LIMIT 1", )
    // }

    pub fn insert_documents(&self, document: &str, title: &str) -> Result<i64> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO documents (content, title) VALUES (?1, ?2)")?;
        stmt.execute([document, title])?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn insert_flashcards(&self, cards: &Vec<FlashCard>) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO flashcards (questions, answers, document) VALUES (?1, ?2, ?3)")?;
        for card in cards {
            stmt.execute([
                card.get_questions(),
                card.get_answers(),
                &card.get_id().to_string(),
            ])?;
        }
        Ok(())
    }

    pub fn get_last_id(&self) -> i64 {
        let data: Result<i64> = self.conn.query_row(
            "SELECT id FROM documents ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        );
        data.expect("Cannot fetch the last id.")
    }
}
