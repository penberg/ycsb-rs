mod core_workload;

pub use core_workload::CoreWorkload;

use crate::db::DB;

pub trait Workload {
    fn do_insert(&self, db: &impl DB);
    fn do_transaction(&self, db: &impl DB);
}
