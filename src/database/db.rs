use rusqlite::{Connection, Result};
use sea_query::{
    ColumnDef, ForeignKey, ForeignKeyAction, SqliteQueryBuilder,
    Table,
};

use super::flashcards::FlashCards;
use super::documents::Documents;
use super::files::Files;
sea_query::sea_query_driver_rusqlite!();

#[derive(Debug)]
pub struct Database {
    pub conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            conn: Connection::open("").unwrap(),
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
        let queries = [
            Table::create()
                .table(Files::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Files::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Files::Path).string().unique_key())
                .col(ColumnDef::new(Files::Name).string())
                .build(SqliteQueryBuilder),
            Table::create()
                .table(Documents::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Documents::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Documents::Title).string().unique_key())
                .col(ColumnDef::new(Documents::Content).string())
                .build(SqliteQueryBuilder),
            Table::create()
                .table(FlashCards::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(FlashCards::Id)
                        .integer()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(FlashCards::Questions).string().unique_key())
                .col(ColumnDef::new(FlashCards::Answers).string())
                .col(ColumnDef::new(FlashCards::DocId).integer())
                .col(ColumnDef::new(FlashCards::Confidence).float())
                .foreign_key(
                    ForeignKey::create()
                        .from(FlashCards::Table, FlashCards::DocId)
                        .to(Documents::Table, Documents::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .build(SqliteQueryBuilder),
        ]
        .join(";");
        conn.execute_batch(&queries)?;
        if let Ok(_) = conn.close() {
            println!("Connection closed")
        }
        Ok(())
    }
}
