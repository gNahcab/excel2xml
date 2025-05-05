use std::collections::HashMap;
use std::vec;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_info::domain::xlsx_sheet_info::SheetInfo;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::header_value::HeaderValue;
use crate::parse_info::methods_domain::create_method::CreateMethod;
use crate::parse_xlsx::domain::data_column::DataColumn;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::intermediate_sheet::IntermediateSheet;
use crate::parse_xlsx::domain::manipulations::{perform_combine, perform_create, perform_lower, perform_replace, perform_to_date, perform_upper};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::special_propnames::SpecialPropnames;

#[derive(Clone)]
pub struct  {
    pub res_name: String,
    pub headers_xlsx: DataRow,
    pub headers_created: DataRow,
    pub data_rows: Vec<DataRow>,
    pub created_data_rows: Vec<DataRow>,
}


impl  {
    fn new(res_name: String, headers: DataRow, data_rows: Vec<DataRow>, created_headers: DataRow, created_data_rows: Vec<DataRow>) -> Self {
         {
            res_name,
            headers_xlsx: headers,
            headers_created: created_headers,
            data_rows,
            created_data_rows,
        }
    }
}

struct TransientDataSheet {
    res_name: String,
    headers: Option<DataRow>,
    data_rows: Vec<DataRow>,
}

impl TransientDataSheet {
    fn new(res_name: String) -> Self {
        TransientDataSheet{
            res_name,
            headers: None ,
            data_rows: vec![],
        }
    }
    pub(crate) fn add_headers(&mut self, headers: DataRow) {
        self.headers = Option::Some(headers);
    }
    pub(crate) fn add_row(&mut self, row: DataRow) {
        self.data_rows.push(row);
    }
    pub(crate) fn compare_headers_to_data_model(&self, data_model: &DataModel, special_propnames: &SpecialPropnames) -> Result<(), ExcelDataError> {
        let resource:&DMResource = match data_model.resources.iter().find(|resource|resource.name == self.res_name) {
            None => {return Err(ExcelDataError::ParsingError(format!("cannot find res-name '{}' in datamodel!", self.res_name)))}
            Some(resource) => {resource}
        };
        let headers: Vec<&String> = self.headers.as_ref().unwrap().row.iter().map(|header| header).collect();
        let propnames_not_found: Vec<&String> = resource.properties.iter().filter(|property|!headers.contains(&&property.propname)).map(|property|&property.propname).collect();
        if propnames_not_found.len() != 0 {
            return Err(ExcelDataError::ParsingError(format!("found propnames that don't exist in the headers of file of resource '{}'. Not existing propnames: '{:?}', header: '{:?}'", self.res_name, propnames_not_found, self.headers.as_ref().unwrap().row)))
        }
        let special_headers_not_found: Vec<&String> = special_propnames.resource_header.iter().filter(|special_res_header|headers.contains(special_res_header)).map(|special_res_header| special_res_header).collect();
        if special_headers_not_found.len() != 0 {
            return Err(ExcelDataError::ParsingError(format!("found propnames that don't exist in the headers of file of resource '{}'. Not existing propnames: '{:?}', header: '{:?}'", self.res_name, propnames_not_found, self.headers.as_ref().unwrap().row)))
        }
        // check for bitstream if resource will contain media-files
        todo!()
    }
}
pub(crate) struct DataSheetWrapper(pub(crate) IntermediateSheet);

impl DataSheetWrapper {
    pub(crate) fn to_data_sheet(&self, sheet_info: &SheetInfo) -> Result<, HCLDataError> {
        // this is where the changes requested in the parse-information file should be processed
        let (headers, old_rows) = &self.0.data_rows.split_at(1);
        let headers = &headers[0];
        let columns = rows_to_columns(old_rows);
        let (created_headers, created_columns) = create_data(&columns, headers, sheet_info)?;
        let rows = columns_to_rows(columns);
        let created_rows = columns_to_rows(created_columns);
        Ok(::new(sheet_info.resource_name.to_owned(), headers.to_owned(), rows, created_headers, created_rows))
    }
}

fn columns_to_rows(columns: Vec<DataColumn>) -> Vec<DataRow> {
    let mut data_rows = vec![];
    for (curr_pos, value_1) in columns.get(0).as_ref().unwrap().column.iter().enumerate() {
        let mut data_row = DataRow::new();
        data_row.add_data(value_1.to_owned());
        for col in columns.iter().skip(1) {
            data_row.add_data(col.column.get(curr_pos).unwrap().to_owned());
        }
        data_rows.push(data_row);
    }
    data_rows
}

