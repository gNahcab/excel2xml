use std::collections::HashMap;
use std::path::PathBuf;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_info::domain::parse_info_draft::ParseInformationDraft;
use crate::parse_info::domain::supplements::Supplements;
use crate::parse_info::domain::xlsx_workbook_info::XLSXWorbookInfo;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::transformations::Transformations;
use crate::special_propnames::SpecialPropnames;

pub struct ParseInformation {
    pub shortcode: String,
    pub rel_path_to_xlsx_workbooks:HashMap<String, XLSXWorbookInfo>,
    pub res_folder: PathBuf,
    pub separator: String,
    pub dm_path: PathBuf,
    pub set_permissions: bool,
    pub res_name_to_updates: HashMap<String, Transformations>,
    pub res_name_to_supplements: HashMap<String, Supplements>
}

impl ParseInformation {
    pub(crate) fn new(p_i_draft: ParseInformationDraft, dm_path: PathBuf, data_folder: PathBuf) -> ParseInformation {
        ParseInformation{
            shortcode: p_i_draft.shortcode,
            rel_path_to_xlsx_workbooks: p_i_draft.rel_path_to_xlsx_workbooks,
            res_folder: data_folder,
            dm_path,
            separator: p_i_draft.separator,
            set_permissions: p_i_draft.set_permissions,
            res_name_to_updates: p_i_draft.res_name_to_updates,
            res_name_to_supplements: p_i_draft.res_name_to_supplements,
        }
    }
    pub(crate) fn compare_parse_info_to_datamodel(&self, data_model: &DataModel) -> Result<(), HCLDataError> {
        // todo extend this
        // check for shortcode
        if self.shortcode != data_model.shortcode {
            return Err(HCLDataError::ParsingError(format!("Shortcode of Parse-Info and Datamodel don't match. Parse-info: {}, Datamodel: {}", self.shortcode, data_model.shortcode)));
        }
        Ok(())
    }
}


