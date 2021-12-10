use crate::sqlite::SQLite;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait DB {
    fn init(&self) -> Result<()>;
    async fn insert(
        &self,
        table: String,
        key: String,
        values: HashMap<String, String>,
    ) -> Result<()>;
    async fn read(&self, table: String, key: String) -> Result<HashMap<String, String>>;
}

pub fn create_db(db: &str) -> Result<Arc<dyn DB + Send + Sync>> {
    match db {
        "sqlite" => Ok(Arc::new(SQLite::new()?)),
        db => Err(anyhow!("{} is an invalid database name", db)),
    }
}
