use crate::db::DB;
use crate::workload::Workload;
use anyhow::{bail, Result};
use properties::Properties;
use std::fs;
use std::time::{Duration, Instant};
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
    #[structopt(short, long)]
    workload: String,
}

fn load(props: &Properties, wl: &mut CoreWorkload, db: &impl DB) {
    for _ in 0..props.operation_count {
        wl.do_insert(db);
    }
}

fn run(props: &Properties, wl: &mut CoreWorkload, db: &impl DB) {
    for _ in 0..props.operation_count {
        wl.do_transaction(db);
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let raw_props = fs::read_to_string(&opt.workload)?;

    let props: Properties = toml::from_str(&raw_props)?;

    let mut wl = CoreWorkload::new(&props);

    let mut db = db::create_db(&opt.database)?;

    db.init()?;

    if opt.commands.is_empty() {
        bail!("no command specified");
    }

    for cmd in opt.commands {
        let f = match &cmd[..] {
            "load" => load,
            "run" => run,
            cmd => bail!("invalid command: {}", cmd),
        };
        let start = Instant::now();
        f(&props, &mut wl, &db);
        let runtime = start.elapsed().as_millis();
        println!("[OVERALL], RunTime(ms), {}", runtime);
        let throughput = props.operation_count as f64 / (runtime as f64 / 1000.0);
        println!("[OVERALL], Throughput(ops/sec), {}", throughput);
    }

    Ok(())
}
