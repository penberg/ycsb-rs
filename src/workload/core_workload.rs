use crate::db::DB;
use crate::workload::Workload;
use rand::distributions::Uniform;
use rand::distributions::{Alphanumeric, DistString};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::collections::HashMap;

use crate::generator::{
    AcknowledgedCounterGenerator, ConstantGenerator, CounterGenerator, DiscreteGenerator,
    Generator, UniformLongGenerator, WeightPair, ZipfianGenerator,
};
use crate::properties::Properties;

#[derive(Copy, Clone, Debug)]
pub enum CoreOperation {
    Read,
    Update,
    Insert,
    Scan,
    ReadModifyWrite,
}

impl std::fmt::Display for CoreOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct CoreWorkload {
    rng: SmallRng,
    table: String,
    field_count: u64,
    field_names: Vec<String>,
    field_length_generator: Box<dyn Generator<u64>>,
    read_all_fields: bool,
    write_all_fields: bool,
    data_integrity: bool,
    key_sequence: Box<dyn Generator<u64>>,
    operation_chooser: DiscreteGenerator<CoreOperation>,
    //key_chooser: Box<dyn Generator<String>>,
    //field_chooser: Box<dyn Generator<String>>,
    transaction_insert_key_sequence: AcknowledgedCounterGenerator,
    //scan_length: Box<dyn Generator<u64>>,
    ordered_inserts: bool,
    record_count: usize,
    zero_padding: usize,
    insertion_retry_limit: u64,
    insertion_retry_interval: u64,
}

impl CoreWorkload {
    pub fn new(prop: &Properties) -> Self {
        let mut rng = SmallRng::from_entropy();
        let field_name_prefix = "field";
        let field_count = 10;
        let mut field_names = vec![];
        for i in 0..field_count {
            field_names.push(format!("{}{}", field_name_prefix, i));
        }
        CoreWorkload {
            rng,
            table: String::from("usertable"),
            field_count,
            field_names,
            field_length_generator: get_field_length_generator(prop),
            read_all_fields: true,
            write_all_fields: true,
            data_integrity: true,
            key_sequence: Box::new(CounterGenerator::new(1)),
            operation_chooser: create_operation_generator(prop),
            //key_chooser: Box<dyn Generator<String>>,
            //field_chooser: Box<dyn Generator<String>>,
            transaction_insert_key_sequence: AcknowledgedCounterGenerator::new(1),
            //scan_length: Box<dyn Generator<u64>>,
            ordered_inserts: true,
            record_count: 1,
            zero_padding: 1,
            insertion_retry_limit: 0,
            insertion_retry_interval: 0,
        }
    }
}

impl Workload for CoreWorkload {
    fn do_insert(&mut self, db: &impl DB) {
        let dbkey = self.key_sequence.next_value(&mut self.rng);
        let dbkey = format!("{}", fnvhash64(dbkey));
        let mut values = HashMap::new();
        for field_name in &self.field_names {
            let field_len = self.field_length_generator.next_value(&mut self.rng);
            let s = Alphanumeric.sample_string(&mut self.rng, field_len as usize);
            values.insert(&field_name[..], s);
        }
        db.insert(&self.table, &dbkey, &values).unwrap();
    }

    fn do_transaction(&self, db: &impl DB) {
        todo!("transaction");
    }
}

// http://en.wikipedia.org/wiki/Fowler_Noll_Vo_hash
fn fnvhash64(val: u64) -> u64 {
    let mut val = val;
    let prime = 0xcbf29ce484222325;
    let mut hashval = prime;
    for i in 0..8 {
        let octet = val & 0x00ff;
        val = val >> 8;
        hashval = hashval ^ octet;
        hashval = hashval.wrapping_mul(prime);
    }
    hashval
}

fn get_field_length_generator(prop: &Properties) -> Box<dyn Generator<u64>> {
    match prop.field_length_distribution.to_lowercase().as_str() {
        "constant" => Box::new(ConstantGenerator::new(prop.field_length)),
        "uniform" => Box::new(UniformLongGenerator::new(1, prop.field_length)),
        "zipfian" => Box::new(ZipfianGenerator::from_range(1, prop.field_length)),
        "histogram" => unimplemented!(),
        _ => panic!(
            "unknown field length distribution {}",
            prop.field_length_distribution
        ),
    }
}

fn create_operation_generator(prop: &Properties) -> DiscreteGenerator<CoreOperation> {
    let mut pairs = vec![];
    if prop.read_proportion > 0.0 {
        pairs.push(WeightPair::new(prop.read_proportion, CoreOperation::Read));
    }
    if prop.update_proportion > 0.0 {
        pairs.push(WeightPair::new(
            prop.update_proportion,
            CoreOperation::Update,
        ));
    }
    if prop.insert_proportion > 0.0 {
        pairs.push(WeightPair::new(
            prop.insert_proportion,
            CoreOperation::Insert,
        ));
    }
    if prop.scan_proportion > 0.0 {
        pairs.push(WeightPair::new(prop.scan_proportion, CoreOperation::Scan));
    }
    if prop.read_modify_write_proportion > 0.0 {
        pairs.push(WeightPair::new(
            prop.read_modify_write_proportion,
            CoreOperation::ReadModifyWrite,
        ));
    }

    DiscreteGenerator::new(pairs)
}
