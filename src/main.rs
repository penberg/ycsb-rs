use std::fs;
use anyhow::Result;
use properties::Properties;
use std::collections::HashMap;
use structopt::StructOpt;
use workload::Workload;

pub mod db;
pub mod properties;
pub mod sqlite;
pub mod workload;

#[derive(StructOpt, Debug)]
#[structopt(name = "ycbs")]
struct Opt {
    #[structopt(name = "COMMANDS")]
    _commands: Vec<String>,
    #[structopt(short, long)]
    database: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let raw_props = fs::read_to_string("workloads/workloada.toml")?;

    let props: Properties = toml::from_str(&raw_props)?;

    let mut db = db::create_db(&opt.database)?;

    let _wl = Workload {};

    db.init()?;

    let mut fields = HashMap::new();
    fields.insert("field0", "bar");
    fields.insert("field1", "baz");
    fields.insert("field2", "zyzzy");
    db.insert("usertable", "foo", &fields)?;

    Ok(())
}
