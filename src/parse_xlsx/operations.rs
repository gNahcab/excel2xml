use std::collections::HashMap;
use calamine::{Data, Range};
use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::resource::DMResource;
use crate::xlsx2data::domain::data_sheet2::{DataSheet2, DataSheetWrapper};
use crate::xlsx2data::domain::data_resource::{DataResource, DataResourceWrapper};

#[derive(Clone)]
pub struct Worksheet {
    pub(crate) res_name: String,
    pub(crate) table: Range<Data>,
}

impl Worksheet {
    fn new(res_name: String, table: Range<Data>) -> Self {
        Worksheet{res_name, table}
    }
}

fn to_worksheets(xlsx_sheets: Vec<(String, Range<Data>)>, data_model: &DataModel, xlsx_path: &str) -> Result<Vec<Worksheet>, Excel2XmlError> {
    let mut worksheets: Vec<Worksheet> = vec![];
    // we already know that worksheets.len() cannot be 0 here, we checked that before
    if xlsx_sheets.len() == 1 {
        let position = xlsx_path.rfind("/").unwrap();
        let short_name = &xlsx_path[position..];
        let res_name = which_res_name(short_name, data_model.resources.iter().map(|dmresource: &DMResource| dmresource.name.as_str()).collect())?;
        let (_, table) = xlsx_sheets.get(0).unwrap().to_owned();
        let worksheet = Worksheet::new(res_name, table);
        worksheets.push(worksheet);
    } else {
        // > 1 worksheets
        for (name, table) in xlsx_sheets.iter() {
            let res_name = which_res_name(name, data_model.resources.iter().map(|dmresource: &DMResource| dmresource.name.as_str()).collect())?;
            let worksheet_wrapper = Worksheet::new(res_name, table.to_owned());
            worksheets.push(worksheet_wrapper);
        }

    }
    Ok(worksheets)
}
pub fn process_worksheets_to_datasheets(xlsx_sheets: Vec<(String, Range<Data>)>, data_model: DataModel, xlsx_path: &str, separator: String) -> Result<Vec<DataSheet2>, Excel2XmlError> {
    // first get res_name to table as worksheet
    let worksheets = to_worksheets(xlsx_sheets, &data_model, xlsx_path)?;

    // then transform worksheet to raw datasheet-resources
    let mut datasheets:Vec<DataSheet2> = vec![];
    for worksheet in worksheets.iter() {
        datasheets.push(DataSheetWrapper(worksheet.to_owned()).to_datasheet(&data_model)?);
    }
    /*
    // transform datasheet to dataclusters
    let mut dataclusters:Vec<DataCluster> = vec![];
    for datasheets in &datasheets {

    }

     */
    // parse raw dataclusters into trustful data resources
    let mut  resource_name_to_resources: HashMap<String, Vec<DataResource>> = Default::default();
    for datasheet in &datasheets {
        let mut data_resources: Vec<DataResource> = vec![];
        for raw_resource in datasheet.tabular_data.iter() {
            let data_resource  = DataResourceWrapper(raw_resource.to_owned()).to_data_resource(&data_model, separator.to_string())?;
            data_resources.push(data_resource);
        }
        resource_name_to_resources.insert(datasheet.resource_name.to_string(), data_resources);
    }
    // finally we turn the datasheet-resources into xml-code
    todo!()
}

fn which_res_name(inexact_name: &str, dm_res_names: Vec<&str>) -> Result<String, Excel2XmlError> {
    let candidates: Vec<&&str> = dm_res_names.iter().filter(|name| inexact_name.contains(name.to_owned())).collect();
    if candidates.len() == 1 {
        Ok(candidates[0].to_string())
    } else if candidates.len() == 0 {
        Err(Excel2XmlError::InputError(format!("not found a corresponding res name for xlsx-path/table-name: {}", inexact_name)))
    } else {
        Err(Excel2XmlError::InputError(format!("found multiple corresponding res names for xlsx-path/table-name: {}. Candidates were: {:#?}", inexact_name, candidates)))
    }
}