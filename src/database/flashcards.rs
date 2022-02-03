use fallible_iterator::FallibleIterator;
use rusqlite::Result;
use sea_query::{Iden, Query, SqliteQueryBuilder};

sea_query::sea_query_driver_rusqlite!();
use sea_query_driver_rusqlite::RusqliteValues;

use crate::org::FlashCard;

use super::Database;

pub enum FlashCards {
    Table,
    Id,
    DocId,
    Questions,
    Answers,
    Confidence,
}

impl Iden for FlashCards {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                FlashCards::Table => "flashcards",

  FlashCards::Id => "id",
                FlashCards::DocId => "document",
                FlashCards::Questions => "questions",
                FlashCards::Answers => "answers",
                FlashCards::Confidence => "confidence",
            }
        )
        .unwrap();
    }
}

impl Database {
    pub fn insert_flashcards(&self, cards: &Vec<FlashCard>) -> Result<()> {
        for card in cards {
            let (sql, values) = Query::insert()
                .into_table(FlashCards::Table)
                .columns(vec![
                    FlashCards::Questions,
                    FlashCards::Answers,
                    FlashCards::DocId,
                    FlashCards::Confidence,
                ])
                .values_panic(vec![
                    card.get_questions().into(),
                    card.get_answers().into(),
                    card.get_id().into(),
                    card.get_confidence().into(),
                ])
                .build(SqliteQueryBuilder);

            self.conn.execute(
                sql.as_str(),
                RusqliteValues::from(values).as_params().as_slice(),
            )?;
        }
        Ok(())
    }

    pub fn _update_flashcards(&self, cards: &Vec<FlashCard>) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn get_flashcards(&self, num: i32) -> Result<Vec<FlashCard>> {
        let mut stmt = self.conn.prepare("SELECT * from flashcards LIMIT ?")?;
        let rows = stmt.query([&num.to_string()])?;
        rows.map(|row| {
            let questions: String = row.get(1)?;
            let answers: String = row.get(2)?;
            let doc_id: i64 = row.get(3)?;
            let confidence: f64 = row.get(4)?;
            Ok(FlashCard::from_db(&questions, &answers, doc_id, confidence))
        })
        .collect()
    }
}
