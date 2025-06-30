use std::collections::{HashMap};
use std::path::PathBuf;
use hcl::{Body, Expression};
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_hcl::domain::command::{ParseInfoCommandWrapper};
use crate::parse_hcl::domain::command_path::CommandOrPath;
use crate::parse_hcl::domain::supplements::Supplements;
use crate::parse_hcl::domain::xlsx_workbook_info::{XLSXWorbookInfo, XLSXWorkbookInfoWrapper};
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::transformations::Transformations;

pub struct ParseInformationDraft {
    pub shortcode: String,
    pub rel_path_to_xlsx_workbooks:HashMap<String, XLSXWorbookInfo>,
    pub res_folder: PathBuf,
    pub separator: String,
    pub dm_path: CommandOrPath,
    pub set_permissions: bool,
    pub res_name_to_updates: HashMap<String, Transformations>,
    pub res_name_to_supplements: HashMap<String, Supplements>
}


impl ParseInformationDraft{
    fn new(transient_parse_information: TransientParseInformation) -> Self {
        ParseInformationDraft {
            shortcode: transient_parse_information.shortcode.unwrap(),
            rel_path_to_xlsx_workbooks: transient_parse_information.rel_path_to_xlsx_wb_info,
            res_folder: transient_parse_information.res_folder.unwrap(),
            separator: transient_parse_information.separator.unwrap(),
            dm_path: transient_parse_information.dm_path.unwrap(),
            set_permissions: transient_parse_information.permissions_set.unwrap(),
            res_name_to_updates: transient_parse_information.res_name_to_updates,
            res_name_to_supplements: transient_parse_information.res_name_to_supplements,
        }
    }
}

impl TryFrom<hcl::Body> for ParseInformationDraft {
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
                                Err(_) => {
                                    HCLDataError::ParseInt(format!("cannot parse shortcode '{}' to usize", shortcode));
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
                "set_permissions" => {
                    match attribute.expr.to_owned() {
                        Expression::Bool(set_permissions) => {
                            transient_parse_info.add_set_permissions(set_permissions)?
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("parse-info-hcl: \
                            set_permissions is not a Expression::Bool: '{}'", attribute.expr)));
                        }
                    }
                }
                "resources_folder_path" => {
                    match attribute.expr.to_owned() {
                        Expression::String(res_folder) => {
                            transient_parse_info.add_res_folder(PathBuf::from(res_folder))?;
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
                "datamodel_path" => {
                    let command_or_path = match attribute.expr.to_owned() {
                        Expression::String(path) => {
                            CommandOrPath::new_path(PathBuf::from(path))
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
                    let xlsx_workbook: XLSXWorbookInfo = XLSXWorkbookInfoWrapper {0: block.to_owned().to_owned()}.to_wb_info()?;

                    let res_name_to_suppl = xlsx_workbook.sheet_infos
                        .iter()
                        .filter(|(sheet_nr, sheet_info)| sheet_info.supplements.is_some())
                        .map(|(sheet_nr, sheet_info)| (sheet_info.resource_name.to_owned(), sheet_info.supplements.as_ref().unwrap().to_owned()))
                        .collect::<Vec<(String, Supplements)>>();
                    transient_parse_info.add_res_name_to_suppl(res_name_to_suppl)?;
                    transient_parse_info.add_xlsx_workbook(xlsx_workbook)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("unknown identifier in parse-info-hcl: {}", block.identifier))) }
            }
        }
        transient_parse_info.add_updates();
        transient_parse_info.complete()?;
        Ok(ParseInformationDraft::new(transient_parse_info))
    }
}

struct TransientParseInformation {
    shortcode: Option<String>,
    rel_path_to_xlsx_wb_info: HashMap<String, XLSXWorbookInfo>,
    res_folder: Option<PathBuf>,
    separator: Option<String>,
    dm_path: Option<CommandOrPath>,
    permissions_set: Option<bool>,
    res_name_to_updates:  HashMap<String, Transformations>,
    res_name_to_supplements: HashMap<String, Supplements>
}

