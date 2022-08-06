use anyhow::Result;
use std::collections::HashMap;

pub trait DB: Send + Sync {
    fn init(&self) -> Result<()>;
    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> Result<()>;
    fn read(&self, table: &str, key: &str, result: &mut HashMap<String, String>) -> Result<()>;
}
