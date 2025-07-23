use std::collections::HashMap;
use std::hash::Hash;
use calamine::{Data, ExcelDateTime, Range};
use crate::parse_xlsx::domain::data_col::{DataCol};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::read_xlsx::sheet::Sheet;
#[derive(Clone)]
pub struct IntermediateSheet {
    pub(crate) res_name: String,
    pub rel_path: String,
    pub sheet_info_nr: usize,
    pub col_nr_to_data_cols: HashMap<usize, DataCol>,
}

impl IntermediateSheet {
    fn new(res_name: String, rel_path: String, sheet_info_nr: usize) -> Self {
        IntermediateSheet {
            res_name,
            rel_path,
            sheet_info_nr,
            col_nr_to_data_cols: Default::default(),
        }
    }
    pub(crate) fn add_col(&mut self, id_: usize, data_col: DataCol) {
        self.col_nr_to_data_cols.insert(id_, data_col);
    }
}
pub(crate) struct IntermediateSheetWrapper(pub(crate) Sheet);

impl IntermediateSheetWrapper {
    pub(crate) fn to_intermediate_sheet(&self) -> Result<IntermediateSheet, ExcelDataError> {
        let mut data_sheet: IntermediateSheet = IntermediateSheet::new(self.0.res_name.to_owned(), self.0.rel_path.to_owned(), self.0.sheet_info_nr);
        if self.0.table.is_empty() {
            return Err(ExcelDataError::InputError("table cannot be empty".to_string()));
        }
        // prepare cols
        let mut cols: Vec<Vec<String>> = vec![];
        for _ in 0..self.0.table.width() {
            &cols.push(vec![]);
        }
        for row in self.0.table.rows() {
            for (col_nr, value) in row.iter().enumerate() {
                let value: String = parse_data_to_string(value)?;
                cols[col_nr].push(value)
            }
        }
        for (col_id, col) in cols.iter().enumerate() {
            let (head, sliced_col) = col.split_at(1);
            let head = clean_string(&head[0]);
            let data_col: DataCol = DataCol::new(sliced_col.to_vec(), head);
            data_sheet.add_col(col_id, data_col);
        }
        Ok(data_sheet)
    }
}

fn clean_string(value: &String) -> String {
    // remove whitespace and \n (new line)
    value.trim().replace("\n", "")
}

pub fn parse_data_to_string(value: &Data) -> Result<String, ExcelDataError> {
    // parse data to string; adjust this according to needs later
    let value = match value {
        Data::Int(number) => {
            number.to_string()
        }
        Data::Float(number) => {
            number.to_string()}
        Data::String(string) => {
            string.to_owned()}
        Data::Bool(bool) => {
            bool.to_string()}
        Data::DateTime(date) => {
            match date.as_datetime(){
                None => {
                    return Err(ExcelDataError::ParsingError(format!("Cannot parse date as datetime: '{:?}'", date)))
                }
                Some(value) => {
                    value.date().to_string()
                }
            }}
        Data::DateTimeIso(date) => {
            date.to_owned()}
        Data::DurationIso(duration) => {
            duration.to_owned()}
        Data::Error(err) => {
            return Err(ExcelDataError::CellError(err.to_owned()));
        }
        Data::Empty => {
            "".to_string()
        }
    };
    Ok(value)
}


pub fn intermediate_sheets(sheets: Vec<Sheet>) -> Result<Vec<IntermediateSheet>, ExcelDataError> {
    let mut data_sheets = vec![];
    for sheet in sheets.iter() {
        data_sheets.push(IntermediateSheetWrapper(sheet.to_owned()).to_intermediate_sheet()?);
    }

    Ok(data_sheets)
}
