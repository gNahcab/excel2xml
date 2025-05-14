use crate::parse_dm::domain::property::Property;
use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Header {
    ID,
    Label,
    /*
    Permissions,
    Comment,
    Encoding,
    ARK,
    IRI,
    Bitstream,
    ProjectProp(String)
     */
}


pub trait Extractor {
    fn extract_value(&self) -> Result<String, ExcelDataError>;
}

/*
impl Extractor for Header {
    fn extract_value(&self) -> Result<String, ExcelDataError> {
        match self {
            Header::ProjectProp(value) => {Ok(value.to_owned())}
            _ => {
                Err(ExcelDataError::ParsingError("can only extract from ProjectProp".to_string()))
            }
        }
    }
}

 */
