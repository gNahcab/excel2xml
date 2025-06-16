use calamine::XlsxError;

#[derive(Debug)]
pub enum ReadXlsxError {
    IOError(std::io::Error),
    InputError(String),
    PathNotFound(String),
    XlsxError(XlsxError),
}

impl From<std::io::Error> for ReadXlsxError {
    fn from(error: std::io::Error) -> Self {
        ReadXlsxError::IOError(error)
    }
}

impl From<calamine::XlsxError> for ReadXlsxError {
    fn from(error: XlsxError) -> Self {
        ReadXlsxError::XlsxError(error)
    }
}
