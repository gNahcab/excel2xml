use std::fmt::Display;
use crate::parse_xlsx::domain::encoding::Encoding::{UTF8, XML};
use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Clone, Debug)]
pub enum Encoding {
    UTF8,
    XML
}
impl Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UTF8 => { "utf8".to_string() }
            XML => { "xml".to_string() }
        };
        write!(f, "{}", str)
    }
}

pub struct EncodingWrapper (pub(crate) String);
impl EncodingWrapper {
    pub fn to_encoding(&self) -> Result<Encoding, ExcelDataError> {
        match self.0.to_lowercase().trim() {
            "utf-8" | "utf8" | "utf 8" => {
                Ok(UTF8)
            },
            "xml" => {
                Ok(XML)
            },
            _ => {
                 Err(ExcelDataError::ParsingError(format!("cannot parse this encoding '{}'.", self.0)))
            }
        }
    }
}