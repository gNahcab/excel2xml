use crate::parse_xlsx::domain::encoding::Encoding::{UTF8, XML};
use crate::parse_xlsx::errors::ExcelDataError;

pub enum Encoding {
    UTF8,
    XML
}

pub struct EncodingWrapper (pub(crate) String);
impl EncodingWrapper {
    pub fn to_encoding(&self) -> Result<Encoding, ExcelDataError> {
        if self.0.trim().is_empty() {
            // empty default: UTF8
            return Ok(UTF8)
        }
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