impl TransientParseInformation {
}

impl TransientParseInformation {
    fn new() -> Self {
        TransientParseInformation { shortcode: None, rel_path_to_xlsx_wb_info: Default::default(), res_folder: None, separator: None, dm_path: None, permissions_set: None, res_name_to_updates: Default::default(), res_name_to_supplements: Default::default() }
    }
    pub(crate) fn add_res_name_to_suppl(&mut self, res_name_suppl: Vec<(String, Supplements)>) -> Result<(), HCLDataError> {
        for (res_name, supplements) in res_name_suppl {
            if self.res_name_to_supplements.contains_key(&res_name) {
                return Err(HCLDataError::InputError(format!("Found same res-name '{}' multiple times as key for 'res_name_to_suppl'", res_name)));
            }
            self.res_name_to_supplements.insert(res_name, supplements);

        }
        Ok(())
    }
    pub(crate) fn add_shortcode(&mut self, shortcode: String) -> Result<(), HCLDataError> {
        if self.shortcode.is_some() {
            return Err(HCLDataError::InputError("parse-info-hcl: shortcode has a duplicate.".to_string()));
        }
        self.shortcode = Option::Some(shortcode);
        Ok(())
    }
    pub(crate) fn add_res_folder(&mut self, res_folder: PathBuf) -> Result<(), HCLDataError> {
        if self.res_folder.is_some() {
            return Err(HCLDataError::InputError(format!("res_folder with value '{:?}' has a duplicate.", res_folder)));
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
    pub(crate) fn add_xlsx_workbook(&mut self, xlsx_workbook: XLSXWorbookInfo) -> Result<(), HCLDataError> {
        if self.rel_path_to_xlsx_wb_info.contains_key(xlsx_workbook.rel_path.as_str()) {
            return Err(HCLDataError::InputError(format!("parse-info-hcl in files: found duplicate with same relative path for different xlsx-workbooks: {}", xlsx_workbook.rel_path)));
        }
        self.rel_path_to_xlsx_wb_info.insert(xlsx_workbook.rel_path.to_owned(), xlsx_workbook);
        Ok(())
    }
    pub(crate) fn add_set_permissions(&mut self, permissions_set: bool) -> Result<(), HCLDataError> {
        if self.permissions_set.is_some() {
            return Err(HCLDataError::InputError(format!("parse-info-hcl in files: found duplicate permissions-set. First: {:?}, Second: {:?}", self.permissions_set, permissions_set)));
        }
        self.permissions_set = Some(permissions_set);
        Ok(())

    }
    pub(crate) fn complete(&self) -> Result<(), HCLDataError> {
        if self.shortcode.is_none() {
            return Err(HCLDataError::InputError("'shortcode' not found.".to_string())) }
        if self.permissions_set.is_none() {
            return Err(HCLDataError::InputError("'permissions-set' not found. Must be set to true or false.".to_string())) }
        /*
        if self.res_folder.is_none() {
            return Err(HCLDataError::InputError("'resource' folder not found.".to_string()))
        }
         */
        if self.separator.is_none() {
            return Err(HCLDataError::InputError("'separator' not found.".to_string()))
        }
        /*
        if self.dm_path.is_none() {
            return Err(HCLDataError::InputError("'dm_path' not found.".to_string()))
        }
        */
        Ok(())
    }
    pub(crate) fn add_updates(&mut self) {
        for (_, wb_info) in self.rel_path_to_xlsx_wb_info.iter() {
            for (_, sheet_info) in wb_info.sheet_infos.iter() {
                if sheet_info.transformations.is_some() {
                    self.res_name_to_updates.insert(sheet_info.resource_name.to_owned(), sheet_info.transformations.as_ref().unwrap().to_owned());
                }
            }
        }
    }

}