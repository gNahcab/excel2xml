#[derive(Debug)]
pub enum HCLDataError {
    ParsingError(String),
    InputError(String),
    RegexError(regex::Error),
    ParseInt(String),
    MethodError(MethodError)
}
impl From<regex::Error> for HCLDataError {
    fn from(error: regex::Error) -> Self {
        HCLDataError::RegexError(error)
    }
}
impl From<MethodError> for HCLDataError {
    fn from(error: MethodError) -> Self {
        HCLDataError::MethodError(error)
    }
}

#[derive(Debug)]
pub enum MethodError {
    Combine(String)
}