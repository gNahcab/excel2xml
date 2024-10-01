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

use std::error::Error;
use std::path::Path;
use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use parse_info::domain::parse_info::ParseInformation;
use crate::read_json::get_file::read_from_json;
use crate::read_xlsx::get_file::read_xlsx;
use crate::parse_xlsx::operations::process_worksheets_to_datasheets;
use crate::read_hcl::get_file::read_hcl_body;
use crate::read_xlsx::worksheet::Worksheet;
use crate::special_propnames::SpecialPropnames;
use crate::extract::Extract;

fn main() {
    let folder_path = "/Users/gregorbachmann/Documents/DaSCH_projects/BIZ/240802/create_xml/final_results";
    let datamodel_path = "/Users/gregorbachmann/Documents/DaSCH_projects/BIZ/240802/240830_biz.json";
    let hcl_path = "resources/test/transform_xlsx.hcl";
    let special_propnames: SpecialPropnames = SpecialPropnames::new();
    let file = read_from_json(datamodel_path).unwrap();
    let data_model: DataModel = file.try_into().unwrap();
    let parse_info = parse_information_file(hcl_path).unwrap();
    parse_info.correct_parse_info(&data_model, special_propnames).unwrap();

    let worksheets: Vec<Worksheet> = read_worksheets(folder_path, parse_info).unwrap();
    //let worksheets = read_xlsx(xlsx_path).unwrap();
    //process_worksheets_to_datasheets(worksheets, data_model, xlsx_path, parse_info).unwrap();



}

fn read_worksheets<P: AsRef<Path>>(folder_path: P, parse_information: ParseInformation) -> Result<Vec<Worksheet>, std::io::Error> {
    let parse_info_names: Vec<String> = parse_information.rel_path_to_xlsx_workbooks.iter().map(|(path, _)| path.extract_name())
        .collect();
    let paths = std::path::Path::read_dir(folder_path.as_ref())?;
    let filter_paths: Vec<String> = paths.map(|path| path.expect("cannot read path from directory")
        .path().to_str().expect("cannot read path to str").to_string())
        .filter(|path|parse_info_names.contains(path)).collect();





    todo!()
}

fn parse_information_file(hcl_path: &str) -> Result<ParseInformation, Excel2XmlError> {
    let hcl_body:hcl::Body = read_hcl_body(hcl_path)?;
    let parse_info: ParseInformation = hcl_body.try_into()?;
    Ok(parse_info)
}