fn rows_to_columns(old_rows: &&[DataRow]) -> Vec<DataColumn> {
    let mut columns: Vec<DataColumn> = vec![];
    // it is supposed that every row has the same length
    for _ in 0..old_rows[0].row.len() {
        let column =  DataColumn::new();
        &columns.push(column);
    }
    for data_row in old_rows.iter() {
        // open row
        for col_nr in 0..old_rows[0].row.len() {
            // add every column to the respective column vec
            columns[col_nr].column.push(data_row.row[col_nr].to_owned());
        }
    }
    columns
}

fn create_data(data_columns: &Vec<DataColumn>, header: &DataRow, sheet_info: &SheetInfo) -> Result<(DataRow, Vec<DataColumn>), HCLDataError> {
    if sheet_info.transformations.is_none() {
        return Ok((DataRow::new(), vec![]));
    }
    let mut additional_columns = vec![];
    let mut additional_headers = DataRow::new();
    let transformations = sheet_info.transformations.as_ref().unwrap();
    for replace_method in &transformations.replace_methods {
        perform_replace(replace_method, &mut additional_columns, &mut additional_headers, header, data_columns)?
    }
    for lower_method in &transformations.lower_methods {
        let data_col = perform_lower(lower_method, header, data_columns)?;
        additional_columns.push(data_col);
        additional_headers.row.push(lower_method.output.to_owned());

    }
    for upper_method in &transformations.upper_methods {
        let data_col = perform_upper(upper_method,  header, data_columns)?;
        additional_columns.push(data_col);
        additional_headers.row.push(upper_method.output.to_owned());

    }
    for combine_method in &transformations.combine_methods {
        let data_col = perform_combine(combine_method, header, data_columns)?;
        additional_columns.push(data_col);
        additional_headers.row.push(combine_method.output.to_owned());

    }
    for to_date_method in &transformations.to_date_methods {
        let data_col = perform_to_date(to_date_method, header, data_columns)?;
        additional_columns.push(data_col);
        additional_headers.row.push(to_date_method.output.to_owned());
    }
    for create_method in &transformations.create_methods {
        let data_col = perform_create(create_method, data_columns);
        additional_columns.push(data_col);
        additional_headers.row.push(create_method.output.to_owned());
    }
    Ok((additional_headers, additional_columns))
}


fn assign_headers(raw_headers: &DataRow, assignments: &HashMap<String, HeaderValue>) -> Result<DataRow, HCLDataError> {
    let mut old_string_to_new_string: HashMap<String, String> = HashMap::new();
    let mut pos_number_to_new_string: HashMap<u8, String> = HashMap::new();
    for (new_name, header) in assignments {
        match header {
            HeaderValue::Name(old_name) => {
                old_string_to_new_string.insert(old_name.to_owned(), new_name.to_owned());
            }
            HeaderValue::Number(pos_number) => {
                pos_number_to_new_string.insert(*pos_number, new_name.to_owned());
            }
        }
    }
    let mut data_row = DataRow::new();
    for (curr_pos, raw_header) in raw_headers.row.iter().enumerate() {
        if pos_number_to_new_string.get(&(curr_pos as u8)).is_some() {
            let value = pos_number_to_new_string.remove(&(curr_pos as u8)).unwrap();
            if old_string_to_new_string.get(raw_header).is_some() {
                return Err(HCLDataError::ParsingError(format!("'This position of the header was called two times in the hcl-transform-file; once as string '{}', once as integer '{}'. This results in a conflict.", raw_header, curr_pos)))
            }
            data_row.row.push(value.to_owned());
        } else if old_string_to_new_string.get(raw_header).is_some() {
            data_row.row.push(old_string_to_new_string.remove(raw_header).unwrap().to_owned());
        }
        else {
            data_row.row.push(raw_header.to_owned());
        }
    }
    Ok(data_row)
}

pub fn data_sheets(sheets: Vec<IntermediateSheet>, parse_info: &ParseInformation) -> Result<Vec<>, HCLDataError> {
    let mut data_sheets = vec![];
    for sheet in sheets.iter() {
        let sheet_info = parse_info.rel_path_to_xlsx_workbooks.get(&sheet.rel_path).unwrap().sheet_infos.get(&sheet.sheet_info_nr).unwrap();
        let data_sheet = DataSheetWrapper(sheet.to_owned()).to_data_sheet(sheet_info)?;
        data_sheets.push(data_sheet);
    }
    Ok(data_sheets)
}


