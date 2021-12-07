use crate::sqlite::SQLite;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub trait DB {
    fn init(&mut self) -> Result<()>;
    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, &str>) -> Result<()>;
}

pub fn create_db(db: &str) -> Result<Box<dyn DB>> {
    match db {
        "sqlite" => Ok(Box::new(SQLite::new())),
        db => Err(anyhow!("{} is an invalid database name", db)),
    }
}
