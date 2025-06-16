
#[derive(Debug)]
pub enum ReadHCLError {
    IO(std::io::Error),
    HCLError(hcl::Error),
}
impl From<std::io::Error> for ReadHCLError {
    fn from(error: std::io::Error) -> Self {
        ReadHCLError::IO(error)
    }
}
impl From<hcl::Error> for ReadHCLError {
    fn from(error: hcl::Error) -> Self {
        ReadHCLError::HCLError(error)
    }
}
