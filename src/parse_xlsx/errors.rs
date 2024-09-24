#[derive(Debug, PartialEq)]
pub enum ExcelDataError {
    ParsingError(String),
    InputError(String),
}
