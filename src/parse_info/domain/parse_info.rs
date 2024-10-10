use std::collections::{HashMap};
use std::num::ParseIntError;
use hcl::{Body, Expression};
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::resource::DMResource;
use crate::parse_info::domain::command::{ParseInfoCommandWrapper};
use crate::parse_info::domain::command_path::CommandOrPath;
use crate::parse_info::domain::xlsx_workbook::{XLSXWorbook, XLSXWorkbookWrapper};
use crate::parse_info::errors::HCLDataError;
use crate::special_propnames::SpecialPropnames;

pub struct ParseInformation {
    pub shortcode: String,
    pub rel_path_to_xlsx_workbooks:HashMap<String, XLSXWorbook>,
    pub res_folder: String,
    pub separator: String,
    pub dm_path: CommandOrPath,
}

impl ParseInformation {
    pub(crate) fn correct_parse_info(&self, data_model: &DataModel, special_propnames: &SpecialPropnames) -> Result<(), HCLDataError> {
        if self.shortcode != data_model.shortcode {
            return Err(HCLDataError::ParsingError(format!("Shortcode of Parse-Info and Datamodel don't match. Parse-info: {}, Datamodel: {}", self.shortcode, data_model.shortcode)));
        }

        for (_, xlsx_workbook) in self.rel_path_to_xlsx_workbooks.iter() {
            let dm_resources: Vec<&DMResource> = data_model.resources.iter().collect();
            let names: Vec<&String> = dm_resources.iter().map(|resource|&resource.name).collect();
            for (_ ,sheet_info) in xlsx_workbook.sheet_infos.iter() {
                if !names.contains(&&sheet_info.resource_name) {
                    return Err(HCLDataError::ParsingError(format!("cannot find resource-name '{}' in name of resources of datamodel: '{:?}'.",sheet_info.resource_name, names)));
                }
                let specific_dm_resource: &&&DMResource = dm_resources.iter().filter(|dmresource| dmresource.name == sheet_info.resource_name).collect::<Vec<&&DMResource>>().get(0).unwrap();

                let prop_names: Vec<String> = sheet_info.assignments.header_to_propname.iter().map(|(header, propname)| propname.to_lowercase()).collect();
                // 0. filter special propnames
                let prop_names: Vec<&String> = prop_names.iter().filter(|prop_name| !(special_propnames.resource_header.contains(prop_name) && special_propnames.bitstream.contains(prop_name) && special_propnames.properties.contains(prop_name))).collect();
                // 1. prop-names are part of properties


                // 2. prop_names should be part of the specific DMResource
            }
        }
        Ok(())
    }
}

impl ParseInformation {
    fn new(transient_parse_information: TransientParseInformation) -> Self {
        ParseInformation{
            shortcode: transient_parse_information.shortcode.unwrap(),
            rel_path_to_xlsx_workbooks: transient_parse_information.xlsx_workbooks,
            res_folder: transient_parse_information.res_folder.unwrap(),
            separator: transient_parse_information.separator.unwrap(),
            dm_path: transient_parse_information.dm_path.unwrap(),
        }
    }
}

