use crate::db::DB;
use crate::workload::Workload;
use anyhow::{bail, Result};
use properties::Properties;
use std::fs;
use structopt::StructOpt;
use workload::CoreWorkload;

pub mod db;
pub mod generator;
pub mod properties;
pub mod sqlite;
pub mod workload;

#[derive(StructOpt, Debug)]
#[structopt(name = "ycbs")]
struct Opt {
    #[structopt(name = "COMMANDS")]
    commands: Vec<String>,
    #[structopt(short, long)]
    database: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let raw_props = fs::read_to_string("workloads/workloada.toml")?;

    let props: Properties = toml::from_str(&raw_props)?;

    let mut wl = CoreWorkload::new(&props);

    let mut db = db::create_db(&opt.database)?;

    db.init()?;

    if opt.commands.is_empty() {
        bail!("no command specified");
    }

    for cmd in opt.commands {
        match &cmd[..] {
            "load" => {
                for _ in 0..props.operation_count {
                    wl.do_insert(&db);
                }
            }
            cmd => bail!("invalid command: {}", cmd),
        }
    }

    Ok(())
}
