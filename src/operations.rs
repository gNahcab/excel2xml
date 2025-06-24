use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde_json::Value;
use crate::parse_dm::domain::data_model::DataModel;
use crate::errors::Excel2XmlError;
use crate::parse_dm::errors::DataModelError;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_info::domain::parse_info_draft::ParseInformationDraft;
use crate::parse_xlsx::domain::data_container::{DataContainer, DataContainerWrapper};
use crate::parse_xlsx::domain::expanded_data_sheet::{expanded_data_sheets, ExpandedDataSheet};
use crate::parse_xlsx::domain::intermediate_sheet::{intermediate_sheets, IntermediateSheet};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::transformations::Transformations;
use crate::parse_xlsx::domain::updated_data_sheet::{UpdatedDataSheet, UpdatedDataSheetWrapper};
use crate::path_operations::path_operations::{canonicalize_paths, filter_paths_based_on_extension};
use crate::read_hcl::get_file::read_hcl_body;
use crate::read_json::get_file::read_from_json;
use crate::read_xlsx::get_file::read_xlsx;
use crate::read_xlsx::sheet::{sheets, Sheet};
use crate::write_hcl::write_hcl::write_hcl;
use crate::write_xml::write_xml::write_xml;

pub fn write_hcl_default(folder_path: &PathBuf) {
    let dm_paths = filter_paths_based_on_extension(folder_path, "json").unwrap();
    let dm_path = if dm_paths.len() == 1 {
        dm_paths.get(0).unwrap()
    } else {
        panic!("Found {} datamodel-paths, but should find exactly one. datamodel-paths: {:?}", dm_paths.len(), dm_paths);
    };
    let file = read_from_json(dm_path).unwrap();
    let datamodel = load_data_model(file).unwrap();
    let xlsx_paths = filter_paths_based_on_extension(folder_path, "xlsx").unwrap();
    let mut file_name_table_name_table_headers = vec![];
    for path in xlsx_paths {
        file_name_table_name_table_headers.push(extract_file_name_table_name_header(&path));
    }
    write_hcl(file_name_table_name_table_headers, datamodel).unwrap();
}

fn extract_file_name_table_name_header(path: &PathBuf) -> (String, String, Vec<String>) {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let xlsx_tables = read_xlsx(&path).unwrap();
        // attention: only first table is taken
        let (table_name, table) = match xlsx_tables.get(0) {
            None => {panic!("Did not find tables in {:?}", path)}
            Some(name_and_table) => {name_and_table}
        };
        let header_rows = table.rows().take(0);
        let header_rows: Vec<String> = header_rows.last().unwrap().to_owned().iter().map(|value|value.to_string()).collect();
        (file_name, table_name.to_owned(), header_rows)
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
    let data_model = load_data_model(file).unwrap();
    parse_info.compare_parse_info_to_datamodel(&data_model).unwrap();

    //&parse_info.compare_parse_info_to_datamodel(&data_model, special_propnames)?;

    // import sheets
    let sheets: Vec<Sheet> = sheets(&parse_info.res_folder, &parse_info).unwrap();
    // prepare
    let intermediate_sheets: Vec<IntermediateSheet> = intermediate_sheets(sheets).unwrap();
    // edit
    let expanded_data_sheets:Vec<ExpandedDataSheet> = expanded_data_sheets(intermediate_sheets, &parse_info, &data_model).unwrap();
    let updated_data_sheets: Vec<UpdatedDataSheet> = updated_data_sheets(expanded_data_sheets, &parse_info.res_name_to_updates).unwrap();
    // structure & review
    let mut data_containers: Vec<DataContainer> = data_containers(&updated_data_sheets, &data_model, &parse_info).unwrap();
    for  data_container in data_containers.iter() {
        write_xml(&data_container, &data_model, &parse_info).unwrap();
    }
}

fn load_data_model(file: Value) -> Result<DataModel, DataModelError> {
    file.try_into()
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
    let (res_folder, dm_path) = canonicalize_paths(&hcl_info_draft.dm_path, &hcl_info_draft.res_folder, hcl_path)?;
    let parse_info: ParseInformation = ParseInformation::new(hcl_info_draft, dm_path, res_folder);
    Ok(parse_info)
}
