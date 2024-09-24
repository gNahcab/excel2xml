use calamine::XlsxError;
use crate::parse_info::errors::HCLDataError;
use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Debug)]
pub enum Excel2XmlError {
    IOError(std::io::Error),
    XlsxError(XlsxError),
    InputError(String),
    ExcelDataError(ExcelDataError),
    HCLError(hcl::Error),
    HCLDataError(HCLDataError),
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
impl From<std::io::Error> for Excel2XmlError {
    fn from(error: std::io::Error) -> Self {
        Excel2XmlError::IOError(error)
    }
}
impl From<XlsxError> for Excel2XmlError {
    fn from(error: XlsxError) -> Self {
        Excel2XmlError::XlsxError(error)
    }
}
impl From<hcl::Error> for Excel2XmlError {
    fn from(error: hcl::Error) -> Self {
        Excel2XmlError::HCLError(error)
    }
}
