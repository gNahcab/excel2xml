use std::path::{Path, PathBuf};
use calamine::{Data, Range};
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::read_xlsx::errors::ReadXlsxError;
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
pub fn sheets<P: AsRef<Path> + std::fmt::Debug>(folder_path: P, parse_information: &ParseInformation) -> Result<Vec<Sheet>, ReadXlsxError> {
    let read_dir = Path::read_dir(folder_path.as_ref())?;
    let paths = read_dir
        // remove Errors
        .filter(|entry|entry.is_ok())
        .map(|entry|entry.unwrap())
        // only files
        .filter(|path|path.path().is_file())
        .collect::<Vec<_>>();

    let mut sheets: Vec<Sheet> = vec![];

    for path in paths {
        let workbook = match parse_information.rel_path_to_xlsx_workbooks.get(&path.file_name().to_str().unwrap().to_string()) {
            None => {
                // the file-name doesn't exist in parse-information
                continue
            }
            Some(workbook_info) => {
                workbook_info
            }
        };
        for (pos, worksheet) in read_xlsx(PathBuf::from(path.path()))?.iter().enumerate() {
            let sheet_info = match workbook.sheet_infos.get(&(pos + 1)) {
                None => {
                    // worksheet doesn't exist in parse-information
                    continue }
                Some(sheet_info) => {sheet_info}
            };

            let sheet = Sheet::new(sheet_info.resource_name.to_owned(), workbook.rel_path.to_owned(), pos + 1, worksheet.1.to_owned());
            sheets.push(sheet);
        }
    }
    if sheets.is_empty() {
        return Err(ReadXlsxError::PathNotFound(format!("cannot find any sheets that match the described sheet in HCL. The folder '{:?}' doesn't seem to contain the files.", folder_path)));
    }
    Ok(sheets)
}
