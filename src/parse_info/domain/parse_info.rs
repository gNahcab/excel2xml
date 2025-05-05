use std::collections::HashMap;
use std::path::PathBuf;
use crate::hcl_info::domain::parse_info_draft::ParseInformationDraft;
use crate::hcl_info::domain::xlsx_workbook_info::XLSXWorbookInfo;
use crate::hcl_info::transformations::Transformations;

pub struct ParseInformation {
    pub shortcode: String,
    pub rel_path_to_xlsx_workbooks:HashMap<String, XLSXWorbookInfo>,
    pub res_folder: PathBuf,
    pub separator: String,
    pub dm_path: PathBuf,
    pub set_permissions: bool,
    pub res_name_to_updates: HashMap<String, Transformations>
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
            res_name_to_updates: p_i_draft.res_name_to_updates
        }
    }
}


