use crate::db::DB;
use crate::workload::Workload;
use anyhow::{bail, Result};
use properties::Properties;
use std::fs;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
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
    #[structopt(short, long, default_value = "1")]
    threads: usize,
}

fn load(wl: Arc<CoreWorkload>, db: &impl DB, operation_count: usize) {
    for _ in 0..operation_count {
        wl.do_insert(db);
    }
}

fn run(wl: Arc<CoreWorkload>, db: &impl DB, operation_count: usize) {
    for _ in 0..operation_count {
        wl.do_transaction(db);
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let raw_props = fs::read_to_string(&opt.workload)?;

    let props: Properties = toml::from_str(&raw_props)?;

    let props = Arc::new(props);

    let wl = Arc::new(CoreWorkload::new(&props));

    if opt.commands.is_empty() {
        bail!("no command specified");
    }

    let database = opt.database.clone();
    let thread_operation_count = props.operation_count as usize / opt.threads;
    for cmd in opt.commands {
        let start = Instant::now();
        let mut threads = vec![];
        for _ in 0..opt.threads {
            let database = database.clone();
            let wl = wl.clone();
            let cmd = cmd.clone();
            threads.push(thread::spawn(move || {
                let mut db = db::create_db(&database).unwrap();

                db.init().unwrap();

                match &cmd[..] {
                    "load" => load(wl.clone(), &db, thread_operation_count as usize),
                    "run" => run(wl.clone(), &db, thread_operation_count as usize),
                    cmd => panic!("invalid command: {}", cmd),
                };
            }));
        }
        for t in threads {
            let _ = t.join();
        }
        let runtime = start.elapsed().as_millis();
        println!("[OVERALL], ThreadCount, {}", opt.threads);
        println!("[OVERALL], RunTime(ms), {}", runtime);
        let throughput = props.operation_count as f64 / (runtime as f64 / 1000.0);
        println!("[OVERALL], Throughput(ops/sec), {}", throughput);
    }

    Ok(())
}
