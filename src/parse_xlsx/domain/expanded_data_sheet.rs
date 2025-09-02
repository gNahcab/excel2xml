use std::collections::HashMap;
use std::vec;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_hcl::domain::assignments::Assignments;
use crate::parse_hcl::domain::parse_info::ParseInformation;
use crate::parse_hcl::domain::xlsx_sheet_info::SheetInfo;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::HeaderValue;
use crate::parse_xlsx::domain::data_col::DataCol;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::intermediate_sheet::IntermediateSheet;
use crate::parse_xlsx::domain::manipulations::{perform_combine, perform_alter, perform_create, perform_lower, perform_replace, perform_to_date, perform_upper, perform_replace_label_name, perform_replace_with_iri, perform_separate};

#[derive(Clone)]
pub struct ExpandedDataSheet {
    pub res_name: String,
    pub col_nr_to_cols: HashMap<usize, DataCol>,
    pub header_to_col_nr: HashMap<String, usize>
}


impl ExpandedDataSheet {
    fn new(res_name: String, col_nr_to_cols: HashMap<usize, DataCol>, header_to_col_nr: HashMap<String, usize>) -> Self {
        ExpandedDataSheet {
            res_name,
            col_nr_to_cols,
            header_to_col_nr
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
    /*
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
     */
}

pub(crate) struct ExpandedDataSheetWrapper(pub(crate) IntermediateSheet);

impl ExpandedDataSheetWrapper {
    pub(crate) fn to_expanded_data_sheet(&self, sheet_info: &SheetInfo, data_model: &DataModel, res_name_iri: &HashMap<String, HashMap<String, String>>, separator: &String) -> Result<ExpandedDataSheet, HCLDataError> {
        // this is where the changes requested in the parse-information file should be processed
        let header_to_col_nr = header_to_col_nr(&sheet_info.assignments, &self.0.col_nr_to_data_cols)?;
        let (col_nr_to_cols, header_to_col_nr) = match sheet_info.transformations {
            None => {
                (self.0.col_nr_to_data_cols.to_owned(), header_to_col_nr)
            }
            Some(_) => {
                create_data(self.0.col_nr_to_data_cols.to_owned(), header_to_col_nr, sheet_info, &data_model, res_name_iri, separator)?
            }
        };
        Ok(ExpandedDataSheet::new(sheet_info.resource_name.to_owned(), col_nr_to_cols, header_to_col_nr))
    }
}

fn header_to_col_nr(assignments: &Assignments, col_nr_to_data_col: &HashMap<usize, DataCol>) -> Result<HashMap<String, usize>, HCLDataError> {
    // here we link all headers to their col-nr
    // don't replace or remove 'old' headers here, filter later
    let mut header_to_col_nr:HashMap<String, usize> = col_nr_to_data_col.iter().map(|(col_id, data_col)|(data_col.head.to_owned(), col_id.to_owned())).collect();

    for (new_header, old_header_token) in assignments.propname_to_header.iter() {
            match old_header_token {
                HeaderValue::Name(old_name) => {
                    match header_to_col_nr.get(old_name) {
                        None => {
                            return Err(HCLDataError::ParsingError(format!("Header '{}' does not exist in data. Existing headers: {:?}",old_name, header_to_col_nr.iter().map(|(header, _)| header.to_owned()).collect::<Vec<_>>())));
                        }
                        Some(number) => {
                            header_to_col_nr.insert(new_header.to_owned(), number.to_owned());
                        }
                    }
                }
                HeaderValue::Number(col_number) => {
                    let number = col_number.to_owned() as usize;
                    if col_nr_to_data_col.contains_key(&number) {
                            header_to_col_nr.insert(new_header.to_owned(), number.to_owned());
                    } else {
                            return Err(HCLDataError::ParsingError(format!("Number {} is not an index number in headers. Probably out of bounds, length of headers: {}", number, col_nr_to_data_col.len())));
                    }
                }
            }
        }

    Ok(header_to_col_nr)
}

fn create_data(mut col_nr_to_cols_expanded: HashMap<usize, DataCol>, mut header_to_col_nr_expanded: HashMap<String, usize>, sheet_info: &SheetInfo, data_model: &&DataModel, res_name_iri: &HashMap<String, HashMap<String, String>>, separator: &String) -> Result<(HashMap<usize, DataCol>, HashMap<String, usize>), HCLDataError> {
    let transformations = sheet_info.transformations.as_ref().unwrap();
    for replace_method in &transformations.replace_methods {
        let data_col = perform_replace(replace_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
    }
    for replace_with_iri in &transformations.update_with_server_methods {
        let data_col = perform_replace_with_iri(replace_with_iri, &col_nr_to_cols_expanded, &header_to_col_nr_expanded, res_name_iri, separator)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
    }
    for replace_label_name_method in &transformations.replace_label_name_methods {
        let data_col = perform_replace_label_name(replace_label_name_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded, data_model, separator)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
    }
    for lower_method in &transformations.lower_methods {
        let data_col = perform_lower(lower_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);

    }
    for upper_method in &transformations.upper_methods {
        let data_col = perform_upper(upper_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);

    }
    for combine_method in &transformations.combine_methods {
        let data_col = perform_combine(combine_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);

    }
    for to_date_method in &transformations.to_date_methods {
        let data_col = perform_to_date(to_date_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded, separator)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
    }
    // infer length for create method
    let mut length = 0usize;
    if !transformations.create_methods.is_empty() {
        length = match col_nr_to_cols_expanded.get(&0usize) {
            None => {
                return Err(HCLDataError::ParsingError("Create-methods: No other columns exist, so I cannot infer the length of the column that should be created.".to_string()))
            }
            Some(data_col) => {data_col.col.len()}
        };
    }
    for create_method in &transformations.create_methods {
        let data_col = perform_create(create_method, length);
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
    }
    for alter_method in &transformations.alter_methods {
        let data_col = perform_alter(alter_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded)?;
        add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
    }
    for separate_method in &transformations.separate_methods {
        let data_cols = perform_separate(separate_method, &col_nr_to_cols_expanded, &header_to_col_nr_expanded)?;
        for data_col in data_cols {
            add_to_header_cols(&mut header_to_col_nr_expanded, &mut col_nr_to_cols_expanded, data_col);
        }
    }
    Ok((col_nr_to_cols_expanded, header_to_col_nr_expanded))
}

pub fn add_to_header_cols(header_to_col_nr_expanded: &mut HashMap<String, usize>, col_nr_to_cols_expanded: &mut HashMap<usize, DataCol>, data_col_to_add: DataCol) {
    // the next id is the length of the col_nr_to_cols_expanded/col_nr_to_cols_expanded
    let id_ = col_nr_to_cols_expanded.len();
    header_to_col_nr_expanded.insert(data_col_to_add.head.to_owned(), id_);
    col_nr_to_cols_expanded.insert(id_, data_col_to_add);
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
    todo!()
    /*
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

     */
}

pub fn expanded_data_sheets(sheets: Vec<IntermediateSheet>, parse_info: &ParseInformation, data_model: &DataModel, res_name_iri: HashMap<String, HashMap<String, String>>, separator: &String) -> Result<Vec<ExpandedDataSheet>, HCLDataError> {
    let mut expanded_data_sheets = vec![];
    for sheet in sheets.iter() {
        let sheet_info = parse_info.rel_path_to_xlsx_workbooks.get(&sheet.rel_path).unwrap().sheet_infos.get(&sheet.sheet_info_nr).unwrap();
        let expanded_data_sheet = ExpandedDataSheetWrapper(sheet.to_owned()).to_expanded_data_sheet(sheet_info, data_model, &res_name_iri, separator)?;
        expanded_data_sheets.push(expanded_data_sheet);
    }
    Ok(expanded_data_sheets)
}


