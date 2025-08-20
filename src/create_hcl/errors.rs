
#[derive(Debug)]
pub enum CreateHCLError {
    InputError(String),
    NotFoundError(String),
    IOError(std::io::Error),
    HCLError(hcl::Error)
}
impl From<hcl::Error> for CreateHCLError {
    fn from(error: hcl::Error) -> Self {
        CreateHCLError::HCLError(error)
    }
}
impl From<std::io::Error> for CreateHCLError {
    fn from(error: std::io::Error) -> Self {
        CreateHCLError::IOError(error)
    }
}
