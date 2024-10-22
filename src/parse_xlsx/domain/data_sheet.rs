use std::collections::HashMap;
use std::vec;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_info::domain::xlsx_sheet_info::SheetInfo;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::intermediate_sheet::IntermediateSheet;
use crate::parse_xlsx::errors::ExcelDataError;
use crate::special_propnames::SpecialPropnames;

#[derive(Clone)]
pub struct DataSheet {
    pub res_name: String,
    pub headers: DataRow,
    pub data_rows: Vec<DataRow>,
}


impl DataSheet {
    fn new(res_name: String, headers: DataRow, data_rows: Vec<DataRow>) -> Self {
        DataSheet{
            res_name,
            headers,
            data_rows,
        }
    }
    pub(crate) fn gain_usize_to_propnames_from_headers(&self) -> Result<(), ExcelDataError> {
        for header in self.headers.rows.iter() {
            match header.to_lowercase().as_str() {
                "id" => {},
                "label" => {},
                "permissions" => {},
                "ark" => {},
                "iri" => {},
                "bitstream" => {},
                &_ => {

                } }
        }
        todo!()
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
        let headers: Vec<&String> = self.headers.as_ref().unwrap().rows.iter().map(|header| header).collect();
        let propnames_not_found: Vec<&String> = resource.properties.iter().filter(|property|!headers.contains(&&property.propname)).map(|property|&property.propname).collect();
        if propnames_not_found.len() != 0 {
            return Err(ExcelDataError::ParsingError(format!("found propnames that don't exist in the headers of file of resource '{}'. Not existing propnames: '{:?}', header: '{:?}'", self.res_name, propnames_not_found, self.headers.as_ref().unwrap().rows)))
        }
        let special_headers_not_found: Vec<&String> = special_propnames.resource_header.iter().filter(|special_res_header|headers.contains(special_res_header)).map(|special_res_header| special_res_header).collect();
        if special_headers_not_found.len() != 0 {
            return Err(ExcelDataError::ParsingError(format!("found propnames that don't exist in the headers of file of resource '{}'. Not existing propnames: '{:?}', header: '{:?}'", self.res_name, propnames_not_found, self.headers.as_ref().unwrap().rows)))
        }
        // check for bitstream if resource will contain media-files
        todo!()
    }
}
pub(crate) struct DataSheetWrapper(pub(crate) IntermediateSheet);

impl DataSheetWrapper {
    pub(crate) fn to_data_sheet(&self, sheet_info: &SheetInfo) -> Result<DataSheet, ExcelDataError> {
        // this is where the changes requested in the parse-information file should be processed
        let mut transient_data_sheet =
            TransientDataSheet::new(self.0.res_name.to_owned());
        let (old_headers, old_rows) = &self.0.data_rows.split_at(1);
        let manipulated_header_row: DataRow = manipulate_headers(old_headers.get(0).unwrap(), &sheet_info.assignments.header_to_propname);
        transient_data_sheet.add_headers(manipulated_header_row);
        //todo: change rows here
        old_rows.iter().for_each(|row|transient_data_sheet.add_row(row.to_owned()));
        Ok(DataSheet::new(transient_data_sheet.res_name, transient_data_sheet.headers.unwrap(), transient_data_sheet.data_rows))
    }
}


fn manipulate_headers(header: &DataRow, assignments: &HashMap<String, String>) -> DataRow {
    let mut new_header_row = DataRow::new();
    for old_header in header.rows.iter() {
        match assignments.get(old_header) {
            Some(new_header) => { new_header_row.add_data(new_header.to_owned())}
            None => {new_header_row.add_data(old_header.to_owned())}
        }
    }
    new_header_row
}
pub fn data_sheets(sheets: Vec<IntermediateSheet>, parse_info: &ParseInformation) -> Result<Vec<DataSheet>, ExcelDataError> {
    let mut data_sheets = vec![];
    for sheet in sheets.iter() {
        let sheet_info = parse_info.rel_path_to_xlsx_workbooks.get(&sheet.rel_path).unwrap().sheet_infos.get(&sheet.sheet_info_nr).unwrap();
        let data_sheet = DataSheetWrapper(sheet.to_owned()).to_data_sheet(sheet_info)?;
        data_sheets.push(data_sheet);
    }
    Ok(data_sheets)
}


