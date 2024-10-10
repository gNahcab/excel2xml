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
use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use parse_info::domain::parse_info::ParseInformation;
use crate::read_json::get_file::read_from_json;
use crate::read_hcl::get_file::read_hcl_body;
use crate::read_xlsx::sheet::{sheets, Sheet};
use crate::special_propnames::SpecialPropnames;
use crate::extract::Extract;
use crate::parse_xlsx::domain::data_resource::{data_resources, DataResource};
use crate::parse_xlsx::domain::intermediate_sheet::{intermediate_sheets, IntermediateSheet};
use crate::parse_xlsx::domain::data_sheet::{data_sheets, compare_header_to_data_model, DataSheet};

fn main() {
    let folder_path = "/Users/gregorbachmann/Documents/DaSCH_projects/BIZ/240802/create_xml/final_results";
    let datamodel_path = "/Users/gregorbachmann/Documents/DaSCH_projects/BIZ/240802/240830_biz.json";
    let hcl_path = "resources/test/transform_xlsx.hcl";
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
    let data_resources: Vec<DataResource> = data_resources(&data_sheets, &data_model,&parse_info.separator, &special_propnames).unwrap();

}

fn parse_information_file(hcl_path: &str, data_model: &DataModel, special_propnames: &SpecialPropnames) -> Result<ParseInformation, Excel2XmlError> {
    let hcl_body:hcl::Body = read_hcl_body(hcl_path)?;
    let parse_info: ParseInformation = hcl_body.try_into()?;
    parse_info.correct_parse_info(&data_model, special_propnames).unwrap();
    Ok(parse_info)
}
