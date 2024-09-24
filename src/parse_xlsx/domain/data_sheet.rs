use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use serde_json::to_string;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::resource::DMResource;
use crate::json2datamodel::domain::res_property::ResProperty;
use crate::xlsx2data::domain::data_row::{DataRow, DataRowWrapper};
use crate::xlsx2data::domain::prop_name::PropName;
use crate::xlsx2data::domain::raw_resource_data::{RawResourceData, ResourceDataWrapper};
use crate::xlsx2data::errors::ExcelDataError;
use crate::xlsx2data::worksheet_wrapper::Worksheet;

#[derive(Clone)]
pub struct DataSheet {
    pub resource_name: String,
    pub tabular_data: Vec<Vec<String>>,
    pub headers: Vec<String>,
    pub assignments: HashMap<String, Vec<String>>,
}
impl DataSheet {
    fn new(resource_name: String, tabular_data: Vec<Vec<String>>, headers: Vec<String>, assignments: HashMap<String, Vec<String>>) -> Self {
        DataSheet { resource_name, tabular_data, headers, assignments}
    }
}
struct TransientDataSheet{

}
impl TransientDataSheet {
    fn new() -> Self {
        TransientDataSheet{}
    }
}
pub(crate) struct DataSheetWrapper(pub(crate) Worksheet);

impl DataSheetWrapper {
    pub(crate) fn to_datasheet(&self, parse_info:ParseInformation) -> Result<DataSheet, ExcelDataError> {
        let transient_data_sheet: TransientDataSheet = TransientDataSheet::new();
        if self.0.table.is_empty() {
            return Err(ExcelDataError::InputError("table cannot be empty".to_string()));
        }
        let mut skip = 0;
        if parse_info.headers_exist() {
            match self.0.table.headers() {
                None => {
                    return Err(ExcelDataError::ParsingError("headers should not be empty according to parse information."))
                }
                Some(headers) => {
                    transient_data_sheet.add_headers(headers);
                    skip = 1;
                }
            }
        }

        for row in self.0.table.rows().skip(skip) {
            let data_row: DataRow = DataRowWrapper(row.to_owned()).to_data_row()?;
            transient_data_sheet.add_row(data_row);
        }
        Ok(DataSheet2::new(self.0.res_name.to_string(), resources))
    }

}
fn correct_order(headers: Vec<String>, data_model: &DataModel) -> Result<(), ExcelDataError> {
    let special_prop_dict = HashMap::from([
        ("id".to_string(), PropName::ID),
        ("label".to_string(), PropName::LABEL),
        ("iri".to_string(), PropName::IRI),
        ("ark".to_string(), PropName::ARK),
        ("permissions".to_string(), PropName::PERMISSIONS),
        ("bitstream".to_string(), PropName::BITSTREAM),
    ]);
    resource_props_correct(headers)?;


    todo!()

}

fn resource_props_correct(headers: Vec<String>) -> Result<(), ExcelDataError> {
    // check if label, id, permissions, iri, ark, bitstream are correctly ordered and separated from other properties
    bitstream_correct(headers)?;
    todo!()
}

fn bitstream_correct(p0: Vec<String>) -> Result<(), ExcelDataError> {
    todo!()
}

fn no_duplicates(headers: Vec<String>) -> Result<(), ExcelDataError> {
    let repeating_headers = [
        "id".to_string(),
        "label".to_string(),
        "permissions".to_string(),
        "encoding".to_string(),
        "comment".to_string(),
    ];
    let headers:Vec<String> = headers.iter().filter(|header| !repeating_headers.contains(header)).map(|header|header.to_string()).collect();
    let value = return_first_duplicate(headers.clone());
    if value.is_some() {
        // it is only an error if duplicate
        return Err(ExcelDataError::InputError(format!("headers contains duplicate: {}", value.unwrap())));
    }
    let value = return_first_duplicate(headers.iter().map(|name| name.to_lowercase()).collect());
    if value.is_some() {
        return Err(ExcelDataError::InputError(format!("headers in lowercase contains duplicate: {}", value.unwrap())));
    }
    Ok(())
}
fn return_first_duplicate(headers: Vec<String>) -> Option<String>
{
    let mut uniq: HashSet<String> = HashSet::new();
    for value in headers.into_iter() {
        if !uniq.insert(value.to_string()) {
            return Some(value);
        }
    }
    None
}

fn get_pos_to_headers_propname(headers: Vec<String>, propnames: &Vec<&str>) -> Result<HashMap<usize, (String, PropName)>, ExcelDataError> {
    let special_prop_dict = HashMap::from([
        ("id".to_string(), PropName::ID),
        ("label".to_string(), PropName::LABEL),
        ("iri".to_string(), PropName::IRI),
        ("ark".to_string(), PropName::ARK),
        ("permissions".to_string(), PropName::PERMISSIONS),
        ("bitstream".to_string(), PropName::BITSTREAM),
    ]);
    let mut propname_lower_to_propname:HashMap<String, String> = HashMap::new();
    for propname in propnames {
        propname_lower_to_propname.insert(propname.to_lowercase(), propname.to_string());
    }

    let mut header_to_prop:HashMap<usize, (String, PropName)> = HashMap::new();
    for (pos, header) in headers.iter().enumerate() {
        let propname = propname_lower_to_propname.get(&header.to_lowercase());
        if propname.is_some() {
            header_to_prop.insert(pos, (header.to_owned(), PropName::ProjectProp(propname.unwrap().to_string())));
        } else {
            match special_prop_dict.get(&header.to_lowercase()) {
                None => {
                    //return Err(ExcelDataError::InputError(format!("unknown header '{}'. Not found in properties of resource or special props.", header)));
                    println!("no prop-name for header: {} . ignore...", header);
                }
                Some(prop_name_enum) => {
                    header_to_prop.insert(pos, (header.to_owned(), prop_name_enum.to_owned()));
                }
            }
        }
    }

    Ok(header_to_prop)
}
