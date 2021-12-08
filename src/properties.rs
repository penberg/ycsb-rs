use serde::Deserialize;

fn zero_u64() -> u64 {
    0
}

fn thread_count_default() -> u64 {
    200
}

fn field_length_distribution_default() -> String {
    "constant".to_string()
}

fn field_length_default() -> u64 {
    100
}

fn read_proportion_default() -> f64 {
    0.95
}

fn update_proportion_default() -> f64 {
    0.95
}

fn insert_proportion_default() -> f64 {
    0.0
}

fn scan_proportion_default() -> f64 {
    0.0
}

fn read_modify_write_proportion_default() -> f64 {
    0.0
}

#[derive(Deserialize, Debug)]
pub struct Properties {
    #[serde(default = "zero_u64", rename = "insertstart")]
    pub insert_start: u64,
    #[serde(default = "zero_u64", rename = "insertcount")]
    pub insert_count: u64,
    #[serde(rename = "operationcount")]
    pub operation_count: u64,
    #[serde(default = "zero_u64", rename = "record_count")]
    pub record_count: u64,
    #[serde(default = "thread_count_default", rename = "threacount")]
    pub thread_count: u64,
    #[serde(rename = "maxexecutiontime")]
    pub max_execution_time: Option<u64>,
    #[serde(rename = "warmuptime")]
    pub warmup_time: Option<u64>,
    // field length
    #[serde(
        default = "field_length_distribution_default",
        rename = "fieldlengthdistribution"
    )]
    pub field_length_distribution: String,
    #[serde(default = "field_length_default", rename = "fieldlength")]
    pub field_length: u64,

    // read, update, insert, scan, read-modify-write
    #[serde(default = "read_proportion_default", rename = "readproportion")]
    pub read_proportion: f64,
    #[serde(default = "update_proportion_default", rename = "updateproportion")]
    pub update_proportion: f64,
    #[serde(default = "insert_proportion_default", rename = "insertproportion")]
    pub insert_proportion: f64,
    #[serde(default = "scan_proportion_default", rename = "scanproportion")]
    pub scan_proportion: f64,
    #[serde(
        default = "read_modify_write_proportion_default",
        rename = "readmodifywriteproportion"
    )]
    pub read_modify_write_proportion: f64,
}
