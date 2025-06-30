use std::error::Error;
use std::path::PathBuf;
use csv::StringRecord;

pub fn read_as_headers_rows(path: PathBuf) -> Result<(StringRecord, Vec<StringRecord>) , Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = vec![];
    let headers = reader.headers()?.to_owned();
    for record in reader.records() {
        let record = record?;
        rows.push(record);
    }

    Ok((headers, rows))
}
pub fn to_rows_headers(resources_text_csv: String) -> Result<(StringRecord, Vec<StringRecord>), Box<dyn Error>> {
    let mut rows: Vec<StringRecord> = vec![];
    let mut reader = csv::Reader::from_reader(resources_text_csv.as_bytes());
    let headers = reader.headers()?.to_owned();
    for record in reader.records() {
        let record = record?;
        rows.push(record)
    }
    Ok((headers, rows))
}
