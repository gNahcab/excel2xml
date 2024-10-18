use crate::json2datamodel::domain::object::ValueObject;
use crate::parse_xlsx::domain::encoding::Encoding;
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::errors::ExcelDataError;


pub struct TransientSubheaderValues {
    pub permissions: Option<Vec<Permissions>>,
    pub encodings: Option<Vec<Encoding>>,
    pub comments: Option<Vec<String>>
}



impl TransientSubheaderValues {
    pub(crate) fn new() -> TransientSubheaderValues {
        TransientSubheaderValues{
            permissions: None,
            encodings: None,
            comments: None,
        }
    }
    pub(crate) fn add_permissions(&mut self, permissions: Vec<Permissions>) {
        self.permissions = Some(permissions);
    }
    pub(crate) fn add_encodings(&mut self, encodings: Vec<Encoding>) {
        self.encodings = Some(encodings);
    }
    pub(crate) fn add_comments(&mut self, comments: Vec<String>) {
        self.comments = Some(comments);
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.comments.is_none() & self.encodings.is_none() & self.comments.is_none()
    }
    pub(crate) fn values_ok(&self, value_object:  &ValueObject, propname: &String) -> Result<(), ExcelDataError> {
        if self.encodings.is_some() {
            if !matches!(value_object, ValueObject::TextValue) {
                return Err(ExcelDataError::ParsingError(format!("found encoding, but Value-Object of Property '{}' is not 'TextValue' but: '{:?}'", propname, value_object)));
            }
            // todo check if SimpleText or RichText; if SimpleText: no XML allowed
        }
        Ok(())
    }
}