impl TryFrom<hcl::Body> for ParseInformation {
    type Error = HCLDataError;
    fn try_from(body: Body) -> Result<Self, Self::Error> {
        let mut transient_parse_info = TransientParseInformation::new();
        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        for attribute in attributes.iter() {
            match attribute.key.as_str() {
                "shortcode" => {
                    match attribute.expr.to_owned() {
                        Expression::String(shortcode) => {
                            match shortcode.parse::<i32>(){
                                Err(err) => {
                                    HCLDataError::InputError(format!("cannot parse shortcode '{}' to usize", shortcode));
                                }
                                Ok(_) => {
                                    // do nothing
                                }
                            }
                            transient_parse_info.add_shortcode(shortcode)?;
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("parse-info-hcl: shortcode is not a Expression::Number: '{}'", attribute.expr)));
                        }
                    }
                }
                "resources_folder" => {
                    match attribute.expr.to_owned() {
                        Expression::String(res_folder) => {
                            transient_parse_info.add_res_folder(res_folder)?;
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("parse-info-hcl: \
                            resources_folder is not a Expression::String: '{}'", attribute.expr)));
                        }
                    }
                }
                "separator" => {
                    match attribute.expr.to_owned() {
                        Expression::String(separator) => {
                            transient_parse_info.add_separator(separator)?;
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("parse-info-hcl: \
                            separator is not a Expression::String: '{}'", attribute.expr)));
                        }
                    }
                }
                "datamodel" => {
                    let command_or_path = match attribute.expr.to_owned() {
                        Expression::String(path) => {
                            CommandOrPath::new_path(path)
                        }
                        Expression::Traversal(traversal) => {
                            let command = ParseInfoCommandWrapper(traversal).to_command()?;
                            CommandOrPath::new_command(command)
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("value of 'datamodel' must be a path of Expression::String or a command of Expression::Traversal, but found: '{:?}'", attribute.expr)));
                        }
                    };
                    transient_parse_info.add_command_or_path(command_or_path)?
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("unknown identifier in parse-info-hcl: {}", attribute.key)))
                }
            }
        }
        for block in blocks.iter() {
            match block.identifier.as_str() {
                "xlsx" => {
                    let xlsx_workbook: XLSXWorbook = XLSXWorkbookWrapper {0: block.to_owned().to_owned()}.to_xlsx_workbook()?;
                    transient_parse_info.add_xlsx_workbook(xlsx_workbook)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("unknown identifier in parse-info-hcl: {}", block.identifier))) }
            }
        }
        transient_parse_info.complete()?;
        Ok(ParseInformation::new(transient_parse_info))
    }
}

struct TransientParseInformation {
    shortcode: Option<String>,
    xlsx_workbooks: HashMap<String, XLSXWorbook>,
    res_folder: Option<String>,
    separator: Option<String>,
    dm_path: Option<CommandOrPath>,
}
impl TransientParseInformation {
    fn new() -> Self {
        TransientParseInformation { shortcode: None, xlsx_workbooks: Default::default(), res_folder: None, separator: None, dm_path: None}
    }
    pub(crate) fn add_shortcode(&mut self, shortcode: String) -> Result<(), HCLDataError> {
        if self.shortcode.is_some() {
            return Err(HCLDataError::InputError("parse-info-hcl: shortcode has a duplicate.".to_string()));
        }
        self.shortcode = Option::Some(shortcode);
        Ok(())
    }
    pub(crate) fn add_res_folder(&mut self, res_folder: String) -> Result<(), HCLDataError> {
        if self.res_folder.is_some() {
            return Err(HCLDataError::InputError(format!("res_folder with value '{}' has a duplicate.", res_folder)));
        }
        self.res_folder = Option::Some(res_folder);
        Ok(())
    }
    pub(crate) fn add_separator(&mut self, separator: String) -> Result<(), HCLDataError> {
        if self.separator.is_some() {
            return Err(HCLDataError::InputError("parse-info-hcl: separator has a duplicate.".to_string()));
        }
        self.separator = Option::Some(separator);
        Ok(())
    }
    pub(crate) fn add_command_or_path(&mut self, command_or_path: CommandOrPath) -> Result<(), HCLDataError> {
        if self.dm_path.is_some()  {
                    return Err(HCLDataError::InputError("parse-info-hcl: datamodel-path has a duplicate.".to_string()));
                }
        self.dm_path = Option::Some(command_or_path);
        Ok(())
    }
    pub(crate) fn add_xlsx_workbook(&mut self, xlsx_workbook: XLSXWorbook) -> Result<(), HCLDataError> {
        if self.xlsx_workbooks.contains_key(xlsx_workbook.rel_path.as_str()) {
            return Err(HCLDataError::InputError(format!("parse-info-hcl in files: found duplicate with same relative path for different xlsx-workbooks: {}", xlsx_workbook.rel_path)));
        }
        self.xlsx_workbooks.insert(xlsx_workbook.rel_path.to_owned(), xlsx_workbook);
        Ok(())
    }
    pub(crate) fn complete(&self) -> Result<(), HCLDataError> {
        if self.shortcode.is_none() {
            return Err(HCLDataError::InputError("'shortcode' not found.".to_string())) }
        if self.res_folder.is_none() {
            return Err(HCLDataError::InputError("'resource' folder not found.".to_string()))
        }
        if self.separator.is_none() {
            return Err(HCLDataError::InputError("'separator' not found.".to_string()))
        }
        if self.dm_path.is_none() {
            return Err(HCLDataError::InputError("'dm_path' not found.".to_string()))
        }
        Ok(())
    }

}