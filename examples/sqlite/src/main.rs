use anyhow::Result;
use itertools::join;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use ycsb::db::DB;

const PRIMARY_KEY: &str = "y_id";

#[derive(Clone)]
pub struct SQLite {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl SQLite {
    pub fn new(db_path: &Path) -> Result<Self> {
        let flags = rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
            | rusqlite::OpenFlags::SQLITE_OPEN_FULL_MUTEX
            | rusqlite::OpenFlags::SQLITE_OPEN_CREATE;

        rusqlite::Connection::open_with_flags(db_path, flags)
            .map(|c| SQLite {
                conn: Arc::new(Mutex::new(c)),
            })
            .map_err(|e| anyhow::anyhow!("failed to open db at `{:?}`: {}", db_path, e))
    }
}

impl DB for SQLite {
    fn init(&self) -> Result<()> {
        // TODO: generic on the workload
        let guard = self.conn.lock().unwrap();
        guard.execute(
            "create table if not exists usertable (
                y_id int primary key,
                field0 text,
                field1 text,
                field2 text,
                field3 text,
                field4 text,
                field5 text,
                field6 text,
                field7 text,
                field8 text,
                field9 text
            )",
            [],
        )?;
        Ok(())
    }

    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> Result<()> {
        let mut values_vec = vec![(PRIMARY_KEY.to_string(), key.to_string())];
        values_vec.append(
            &mut values
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>(),
        );
        let guard = self.conn.lock().unwrap();
        let mut stmt = guard.prepare(&format!(
            "insert or replace into {} ({}) values ({})",
            table,
            join(values_vec.iter().map(|(k, _)| k), ", "),
            join(values_vec.iter().map(|_| "?"), ", ")
        ))?;

        stmt.execute(rusqlite::params_from_iter(
            values_vec.into_iter().map(|(_, v)| v),
        ))?;
        Ok(())
    }

    fn read(&self, table: &str, key: &str, result: &mut HashMap<String, String>) -> Result<()> {
        // two paths:
        //  1. empty result map means read all
        //  2. result map with keys means read only those keys

        let guard = self.conn.lock().unwrap();
        if result.is_empty() {
            let mut stmt =
                guard.prepare(&format!("select * from {} where {}=?", table, PRIMARY_KEY))?;
            let mut mapping = HashMap::with_capacity(stmt.column_count());
            for i in 0..(stmt.column_count()) {
                mapping.insert(stmt.column_name(i)?.to_string(), i);
            }
            stmt.query_row(rusqlite::params![key], |row| {
                for (k, v) in result.iter_mut() {
                    let idx = mapping.get(k).unwrap();
                    *v = row.get(*idx)?;
                }
                Ok(())
            })?;
        } else {
            let keys = result.keys().cloned().collect::<Vec<String>>();
            let mut stmt = guard.prepare(&format!(
                "select {} from {} where {}=?",
                join(&keys, ", "),
                table,
                PRIMARY_KEY
            ))?;
            stmt.query_row(rusqlite::params![key], |row| {
                for (idx, key) in keys.iter().enumerate() {
                    if let Ok(val) = row.get(idx) {
                        result.insert((*key).clone(), val);
                    }
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let database = SQLite::new(Path::new("test.db"))?;
    ycsb::ycsb_main(database)
}
