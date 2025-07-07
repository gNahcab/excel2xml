use crate::parse_hcl::errors::HCLDataError;

#[derive(Debug, Clone)]
pub struct ResourceSupplement {
    pub part_of: String,
    pub suppl_type: ResourceSupplType
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceSupplType {
    IRI,
    ARK,
    Permissions,
    Bitstream,
    BitstreamPermissions,
    Authorship,
    License,
    CopyrightHolder,
}
pub fn to_res_supplement_type(key: &str) -> Result<ResourceSupplType, HCLDataError> {
    match key {
        "iri" => {
            Ok(ResourceSupplType::IRI)
        }
        "ark" => {
            Ok(ResourceSupplType::ARK)
        }
        "bitstream" => {
            Ok(ResourceSupplType::Bitstream)
        }
        "permissions" => {
            Ok(ResourceSupplType::Permissions)
        }
        "bitstream-permissions" => {
            Ok(ResourceSupplType::BitstreamPermissions)
        }
        "license" => {
            Ok(ResourceSupplType::License)
        }
        "copyright_holder" => {
            Ok(ResourceSupplType::CopyrightHolder)
        }
        "authorship" => {
            Ok(ResourceSupplType::Authorship)
        }
        _ => {
            return Err(HCLDataError::ParsingError(format!("Unknown Resource-Suppl-Type: '{}'. Add first.", key)))
        }
    }
}

impl ResourceSupplement {
    pub(crate) fn new(res_name: String, suppl_type: ResourceSupplType) -> Self {
        ResourceSupplement{ part_of: res_name, suppl_type }
    }
}
