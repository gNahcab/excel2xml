use crate::parse_hcl::errors::HCLDataError;

#[derive(Clone, Debug)]
pub struct PropSupplement {
    pub part_of: String,
    pub suppl_type: PropSupplType
}
#[derive(Clone, Debug)]
pub enum PropSupplType {
    Comment,
    Encoding,
    Permissions,
}
pub fn to_prop_supplement_type(key: &str) -> Result<PropSupplType, HCLDataError> {
    match key {
        "comment" => {
            Ok(PropSupplType::Comment)
        }
        "encoding" => {
            Ok(PropSupplType::Encoding)
        }
        "permissions" => {
            Ok(PropSupplType::Permissions)
        }
        _ => {
            return Err(HCLDataError::ParsingError(format!("Unknown Prop-Suppl-Type: '{}'. Add first.", key)))
        }
    }
}


impl PropSupplement {
    pub(crate) fn new(propname: String, suppl_type: PropSupplType) -> Self {
        PropSupplement {
            part_of: propname,
            suppl_type
        }
    }
}
