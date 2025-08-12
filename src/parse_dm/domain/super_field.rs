use crate::parse_dm::errors::DataModelError;

#[derive(Debug, PartialEq, Clone)]
pub enum SuperField {
    Resource,
    MovingImageRepresentation,
    StillImageRepresentation,
    AudioRepresentation
}

pub struct SuperFieldWrapper(pub String);

impl SuperFieldWrapper {
    pub(crate) fn to_super_field(&self) -> Result<SuperField, DataModelError> {
        match self.0.as_str() {
            "Resource" => {
                Ok(SuperField::Resource)
            }
            "MovingImageRepresentation" => {
                Ok(SuperField::MovingImageRepresentation)
            }
            "StillImageRepresentation" => {
                Ok(SuperField::StillImageRepresentation)
            }
            "AudioRepresentation" => {
                Ok(SuperField::AudioRepresentation)
            }
            _ => {
                Err(DataModelError::ParsingError(format!("Unknown 'super'-value '{}', cannot match  with existing super-field.", self.0)))
            }
        }
    }
}