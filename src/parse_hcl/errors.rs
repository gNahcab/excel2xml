#[derive(Debug)]
pub enum HCLDataError {
    ParsingError(String),
    InputError(String),
    RegexError(regex::Error),
    ParseInt(String),
}
impl From<regex::Error> for HCLDataError {
    fn from(error: regex::Error) -> Self {
        HCLDataError::RegexError(error)
    }
}
