#[derive(Debug)]
pub enum ReadJsonError {
    IO(std::io::Error)
}
impl From<std::io::Error> for ReadJsonError {
    fn from(error: std::io::Error) -> Self {
        ReadJsonError::IO(error)
    }
}
