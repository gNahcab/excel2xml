use std::collections::HashMap;
use hcl::{Body, Identifier};
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::errors::HCLDataError::InputError;
use crate::parse_info::position::Position;
use crate::parse_xlsx::errors::ExcelDataError::ParsingError;

pub struct ParseInformation {
    xlsx_paths_to_read: Vec<String>,
    path_json_to_datamodel: String,
    xlsx_which_sheet_nr: HashMap<String, usize>,
    xlsx_which_assignments: HashMap<String, HashMap<Position, String>>
}
impl TryFrom<hcl::Body> for ParseInformation {
    type Error = HCLDataError;
    fn try_from(body: Body) -> Result<Self, Self::Error> {
        let mut transient_transform_hcl = TransientTransformHCL::new();
        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        for attribute in attributes.iter() {
            match attribute.key.as_str() {
                "shortcode" => {}
                "resources_folder" => {}
                "separator" => {}
                "datamodel" => {}
                "xlsx" => {}
                _ => {
                    return Err(InputError(""))
                }
            }

        }
        todo!()
    }
}

struct TransientTransformHCL {

}

impl TransientTransformHCL {
    fn new() -> Self {
        TransientTransformHCL{}
    }

}