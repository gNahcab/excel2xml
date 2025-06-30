use std::collections::HashMap;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::methods_domain::identify_method::IdentifyMethod;
use crate::parse_hcl::transformations::Transformations;
use crate::parse_xlsx::domain::data_col::DataCol;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::expanded_data_sheet::{add_to_header_cols, ExpandedDataSheet};
use crate::parse_xlsx::domain::manipulations::perform_identify;

#[derive(Clone)]
pub struct UpdatedDataSheet {
    pub res_name: String,
    pub col_nr_to_cols: HashMap<usize, DataCol>,
    pub header_to_col_nr: HashMap<String, usize>,
}

impl UpdatedDataSheet {
    fn new(col_nr_to_cols: HashMap<usize, DataCol>, header_to_col_nr: HashMap<String, usize>, res_name: String) -> Self {
        UpdatedDataSheet{
            res_name,
            col_nr_to_cols,
            header_to_col_nr,
        }
    }
}

pub(crate) struct UpdatedDataSheetWrapper(pub(crate) ExpandedDataSheet);

impl UpdatedDataSheetWrapper {
    pub(crate) fn to_updated_data_sheet(&self, expanded_data_sheets: &Vec<ExpandedDataSheet>, res_name_to_updates: &HashMap<String, Transformations>) -> Result<UpdatedDataSheet, HCLDataError> {
        // update sheet
        let (col_nr_to_cols, headers_to_col_nr) = match res_name_to_updates.get(self.0.res_name.as_str()) {
            None => {
                // do nothing
                (self.0.col_nr_to_cols.to_owned(), self.0.header_to_col_nr.to_owned())
            }
            Some(transformations) => {
                update_sheet(self.0.col_nr_to_cols.to_owned(), self.0.header_to_col_nr.to_owned(), expanded_data_sheets, transformations)?
            }
        };
        let updated_sheet = UpdatedDataSheet::new(col_nr_to_cols, headers_to_col_nr, self.0.res_name.to_owned());
        Ok(updated_sheet)
    }
}

fn update_sheet(mut col_nr_to_col: HashMap<usize, DataCol>, mut header_to_col_nr: HashMap<String, usize>, expanded_data_sheets: &Vec<ExpandedDataSheet>, transformations: &Transformations) -> Result<(HashMap<usize, DataCol>, HashMap<String, usize>), HCLDataError> {
    // for now only identify method uses multiple resources
    for identify_method in transformations.identify_methods.iter() {
        let expanded_sheet = get_correct_expanded_sheet(expanded_data_sheets, &identify_method.resource_name)?;
        let data_col:DataCol = identify_col(expanded_sheet, identify_method, &col_nr_to_col, &header_to_col_nr)?;
        add_to_header_cols(&mut header_to_col_nr, &mut col_nr_to_col, data_col);
    }
    Ok((col_nr_to_col, header_to_col_nr))
}

fn _base_col_nr(curr_headers: & DataRow, header: & String) -> Result<usize, HCLDataError>{
    for (pos, curr_header) in curr_headers.row.iter().enumerate() {
        if curr_header == header {
            return Ok(pos)
        }
    }
    Err(HCLDataError::ParsingError(format!("Identify-methods: cannot find output-variable '{:?}' in headers '{:?}'", header, curr_headers.row)))
}
fn _key_value_pos(expanded_data_sheet: &ExpandedDataSheet, identify_method: &&IdentifyMethod) -> Result<(usize, usize), HCLDataError>{
    let mut key_pos: Option<usize> = None;
    let mut value_pos: Option<usize> = None;

    for (header, col_nr) in expanded_data_sheet.header_to_col_nr.iter(){
        if header.eq(&identify_method.key) {
            key_pos = Option::from(col_nr.to_owned());
        }
        if header.eq(&identify_method.value) {
            value_pos = Option::from(col_nr.to_owned());
        }
    }
    if key_pos.is_some() && value_pos.is_some() {
        return Ok((key_pos.unwrap(), value_pos.unwrap()))
    }
    return Err(HCLDataError::ParsingError(format!("Identity-Method: Cannot find both key & value for '{}'. Key: {:?}, value: {:?}", identify_method.resource_name, key_pos, value_pos)));
}
fn _key_value_column(expanded_data_sheet: &ExpandedDataSheet, identify_method:  &&IdentifyMethod) -> Result<(Vec<String>, Vec<String>), HCLDataError> {
    let (key_pos, value_pos) = _key_value_pos(expanded_data_sheet, identify_method)?;
    let key_col = expanded_data_sheet.col_nr_to_cols.get(&key_pos).unwrap().col.to_owned();
    let value_col = expanded_data_sheet.col_nr_to_cols.get(&value_pos).unwrap().col.to_owned();
    Ok((key_col, value_col))
}

fn _key_to_value_map(expanded_data_sheet: &ExpandedDataSheet, identify_method:  &&IdentifyMethod) -> Result<HashMap<String, String>, HCLDataError> {
    let mut key_to_value = HashMap::new();
    let (key_col, value_col) = _key_value_column(expanded_data_sheet, identify_method)?;
    for (pos, key) in key_col.iter().enumerate() {
        key_to_value.insert(key.to_owned(), value_col.get(pos).unwrap().to_owned());
    }
    Ok(key_to_value)
}
fn identify_col(other_expaneded_sheet: &ExpandedDataSheet, identify_method: &IdentifyMethod, col_nr_to_cols: &HashMap<usize, DataCol>, header_to_col_nr: &HashMap<String, usize>) -> Result<DataCol, HCLDataError> {
    let key_to_value = _key_to_value_map(other_expaneded_sheet, &identify_method)?;
    let base_pos = match header_to_col_nr.get(identify_method.input.as_str()) {
        None => {
            return Err(HCLDataError::ParsingError(format!("Identify: cannot find input header: '{}'. Possible headers: {:?}", identify_method.input, header_to_col_nr.iter().map(|(header, _)|header).collect::<Vec<_>>())))
        }
        Some(number) => {number}
    };
    let base_col = &col_nr_to_cols.get(base_pos).unwrap().col;
    let new_col = perform_identify(key_to_value, base_col);
    println!("new-identify: {:?}", new_col);
    Ok(DataCol::new(new_col, identify_method.output.to_owned()))
}

fn get_correct_expanded_sheet<'a>(expanded_data_sheets: &'a Vec<ExpandedDataSheet>, res_name: &'a String) -> Result<&'a ExpandedDataSheet, HCLDataError> {
    for expanded in expanded_data_sheets.iter() {
        if expanded.res_name.eq(res_name) {
            return Ok(expanded)
        }
    }
    Err(HCLDataError::ParsingError(format!("Identify-methods: Cannot find expanded-data-sheet with res-name: '{}'", res_name)))
}


