use std::path::Path;

use fallible_iterator::FallibleIterator;
use rusqlite::Result;
use sea_query::{Iden, Query, SqliteQueryBuilder};

sea_query::sea_query_driver_rusqlite!();
use super::Database;
use crate::files::File;
use sea_query_driver_rusqlite::RusqliteValues;

pub enum Files {
    Id,
    Table,
    Name,
    Path,
}

impl Iden for Files {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Files::Id => "id",
                Files::Table => "files",
                Files::Name => "name",
                Files::Path => "path",
            }
        )
        .unwrap();
    }
}

impl Database {
    pub fn insert_file(&self, file: &Path) -> Result<()> {
        let mut data = File::new();
        data.load_path(file);
        let (sql, values) = Query::insert()
            .into_table(Files::Table)
            .columns(vec![Files::Name, Files::Path])
            .values_panic(vec![data.get_name().into(), data.get_path().into()])
            .build(SqliteQueryBuilder);

        self.conn.execute(
            sql.as_str(),
            RusqliteValues::from(values).as_params().as_slice(),
        )?;
        Ok(())
    }

    pub fn load_file_names(&self) -> Result<Vec<File>> {
        let (sql, values) = Query::select()
            .columns(vec![
                Files::Id,
                Files::Path,
                Files::Name,
            ])
            .from(Files::Table)
            .build(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(sql.as_str())?;
        let rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
        rows.map(|row| {
            let item = File::from(row);
            println!("{:?}", &item);
            Ok(item)
        })
        .collect()
    }
}
