use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct Subheader {
    encoding: Option<usize>,
    comment: Option<usize>,
    permissions: Option<usize>,

}

impl Subheader {
    pub(crate) fn new(transient_subheader: TransientSubheader) -> Self {
        Subheader{
            encoding: transient_subheader.encoding,
            comment: transient_subheader.comment,
            permissions: transient_subheader.permissions,
        }
    }
}

pub struct TransientSubheader {
    encoding: Option<usize>,
    comment: Option<usize>,
    permissions: Option<usize>,
}


impl TransientSubheader {
    pub(crate) fn new() -> Self {
        TransientSubheader{
            encoding: None,
            comment: None,
            permissions: None,
        }
    }
    pub(crate) fn add_permissions(&mut self, pos: usize, propname: &String) -> Result<(), ExcelDataError> {
        if self.permissions.is_some() {
            return  Err(ExcelDataError::ParsingError(format!("found multiple 'permissions'-header after propname {}", propname)))
        }
        self.permissions = Option::Some(pos);
        Ok(())
    }
    pub(crate) fn add_comment(&mut self, pos: usize, propname: &String) -> Result<(), ExcelDataError> {
        if self.comment.is_some() {
            return  Err(ExcelDataError::ParsingError(format!("found multiple 'comment'-header after propname {}", propname)))
        }
        self.comment = Option::Some(pos);
        Ok(())
    }
    pub(crate) fn add_encoding(&mut self, pos: usize, propname: &String) -> Result<(), ExcelDataError> {
        if self.encoding.is_some() {
            return  Err(ExcelDataError::ParsingError(format!("found multiple 'encoding'-header after propname {}", propname)))
        }
        self.encoding = Option::Some(pos);
        Ok(())
    }
    pub(crate) fn has_values(&self) -> bool {
        // if any values are some: return true
        self.encoding.is_some() | self.comment.is_some() | self.permissions.is_some()
    }
}