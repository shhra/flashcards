use rusqlite::{params, Connection, Result};

use crate::org::FlashCard;
use fallible_iterator::FallibleIterator;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            conn: Connection::open("./database.db3").unwrap(),
        }
    }
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

        let question = "CREATE TABLE IF NOT EXISTS flashcards(id INTEGER PRIMARY KEY AUTOINCREMENT, questions TEXT UNIQUE, answers TEXT, document INTEGER, confidence REAL, FOREIGN KEY(document) REFERENCES documents(id)
            )";
        conn.execute(question, params![])?;
        if let Ok(_) = conn.close() {
            println!("Connection closed")
        }
        Ok(())
    }

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
            .prepare("INSERT INTO flashcards (questions, answers, document, confidence) VALUES (?1, ?2, ?3, ?4)")?;
        for card in cards {
            stmt.execute([
                card.get_questions(),
                card.get_answers(),
                &card.get_id().to_string(),
                &card.get_confidence().to_string(),
            ])?;
        }
        Ok(())
    }

    pub fn _update_flashcards(&self, cards: &Vec<FlashCard>) -> Result<()> {
        // TODO
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

    pub fn get_flashcards(&self, num: i32) -> Result<Vec<FlashCard>> {
        let mut stmt = self.conn.prepare("SELECT * from flashcards LIMIT ?")?;
        let rows = stmt.query([&num.to_string()])?;
        rows.map(|row| {
            let questions : String = row.get(1)?;
            let answers : String = row.get(2)?;
            let doc_id: i64 = row.get(3)?;
            let confidence : f64 = row.get(4)?;
            Ok(FlashCard::from_db(&questions, &answers, doc_id, confidence))
        })
        .collect()
    }
}
