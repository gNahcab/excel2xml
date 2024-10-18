mod write_xml;
mod read_json;
mod errors;
mod json2datamodel;
mod read_xlsx;
mod parse_xlsx;
mod parse_info;
mod read_hcl;
mod special_propnames;
mod extract;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use parse_info::domain::parse_info::ParseInformation;
use crate::read_json::get_file::read_from_json;
use crate::read_hcl::get_file::read_hcl_body;
use crate::read_xlsx::sheet::{sheets, Sheet};
use crate::special_propnames::SpecialPropnames;
use crate::extract::Extract;
use crate::parse_xlsx::domain::data_container::{DataContainer, DataContainerWrapper};
use crate::parse_xlsx::domain::intermediate_sheet::{intermediate_sheets, IntermediateSheet};
use crate::parse_xlsx::domain::data_sheet::{data_sheets, DataSheet};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::write_xml::write_xml::write_xml;

fn main() {
    dotenv().ok();
    let datamodel_path = std::env::var("DATAMODEL_PATH").expect("DATAMODEL_PATH must be set.");
    let folder_path = std::env::var("FOLDER_PATH").expect("FOLDER_PATH must be set.");
    let hcl_path = std::env::var("PARSE_INFO_PATH").expect("PARSE_INFO_PATH must be set.");
    let hcl_path = fs::canonicalize(hcl_path).expect("unable to find absolute-path of parse-info");
    let folder_path = fs::canonicalize(folder_path).expect("unable to find absolute-path of folder-path");
    let datamodel_path = fs::canonicalize(datamodel_path).expect("unable to find absolute-path of datamodel");
    let special_propnames: SpecialPropnames = SpecialPropnames::new();
    let file = read_from_json(datamodel_path).unwrap();
    let data_model: DataModel = file.try_into().unwrap();
    let parse_info = parse_information_file(hcl_path, &data_model, &special_propnames).unwrap();

    //todo: refactor Sheet to DataResource with state pattern
    // import sheets
    let sheets: Vec<Sheet> = sheets(folder_path, &parse_info).unwrap();
    // prepare
    let intermediate_sheets: Vec<IntermediateSheet> = intermediate_sheets(sheets).unwrap();
    // edit
    let mut data_sheets:Vec<DataSheet> = data_sheets(intermediate_sheets, &parse_info).unwrap();
    // structure & review
    let data_containers: Vec<DataContainer> = data_containers(&data_sheets, &data_model,&parse_info.separator).unwrap();
    for  data_container in data_containers.iter() {
        write_xml(data_container, &data_model).unwrap();
    }
}


fn data_containers(data_sheet: &Vec<DataSheet>, data_model: &DataModel, separator: &String) -> Result<Vec<DataContainer>, ExcelDataError> {
    let mut data_containers = vec![];
    for sheet in data_sheet.iter() {
        data_containers.push(DataContainerWrapper(sheet.to_owned()).to_data_container(data_model, separator)?);
    }
    Ok(data_containers)
}


fn parse_information_file(hcl_path: PathBuf, data_model: &DataModel, special_propnames: &SpecialPropnames) -> Result<ParseInformation, Excel2XmlError> {
    let hcl_body:hcl::Body = read_hcl_body(hcl_path)?;
    let parse_info: ParseInformation = hcl_body.try_into()?;
    parse_info.correct_parse_info(&data_model, special_propnames)?;
    Ok(parse_info)
}
