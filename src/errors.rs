use crate::parse_info::errors::HCLDataError;
use crate::parse_xlsx::errors::ExcelDataError;
use crate::path_operations::errors::PathOpError;
use crate::read_hcl::errors::ReadHCLError;
use crate::read_json::errors::ReadJsonError;
use crate::write_hcl::errors::WriteHCLError;

#[derive(Debug)]
pub enum Excel2XmlError {
    /*
    IOError(std::io::Error),
    XlsxError(XlsxError),
    InputError(String),
     */
    ExcelDataError(ExcelDataError),
    HCLDataError(HCLDataError),
    WriteHCLError(WriteHCLError),
    ReadHCLError(ReadHCLError),
    ReadJsonError(ReadJsonError),
    PathOpError(PathOpError),
}
impl From<PathOpError> for Excel2XmlError {
    fn from(error: PathOpError) -> Self {
        Excel2XmlError::PathOpError(error)
    }
}
impl From<ReadJsonError> for Excel2XmlError {
    fn from(error: ReadJsonError) -> Self {
        Excel2XmlError::ReadJsonError(error)
    }
}
impl From<ReadHCLError> for Excel2XmlError {
    fn from(error: ReadHCLError) -> Self {
        Excel2XmlError::ReadHCLError(error)
    }
}
impl From<WriteHCLError> for Excel2XmlError {
    fn from(error: WriteHCLError) -> Self {
        Excel2XmlError::WriteHCLError(error)
    }
}
impl From<HCLDataError> for Excel2XmlError {
    fn from(error: HCLDataError) -> Self {
        Excel2XmlError::HCLDataError(error)
    }
}
impl From<ExcelDataError> for Excel2XmlError {
    fn from(error: ExcelDataError) -> Self {
        Excel2XmlError::ExcelDataError(error)
    }
}
