#[derive(Debug)]
pub enum HCLDataError {
    ParsingError(String),
    InputError(String),
}
