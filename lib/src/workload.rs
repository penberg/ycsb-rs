mod core_workload;

pub use core_workload::CoreWorkload;

use crate::db::DB;

pub trait Workload {
    fn do_insert<T: DB>(&self, db: &T);
    fn do_transaction<T: DB>(&self, db: &T);
}
