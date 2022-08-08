use crate::db::DB;
use crate::workload::Workload;
use anyhow::{bail, Result};
use std::fs;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use workload::CoreWorkload;

use properties::Properties;
use structopt::StructOpt;

pub mod db;
pub mod generator;
pub mod properties;
pub mod workload;

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

fn load<T: DB>(wl: &CoreWorkload, db: &T, repeat: usize) {
    for _ in 0..repeat {
        wl.do_insert(db);
    }
}

fn run<T: DB>(wl: &CoreWorkload, db: &T, repeat: usize) {
    for _ in 0..repeat {
        wl.do_transaction(db);
    }
}

fn thread_runner<T: DB>(db: Arc<T>, wl: Arc<CoreWorkload>, repeat: usize, cmd: &str) {
    match cmd {
        "load" => load(wl.as_ref(), db.as_ref(), repeat),
        "run" => run(wl.as_ref(), db.as_ref(), repeat),
        _ => panic!("invalid command: {}", cmd),
    };
}

pub fn ycsb_run<T: DB + 'static>(
    db: T,
    commands: Vec<String>,
    wl: CoreWorkload,
    operation_count: usize,
    n_threads: usize,
) -> Result<()> {
    db.init()?;
    let db = Arc::new(db);
    let wl = Arc::new(wl);
    for cmd in commands {
        let start = Instant::now();
        let mut threads = vec![];
        for _ in 0..n_threads {
            let db_ref = db.clone();
            let wl_ref = wl.clone();
            let cmd_str = cmd.clone();
            threads.push(thread::spawn(move || {
                thread_runner(db_ref, wl_ref, operation_count / n_threads, &cmd_str)
            }));
        }
        for t in threads {
            let _ = t.join();
        }
        let runtime = start.elapsed().as_millis();
        println!("[OVERALL], ThreadCount, {}", n_threads);
        println!("[OVERALL], RunTime(ms), {}", runtime);
        let throughput = operation_count as f64 / (runtime as f64 / 1000.0);
        println!("[OVERALL], Throughput(ops/sec), {}", throughput);
    }

    Ok(())
}

pub fn ycsb_main<T: DB + 'static>(db: T) -> Result<()> {
    let opt = Opt::from_args();

    let raw_props = fs::read_to_string(&opt.workload)?;

    let props: Properties = toml::from_str(&raw_props)?;

    let props = Arc::new(props);

    let wl = CoreWorkload::new(&props);

    let commands = opt.commands;

    if commands.is_empty() {
        bail!("no command specified");
    }

    let n_threads = opt.threads;
    let operation_count = props.operation_count as usize;
    ycsb_run(db, commands, wl, operation_count, n_threads)
}
