use std::io::Error;
use std::string;
use crate::json2datamodel::domain::property::Property;
use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Header {
    ID,
    Label,
    Permissions,
    Comment,
    Encoding,
    ARK,
    IRI,
    Bitstream,
    ProjectProp(String)
}

pub(crate) struct HeaderWrapper(pub(crate) String);

impl HeaderWrapper {
    pub fn to_header(&self, properties: &&Vec<Property>) -> Result<Header, ExcelDataError> {
        let propnames: Vec<&String> = properties.iter().map(|property|&property.name).collect();
        match self.0.trim().to_lowercase().as_str() {
            "comment" => {
                Ok(Header::Comment)
            },
            "encoding" => {
                Ok(Header::Encoding)
            },
            "id" => {
                Ok(Header::ID)
            },
            "iri" => {
                Ok(Header::IRI)
            },
            "ark" => {
                Ok(Header::ARK)
            },
            "label" => {
                Ok(Header::Label)
            },
            "bitstream" => {
                Ok(Header::Bitstream)
            },
            "permissions" => {
                Ok(Header::Permissions)
            },
            "permission" => {
                Ok(Header::Bitstream)
            },
            _ => {
                if propnames.contains(&&self.0) {
                    Ok(Header::ProjectProp(self.0.to_owned()))
                } else {
                    Err(ExcelDataError::ParsingError(format!("cannot find '{}' in propnames", self.0)))
                }
            }
        }
    }
}

pub trait Extractor {
    fn extract_value(&self) -> Result<String, ExcelDataError>;
}

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
