use crate::sqlite::SQLite;
use anyhow::{bail, Result};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use structopt::StructOpt;

use ycsb::properties::Properties;
use ycsb::workload::CoreWorkload;

mod sqlite;

#[derive(StructOpt, Debug)]
#[structopt(name = "ycsb")]
struct Opt {
    #[structopt(name = "COMMANDS")]
    commands: Vec<String>,
    #[structopt(short, long)]
    workload: String,
    #[structopt(short, long, default_value = "1")]
    threads: usize,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let raw_props = fs::read_to_string(&opt.workload)?;

    let props: Properties = toml::from_str(&raw_props)?;

    let props = Arc::new(props);

    let wl = Arc::new(CoreWorkload::new(&props));

    let commands = opt.commands;

    if commands.is_empty() {
        bail!("no command specified");
    }

    let database = SQLite::new(Path::new("test.db"))?;
    let n_threads = opt.threads;
    let operation_count = props.operation_count as usize;
    ycsb::ycsb_run(database, commands, wl, operation_count, n_threads)
}

