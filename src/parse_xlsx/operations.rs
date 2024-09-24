use std::collections::HashMap;
use calamine::{Data, Range};
use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::resource::DMResource;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_xlsx::domain::data_sheet::{DataSheet, DataSheetWrapper};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::read_xlsx::worksheet::Worksheet;

fn to_worksheets(xlsx_sheets: Vec<(String, Range<Data>)>, data_model: &DataModel, xlsx_path: &str) -> Result<Vec<Worksheet>, Excel2XmlError> {
    //todo!(use info from hcl-file: which files can be completely ignored, which ones not: separate worksheets : which sheets should be parsed, which can be ignored)
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

fn to_datasheets(worksheets: Vec<Worksheet>) -> Result<Vec<DataSheet>, ExcelDataError>{
    let mut datasheets:Vec<DataSheet> = vec![];
    for worksheet in worksheets.iter() {
        let datasheet = DataSheetWrapper(worksheet.to_owned()).to_datasheet()?;
        datasheets.push(datasheet)
    }
    Ok(datasheets)
}
pub fn process_worksheets_to_datasheets(xlsx_sheets: Vec<(String, Range<Data>)>, data_model: DataModel, xlsx_path: &str, parse_info:ParseInformation) -> Result<(), Excel2XmlError> {
    // first get res_name to table as worksheet
    let worksheets = to_worksheets(xlsx_sheets, &data_model, xlsx_path)?;
    let datasheets = to_datasheets(worksheets)?;
    Ok(())


    }

     /*
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
    */
    // finally we turn the datasheet-resources into xml-code

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