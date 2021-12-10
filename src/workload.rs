mod core_workload;

pub use core_workload::CoreWorkload;

use crate::db::DB;
use std::rc::Rc;

pub trait Workload {
    fn do_insert(&self, db: Rc<dyn DB>);
    fn do_transaction(&self, db: Rc<dyn DB>);
}
