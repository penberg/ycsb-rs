mod core_workload;

pub use core_workload::CoreWorkload;

use crate::db::DB;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Workload {
    async fn do_insert(&self, db: Arc<dyn DB + Send + Sync>);
    async fn do_transaction(&self, db: Arc<dyn DB + Send + Sync>);
}
