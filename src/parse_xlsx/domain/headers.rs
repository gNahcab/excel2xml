use std::collections::{HashMap, HashSet};
use crate::json2datamodel::domain::property::Property;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::header::{Extractor, Header, HeaderWrapper};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct Headers {
    pub(crate) pos_to_headers: HashMap<usize, Header>,
    pub propnames: HashMap<usize, String>,
}

impl Headers {
    fn new(pos_to_header: HashMap<usize, Header>, propnames: HashMap<usize, String>) -> Self {
        Headers{ pos_to_headers: pos_to_header, propnames}
    }
}

pub fn to_headers(raw_headers: &DataRow, properties: &Vec<Property>) -> Result<Headers, ExcelDataError> {
    let mut pos_to_headers: HashMap<usize, Header> = HashMap::new();
    let mut pos_to_propname: HashMap<usize, String> = HashMap::new();
    for (pos, raw_header) in raw_headers.rows.iter().enumerate() {
        let header = match HeaderWrapper(raw_header.to_owned()).to_header(&properties) {
            Ok(header) => {header}
            Err(_) => {
                //todo: for now ignore headers that don't exist, later require specifying this in hcl?
                continue
            }
        };
        if matches!(header, Header::ProjectProp(_)) {
            pos_to_propname.insert(pos, header.extract_value()?);}
        pos_to_headers.insert(pos, header);
        }
    no_duplicates(&pos_to_headers.values().collect())?;
    Ok(Headers::new(pos_to_headers, pos_to_propname))
}
fn no_duplicates(headers: &Vec<&Header>) -> Result<(), ExcelDataError> {
    let mut hash_set: HashSet<&Header> = HashSet::new();
    for header in headers {
        if !hash_set.insert(header) {
            return Err(ExcelDataError::InputError(format!("found duplicate in headers: {:?}", header)));
        }
    }
    Ok(())
}
