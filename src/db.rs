use crate::sqlite::SQLite;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::rc::Rc;

pub trait DB {
    fn init(&self) -> Result<()>;
    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> Result<()>;
    fn read(&self, table: &str, key: &str, result: &mut HashMap<String, String>) -> Result<()>;
}

pub fn create_db(db: &str) -> Result<Rc<dyn DB>> {
    match db {
        "sqlite" => Ok(Rc::new(SQLite::new()?)),
        db => Err(anyhow!("{} is an invalid database name", db)),
    }
}
