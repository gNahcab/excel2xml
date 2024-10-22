use std::path::Path;
use calamine::{Data, Range};
use crate::errors::Excel2XmlError;
use crate::extract::Extract;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::read_xlsx::get_file::read_xlsx;

#[derive(Clone)]
pub struct Sheet {
    pub(crate) res_name: String,
    pub(crate) rel_path: String,
    pub(crate) sheet_info_nr: usize,
    pub(crate) table: Range<Data>,
}

impl Sheet {
    pub fn new(res_name: String, rel_path: String,sheet_info_nr: usize, table: Range<Data>) -> Self {
        Sheet {res_name, rel_path, sheet_info_nr, table}
    }
}
pub fn sheets<P: AsRef<Path>>(folder_path: P, parse_information: &ParseInformation) -> Result<Vec<Sheet>, Excel2XmlError> {
    let paths = std::path::Path::read_dir(folder_path.as_ref())?;
    let mut sheets: Vec<Sheet> = vec![];
    for entry in paths {
        let path: String = match &&entry?.path().to_str(){
            None => {
                return Err(Excel2XmlError::InputError("cannot read path to str".to_string()));
            }
            Some(path) => {path.to_string()}
        };
        let workbook = match parse_information.rel_path_to_xlsx_workbooks.get(&path.extract_name()) {
            None => {
                // the file-name doesn't exist in parse-information
                continue
            }
            Some(workbook) => {workbook}
        };
        for (pos, worksheet) in read_xlsx(path)?.iter().enumerate() {
            let sheet_info = match workbook.sheet_infos.get(&(pos + 1)) {
                None => {
                    // worksheet doesn't exist in parse-information
                    continue}
                Some(sheet_info) => {sheet_info}
            };

            let sheet = Sheet::new(sheet_info.resource_name.to_owned(), workbook.rel_path.to_owned(), pos + 1, worksheet.1.to_owned());
            sheets.push(sheet);
        }
    }
    Ok(sheets)
}
