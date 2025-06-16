use std::path::Path;
use calamine::{Reader, open_workbook, Xlsx, Range, Data};
use crate::read_xlsx::errors::ReadXlsxError;

pub fn read_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<(String, Range<Data>)>, ReadXlsxError> {
    let mut workbook: Xlsx<_> = open_workbook(path.as_ref().to_owned())?;
    let worksheets: Vec<(String, Range<Data>)> = workbook.worksheets();
    if worksheets.len() == 0 {
        return Err(ReadXlsxError::InputError(format!("no worksheets found in xlsx-document '{:?}'", path.as_ref())));
    }
    Ok(worksheets)
}


