use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::parse_dm::domain::data_model::DataModel;
use crate::errors::Excel2XmlError;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_info::domain::parse_info_draft::ParseInformationDraft;
use crate::parse_xlsx::domain::data_container::{DataContainer, DataContainerWrapper};
use crate::parse_xlsx::domain::expanded_data_sheet::{expanded_data_sheets, ExpandedDataSheet};
use crate::parse_xlsx::domain::intermediate_sheet::{intermediate_sheets, IntermediateSheet};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::canonicalize_path;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::transformations::Transformations;
use crate::parse_xlsx::domain::updated_data_sheet::{UpdatedDataSheet, UpdatedDataSheetWrapper};
use crate::read_hcl::get_file::read_hcl_body;
use crate::read_json::get_file::read_from_json;
use crate::read_xlsx::sheet::{sheets, Sheet};
use crate::write_hcl::write_hcl::write_hcl_based_on_xlsx;
use crate::write_xml::write_xml::write_xml;

pub fn write_hcl(excel_path: &PathBuf) {
    write_hcl_based_on_xlsx(excel_path).unwrap();
}

pub fn excel2xml(hcl_path: &PathBuf) {
    // canonicalize paths
    let hcl_path = fs::canonicalize(hcl_path).expect("unable to find absolute-path of parse-info");
    let parse_info: ParseInformation = parse_hcl_info(hcl_path).unwrap();

    /*
    let folder_path = fs::canonicalize(folder_path).expect("unable to find absolute-path of folder-path");
    let datamodel_path = fs::canonicalize(datamodel_path).expect("unable to find absolute-path of datamodel");
    */

    // prepare
    // todo: data-models should be loaded from resources/data_models, not from path specified in hcl (download from server, if data-model is not there or replace if update is indicated)
    let file = read_from_json(&parse_info.dm_path).unwrap();
    let data_model: DataModel = file.try_into().unwrap();

    //&parse_info.compare_parse_info_to_datamodel(&data_model, special_propnames)?;

    // import sheets
    let sheets: Vec<Sheet> = sheets(&parse_info.res_folder, &parse_info).unwrap();
    // prepare
    let intermediate_sheets: Vec<IntermediateSheet> = intermediate_sheets(sheets).unwrap();
    // edit
    let expanded_data_sheets:Vec<ExpandedDataSheet> = expanded_data_sheets(intermediate_sheets, &parse_info).unwrap();
    let updated_data_sheets: Vec<UpdatedDataSheet> = updated_data_sheets(expanded_data_sheets, &parse_info.res_name_to_updates).unwrap();
    // structure & review
    let mut data_containers: Vec<DataContainer> = data_containers(&updated_data_sheets, &data_model, &parse_info).unwrap();
    for  data_container in data_containers.iter() {
        write_xml(&data_container, &data_model, &parse_info).unwrap();
    }
}

fn updated_data_sheets(expanded_data_sheets: Vec<ExpandedDataSheet>, res_name_to_updates: &HashMap<String, Transformations>) -> Result<Vec<UpdatedDataSheet>, HCLDataError> {
    let mut updated_data_sheets = vec![];
    for expanded_data_sheet in expanded_data_sheets.iter() {
        let updated_data_sheet = UpdatedDataSheetWrapper(expanded_data_sheet.to_owned()).to_updated_data_sheet(&expanded_data_sheets, res_name_to_updates)?;
        updated_data_sheets.push(updated_data_sheet);
    }
    Ok(updated_data_sheets)
}


fn data_containers(data_sheet: &Vec<UpdatedDataSheet>, data_model: &DataModel, parse_info: &ParseInformation) -> Result<Vec<DataContainer>, ExcelDataError> {
    let mut data_containers = vec![];
    for updated_sheet in data_sheet.iter() {
        data_containers.push(DataContainerWrapper(updated_sheet.to_owned()).to_data_container(data_model, parse_info)?);
    }
    Ok(data_containers)
}


fn parse_hcl_info(hcl_path: PathBuf) -> Result<ParseInformation , Excel2XmlError> {
    let hcl_body:hcl::Body = read_hcl_body(&hcl_path)?;
    let mut hcl_info_draft: ParseInformationDraft = hcl_body.try_into()?;
    let (res_folder, dm_path) = canonicalize_path::canonicalize_paths(&hcl_info_draft.dm_path, &hcl_info_draft.res_folder, hcl_path)?;
    let parse_info: ParseInformation = ParseInformation::new(hcl_info_draft, dm_path, res_folder);
    Ok(parse_info)
}
