use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use serde_json::Value;
use crate::read_json::errors::ReadJsonError;

pub fn read_from_json<P: AsRef<Path>>(path: P) -> Result<Value, ReadJsonError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json_file: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    Ok(json_file)
}
