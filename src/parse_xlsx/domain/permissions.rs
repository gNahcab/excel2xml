use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Debug)]
pub enum Permissions {
    DEFAULT,
    RESTRICTED
}
pub struct PermissionsWrapper (pub(crate) String);
impl PermissionsWrapper {
    pub(crate) fn to_permissions(&self) -> Result<Permissions, ExcelDataError> {
        match self.0.to_lowercase().as_str() {
            "default" => Ok(Permissions::DEFAULT),
            "restricted" => Ok(Permissions::RESTRICTED),
            _ => Err(ExcelDataError::InputError(format!("invalid permissions string: {}", self.0)))
        }

    }
}
