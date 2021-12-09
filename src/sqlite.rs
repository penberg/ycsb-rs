use crate::db::DB;

use anyhow::Result;
use sql_builder::SqlBuilder;
use sqlite::{Connection, OpenFlags, State};
use std::collections::HashMap;

const PRIMARY_KEY: &str = "y_id";

#[derive(Default)]
pub struct SQLite {
    conn: Option<Connection>,
}

impl SQLite {
    pub fn new() -> Result<Self> {
        Ok(SQLite { conn: None })
    }
}

impl DB for SQLite {
    fn init(&mut self) -> Result<()> {
        let flags = OpenFlags::new().set_read_write().set_no_mutex();
        let conn = Connection::open_with_flags("test.db", flags)?;
        self.conn = Some(conn);
        Ok(())
    }

    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> Result<()> {
        // TODO: cache prepared statement
        let mut sql = SqlBuilder::insert_into(table);
        let mut vals: Vec<String> = Vec::new();
        sql.field(PRIMARY_KEY);
        vals.push(format!(":{}", PRIMARY_KEY));
        for key in values.keys() {
            sql.field(key);
            let marker = format!(":{}", key);
            vals.push(marker);
        }
        sql.values(&vals);
        let sql = sql.sql()?;
        let mut stmt = self.conn.as_ref().unwrap().prepare(sql)?;
        let marker = format!(":{}", PRIMARY_KEY);
        stmt.bind_by_name(&marker, key)?;
        for (key, value) in values {
            let marker = format!(":{}", key);
            stmt.bind_by_name(&marker, &value[..])?;
        }
        let state = stmt.next()?;
        assert!(state == State::Done);
        Ok(())
    }

    fn read(&self, table: &str, key: &str) -> Result<()> {
        // TODO: cache prepared statement
        let mut sql = SqlBuilder::select_from(table);
        sql.field("*");
        // TODO: fields
        sql.and_where(format!("{} = :{}", PRIMARY_KEY, PRIMARY_KEY));
        let sql = sql.sql()?;
        let mut stmt = self.conn.as_ref().unwrap().prepare(sql)?;
        let marker = format!(":{}", PRIMARY_KEY);
        stmt.bind_by_name(&marker, key)?;
        let state = stmt.next()?;
        assert!(state == State::Row);
        // TODO: results
        Ok(())
    }
}
