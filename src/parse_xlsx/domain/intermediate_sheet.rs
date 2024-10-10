use std::hash::Hash;
use crate::parse_xlsx::domain::data_row::{DataRow, DataRowWrapper};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::read_xlsx::sheet::Sheet;
#[derive(Clone)]
pub struct IntermediateSheet {
    pub(crate) res_name: String,
    pub rel_path: String,
    pub sheet_info_nr: usize,
    pub data_rows: Vec<DataRow>,
}

impl IntermediateSheet {
    fn new(res_name: String, rel_path: String, sheet_info_nr: usize) -> Self {
        IntermediateSheet {
            data_rows: vec![],
            res_name,
            rel_path,
            sheet_info_nr,
        }
    }
    pub(crate) fn add_row(&mut self, data_row: DataRow) {
        self.data_rows.push(data_row);
    }
}
pub(crate) struct IntermediateSheetWrapper(pub(crate) Sheet);

impl IntermediateSheetWrapper {
    pub(crate) fn to_intermediate_sheet(&self) -> Result<IntermediateSheet, ExcelDataError> {
        let mut data_sheet: IntermediateSheet = IntermediateSheet::new(self.0.res_name.to_owned(), self.0.rel_path.to_owned(), self.0.sheet_info_nr);
        if self.0.table.is_empty() {
            return Err(ExcelDataError::InputError("table cannot be empty".to_string()));
        }
        for row in self.0.table.rows() {
            let data_row: DataRow = DataRowWrapper(row.to_owned()).to_data_row();
            data_sheet.add_row(data_row);
        }
        Ok(data_sheet)
    }
}

pub fn intermediate_sheets(sheets: Vec<Sheet>) -> Result<Vec<IntermediateSheet>, ExcelDataError> {
    let mut data_sheets = vec![];
    for sheet in sheets.iter() {
        data_sheets.push(IntermediateSheetWrapper(sheet.to_owned()).to_intermediate_sheet()?);
    }
    Ok(data_sheets)
}
