use crate::db::DB;
use crate::workload::Workload;
use anyhow::Result;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use workload::CoreWorkload;

pub mod db;
pub mod generator;
pub mod properties;
pub mod workload;

fn load<T: DB + Clone>(wl: Arc<CoreWorkload>, db: T, operation_count: usize) {
    for _ in 0..operation_count {
        wl.do_insert(db.clone());
    }
}

fn run<T: DB + Clone>(wl: Arc<CoreWorkload>, db: T, operation_count: usize) {
    for _ in 0..operation_count {
        wl.do_transaction(db.clone());
    }
}

pub fn ycsb_run<T: DB + Clone + 'static>(
    db: T,
    commands: Vec<String>,
    wl: Arc<CoreWorkload>,
    operation_count: usize,
    n_threads: usize,
) -> Result<()> {
    let thread_operation_count = operation_count as usize / n_threads;
    for cmd in commands {
        let start = Instant::now();
        let mut threads = vec![];
        for _ in 0..n_threads {
            let database = db.clone();
            let wl = wl.clone();
            let cmd = cmd.clone();
            threads.push(thread::spawn(move || {
                database.init().unwrap();

                match &cmd[..] {
                    "load" => load(wl.clone(), database, thread_operation_count),
                    "run" => run(wl.clone(), database, thread_operation_count),
                    cmd => panic!("invalid command: {}", cmd),
                };
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
