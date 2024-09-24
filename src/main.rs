mod path_dict;
mod write_steer_json;
mod write_xml;
mod read_json;
mod errors;
mod json2datamodel;
mod read_xlsx;
mod parse_xlsx;
mod parse_info;
mod read_hcl;

use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use parse_info::domain::parse_info::ParseInformation;
use crate::read_json::get_file::read_from_json;
use crate::read_xlsx::get_file::read_xlsx;
use crate::parse_xlsx::operations::process_worksheets_to_datasheets;
use crate::read_hcl::get_file::read_hcl_body;

fn main() {
    let xlsx_path = "/Users/gregorbachmann/Documents/DaSCH_projects/BIZ/240802/create_xml/final_results/CSVDocument.xlsx";
    let datamodel_path = "/Users/gregorbachmann/Documents/DaSCH_projects/BIZ/240802/240830_biz.json";
    let hcl_path = "resources/test/transform_xlsx.hcl";
    let parse_info = parse_information_file(hcl_path).unwrap();
    let file = read_from_json(datamodel_path).unwrap();
    let data_model: DataModel = file.try_into().unwrap();
    let worksheets = read_xlsx(xlsx_path).unwrap();


    //process_worksheets_to_datasheets(worksheets, data_model, xlsx_path, parse_info).unwrap();



}

fn parse_information_file(hcl_path: &str) -> Result<ParseInformation, Excel2XmlError> {
    let hcl_body:hcl::Body = read_hcl_body(hcl_path)?;
    let pars_info: ParseInformation = hcl_body.try_into()?;
    Ok(pars_info)
}
