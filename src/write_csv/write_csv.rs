use std::error::Error;
use std::path::PathBuf;
use csv::{StringRecord, Writer};

pub fn write_csv(rows: &Vec<StringRecord>, headers: &StringRecord, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(&path)?;
    wtr.write_record(headers)?;
    for row in rows.iter() {
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    println!("wrote: {:?}", path);
    Ok(())
}