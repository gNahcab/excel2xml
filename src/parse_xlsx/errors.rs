
#[derive(Debug, PartialEq)]
pub enum ExcelDataError {
    ParsingError(String),
    InputError(String),
    CellError(calamine::CellErrorType)
}
impl From<calamine::CellErrorType> for ExcelDataError {
    fn from(error: calamine::CellErrorType) -> Self {
        ExcelDataError::CellError(error)
    }
}
