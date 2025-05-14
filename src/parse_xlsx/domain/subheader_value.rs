use crate::parse_dm::domain::object::ValueObject;
use crate::parse_dm::domain::property::Property;
use crate::parse_xlsx::domain::instance::split_field;
use crate::parse_xlsx::domain::encoding::{Encoding, EncodingWrapper};
use crate::parse_xlsx::domain::header::Header;
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::domain::subheader::Subheader;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct SubheaderValues {
    pub permissions: Vec<Permissions>,
    pub encodings: Option<Vec<Encoding>>,
    pub comments: Option<Vec<String>>

}
impl SubheaderValues {
    fn new() -> Self {
        todo!()
    }

}

/*
pub fn subheader_value(rows:&Vec<String>, subheader: &Subheader, separator: &String, property: &&Property, propname: &String) -> Result<Option<SubheaderValues>, ExcelDataError>{
    // return Subheader or None
    let mut transient_subheader_value: TransientSubheaderValues = TransientSubheaderValues::new();
    if subheader.permissions.is_some() {
        let raw_value = &rows[subheader.permissions.unwrap()].trim();
        let values = split_field(raw_value, separator);
        if !values.is_empty() {
            let mut permissions = vec![];
            for value in values {
                permissions.push(PermissionsWrapper(value).to_permissions()?);
            }
            transient_subheader_value.add_permissions(permissions);
        }
    }
    if subheader.encoding.is_some() {
        let raw_value = &rows[subheader.encoding.unwrap()].trim();
        let values = split_field(raw_value, separator);
        if !values.is_empty() {
            let mut encodings: Vec<Encoding> = vec![];
            for value in values {
                encodings.push(EncodingWrapper(value).to_encoding()?);
            }
            transient_subheader_value.add_encodings(encodings);
        }

    }
    if subheader.comment.is_some() {
        let raw_value = &rows[subheader.comment.unwrap()].trim();
        let values = split_field(raw_value, separator);
        if !values.is_empty() {
            transient_subheader_value.add_comments(values);
        }
    }
    if !transient_subheader_value.is_empty() {
        transient_subheader_value.values_ok(&property.object, propname)?;
        return Ok(Some(SubheaderValues::new(transient_subheader_value)));
    }
    Ok(None)
}

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
        match value_object {
            ValueObject::TextValue => {}
            _ => {
                return Ok(());
            }
        }
        if self.encodings.is_some() {
            if !matches!(value_object, ValueObject::TextValue) {
                return Err(ExcelDataError::ParsingError(format!("found encoding, but Value-Object of Property '{}' is not 'TextValue' but: '{:?}'", propname, value_object)));
            }
            // todo check if SimpleText or RichText; if SimpleText: no XML allowed
        }
        Ok(())
    }
}

 */