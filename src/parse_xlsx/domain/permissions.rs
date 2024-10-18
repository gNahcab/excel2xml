use std::fmt::Display;
use crate::json2datamodel::domain::property::Property;
use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Debug, Clone)]
pub enum Permissions {
    DEFAULT,
    RESTRICTED
}
impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Permissions::DEFAULT => { "prop-default".to_string() }
            Permissions::RESTRICTED => { "prop-restricted".to_string() }
        };
        write!(f, "{}", str)
    }
}
pub struct PermissionsWrapper (pub(crate) String);
impl PermissionsWrapper {
    pub(crate) fn to_permissions(&self) -> Result<Permissions, ExcelDataError> {
        if self.0.trim().is_empty() {
            // default, if empty
            return Ok(Permissions::DEFAULT);
        }
        match self.0.to_lowercase().as_str() {
            "default"|"prop-default" => Ok(Permissions::DEFAULT),
            "restricted"|"prop-restricted" => Ok(Permissions::RESTRICTED),
            _ => Err(ExcelDataError::InputError(format!("invalid permissions string: {}", self.0)))
        }

    }
}
