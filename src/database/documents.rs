use rand::Error;
use rusqlite::{Result, Row};
use sea_query::{Expr, Iden, Query, SqliteQueryBuilder};

use fallible_iterator::FallibleIterator;
sea_query::sea_query_driver_rusqlite!();
use crate::org::Document;
use sea_query_driver_rusqlite::RusqliteValues;

use super::Database;

pub enum Documents {
    Table,
    Id,
    Title,
    Content,
}

impl Iden for Documents {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Documents::Table => "documents",
                Documents::Id => "id",
                Documents::Title => "title",
                Documents::Content => "content",
            }
        )
        .unwrap();
    }
}

impl Database {
    pub fn insert_documents(&self, document: &str, title: &str) -> Result<i64> {
        let (sql, values) = Query::insert()
            .into_table(Documents::Table)
            .columns(vec![Documents::Content, Documents::Title])
            .values_panic(vec![document.into(), title.into()])
            .build(SqliteQueryBuilder);

        self.conn.execute(
            sql.as_str(),
            RusqliteValues::from(values).as_params().as_slice(),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_last_id(&self) -> i64 {
        let data: Result<i64> = self.conn.query_row(
            "SELECT id FROM documents ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        );
        data.expect("Cannot fetch the last id.")
    }

    pub fn load_data(&self, id: i64) -> Result<Vec<Document>> {
        let (sql, values) = Query::select()
            .columns(vec![Documents::Id, Documents::Title, Documents::Content])
            .from(Documents::Table)
            .and_where(Expr::col(Documents::Id).eq(id))
            .build(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(sql.as_str())?;
        let rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
        rows.map(|row| Ok(Document::from(row))).collect()
    }
}
