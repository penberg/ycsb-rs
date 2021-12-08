use rand::distributions::Uniform;

use crate::generator::{
    AcknowledgedCounterGenerator, ConstantGenerator, DiscreteGenerator, Generator,
    UniformLongGenerator, WeightPair, ZipfianGenerator,
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
    table: String,
    field_count: u64,
    field_names: Vec<String>,
    field_length_generator: Box<dyn Generator<String>>,
    read_all_fields: bool,
    write_all_fields: bool,
    data_integrity: bool,
    key_sequence: Box<dyn Generator<String>>,
    operation_chooser: DiscreteGenerator<CoreOperation>,
    key_chooser: Box<dyn Generator<String>>,
    field_chooser: Box<dyn Generator<String>>,
    transaction_insert_key_sequence: AcknowledgedCounterGenerator,
    scan_length: Box<dyn Generator<u64>>,
    ordered_inserts: bool,
    record_count: usize,
    zero_padding: usize,
    insertion_retry_limit: u64,
    insertion_retry_interval: u64,
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
