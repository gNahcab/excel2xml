#[derive(Debug)]
pub enum WriteXMLError {
    IO(std::io::Error),
}
impl From<std::io::Error> for WriteXMLError {
    fn from(error: std::io::Error) -> Self {
        WriteXMLError::IO(error)
    }
}
