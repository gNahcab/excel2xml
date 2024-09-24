use hcl::{BlockLabel, Expression, Identifier};
use hcl::structure::iter::Attributes;
use crate::json2datamodel::domain::property::TransientProperty;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::wrapper::Wrapper;

pub struct XLSXWorbook {
    pub rel_path:String
}
pub(crate) struct XLSXWorkbookWrapper(pub(crate) hcl::Block);

struct TransientXLSXWorkbook {
    rel_path: String
}

impl TransientXLSXWorkbook {
    pub(crate) fn add_rel_path(&mut self, rel_path: String) {
        self.rel_path = rel_path;
    }
}

impl TransientXLSXWorkbook {
    fn new() -> Self {
        TransientXLSXWorkbook { rel_path: "".to_string()}
    }
}

impl XLSXWorkbookWrapper {
    pub(crate) fn to_xlsx_workbook(&self) -> Result<XLSXWorbook, HCLDataError> {
        let rel_path = self.0.get_single_label()?;
        let mut transient_xlsx_sheet_info = TransientXLSXWorkbook::new();
        transient_xlsx_sheet_info.add_rel_path(rel_path);
        let blocks: Vec<&hcl::Block> = self.0.blocks();
        self.0.no_attributes()?;
        for block in blocks {
            match block.identifier.as_str() {
                "sheet" => {
                    let sheet: SheetInfo = SheetInfoWrapper(block.to_owned()).to_sheet_info()?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("parse-info-hcl: only 'sheet' is allowed as block-identifier on first level of 'xlsx'-block. Wrong identifier: {}",block.identifier.as_str())));
                } }
        }
        todo!()
    }
}

fn no_attributes(attributes: Vec<hcl::Attribute>) -> Result<(), HCLDataError> {
    todo!()
}
