#[derive(Debug)]
pub enum PathOpError {
    IOError(std::io::Error),
    WrongPath(String)
}

impl From<std::io::Error> for PathOpError {
    fn from(error: std::io::Error) -> Self {
        PathOpError::IOError(error)
    }
}
