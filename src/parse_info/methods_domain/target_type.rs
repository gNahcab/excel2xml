use crate::hcl_info::errors::HCLDataError;

#[derive(Debug, Clone)]
pub enum TargetType {
    Part,
    Whole,
}

impl TargetType {
    pub(crate) fn target_type(string: String) -> Result<TargetType, HCLDataError>{
        match string.as_str() {
            "part" => Ok(TargetType::Part),
            "whole" => Ok(TargetType::Whole),
            _ => Err(HCLDataError::ParsingError(format!("unknown value for 'target'-attribute: {:?}", string))),
        }

    }
}