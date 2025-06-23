#[derive(Debug)]
pub enum PathOpError {
    IO(std::io::Error),
    WrongPath(String)
}
