use std::hash::Hash;
use crate::parse_xlsx::domain::data_row::{DataRow, DataRowWrapper};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::read_xlsx::worksheet::Worksheet;
pub struct DataSheet {
    data_rows: Vec<DataRow>,
}

impl DataSheet {
    fn new() -> Self {
        DataSheet { data_rows: vec![] }
    }
    pub(crate) fn add_row(&mut self, data_row: DataRow) {
        self.data_rows.push(data_row);
    }
}
pub(crate) struct DataSheetWrapper(pub(crate) Worksheet);

impl DataSheetWrapper {
    pub(crate) fn to_datasheet(&self) -> Result<DataSheet, ExcelDataError> {
        let mut data_sheet: DataSheet = DataSheet::new();
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
