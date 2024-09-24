use std::collections::{HashMap, HashSet};
use crate::parse_info::domain::xlsx_sheet_info::{SheetInfo, SheetInfoWrapper};
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::wrapper_trait::Wrapper;

pub struct XLSXWorbook {
    pub rel_path:String,
    pub sheet_infos: HashMap<usize, SheetInfo>,
}

impl XLSXWorbook {
        fn new(transient_xlsxworkbook: TransientXLSXWorkbook) -> Self{
            XLSXWorbook{ rel_path: transient_xlsxworkbook.rel_path, sheet_infos: transient_xlsxworkbook.sheet_infos}
    }
}

pub(crate) struct XLSXWorkbookWrapper(pub(crate) hcl::Block);

struct TransientXLSXWorkbook {
    rel_path: String,
    sheet_infos: HashMap<usize, SheetInfo>,
}

impl TransientXLSXWorkbook {
}

impl TransientXLSXWorkbook {
    fn new() -> Self {
        TransientXLSXWorkbook { rel_path: "".to_string(), sheet_infos: Default::default() }
    }
    pub(crate) fn add_rel_path(&mut self, rel_path: String) {
        self.rel_path = rel_path;
    }
    pub(crate) fn add_sheet_info(&mut self, sheet_info: SheetInfo) -> Result<(), HCLDataError> {
        if self.sheet_infos.contains_key(&sheet_info.sheet_nr) {
            return Err(HCLDataError::InputError(format!("Found")));
        }
        self.sheet_infos.insert(sheet_info.sheet_nr, sheet_info);
        Ok(())

    }
    pub(crate) fn no_duplicate_res_names(&self) -> Result<(), HCLDataError> {
        let mut uniq: HashSet<String> = HashSet::new();
        for (_, sheet_info) in self.sheet_infos.iter() {
            if !uniq.insert(sheet_info.resource_name.to_string()) {
                return Err(HCLDataError::InputError(format!("Duplicate resource-names '{}' found in sheet-infos of xlsx '{}'", sheet_info.resource_name, self.rel_path)));
            }
        }
        Ok(())
    }
}


impl XLSXWorkbookWrapper {
    pub(crate) fn to_xlsx_workbook(&self) -> Result<XLSXWorbook, HCLDataError> {
        let rel_path = self.0.get_single_label()?;
        let mut transient_xlsx_workbook = TransientXLSXWorkbook::new();
        transient_xlsx_workbook.add_rel_path(rel_path);
        let blocks: Vec<&hcl::Block> = self.0.blocks();
        self.0.no_attributes()?;
        for block in blocks {
            match block.identifier.as_str() {
                "sheet" => {
                    let sheet: SheetInfo = SheetInfoWrapper(block.to_owned()).to_sheet_info()?;
                    transient_xlsx_workbook.add_sheet_info(sheet)?;
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("parse-info-hcl: only 'sheet' is allowed as block-identifier on first level of 'xlsx'-block. Wrong identifier: {}",block.identifier.as_str())));
                } }
        }
        transient_xlsx_workbook.no_duplicate_res_names()?;
       Ok(XLSXWorbook::new(transient_xlsx_workbook))
    }
}

