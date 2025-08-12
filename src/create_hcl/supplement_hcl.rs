use std::collections::HashMap;
use crate::create_hcl::errors::CreateHCLError;


pub struct SupplementHCL {
    supplement_type: SupplementType,
    attached_header: AttachedHeader
}

impl SupplementHCL {
    pub(crate) fn new(supplement_type: SupplementType, attached_header: AttachedHeader) -> Self {
        SupplementHCL{ supplement_type, attached_header }
    }
}

pub fn find_attached_type(curr_pos: &usize, headers: &Vec<String>, pos_header_to_propname: &HashMap<usize, String>, pos_header_to_id_label: &HashMap<usize, String>, pos_header_to_hcl_supplement: &HashMap<usize, SupplementHCL>) -> AttachedHeader {
    for pos in curr_pos.to_owned()..0usize {
        if pos_header_to_propname.contains_key(&pos) {
            return AttachedHeader::Propname(headers.get(pos).unwrap().to_string());
        } else if pos_header_to_id_label.contains_key(&pos) {
            return AttachedHeader::Resource;
        }
    }
    todo!()

}
pub enum AttachedHeader {
    Resource,
    Propname(String)
}
#[derive(Debug)]
pub enum SupplementType {
    Authorship,
    License,
    CopyrightHolder,
    Encoding,
    Comment,
    Bitstream,
    BitstreamPermission,
    Permissions,
    ARK,
    IRI,
}
pub fn to_supplement_type(supplement: &String) -> Result<SupplementType, CreateHCLError>{
    match supplement.as_str() {
        "authorship" => {
            Ok(SupplementType::Authorship)
        },
        "encoding" => {
           Ok(SupplementType::Encoding)
        }
        "licenses" => {
            Ok(SupplementType::License)
        },
        "copyright holder" => {
            Ok(SupplementType::CopyrightHolder)
        },
        "permissions" => {
            Ok(SupplementType::Permissions)
        },
        "bitstream_permissions" => {
            Ok(SupplementType::Permissions)
        },
        "copyright_holder" => {
            Ok(SupplementType::CopyrightHolder)
        },
        "ark"  => {
            Ok(SupplementType::ARK)
        },
        "iri" => {
            Ok(SupplementType::IRI)
        },
        _ => {
            Err(CreateHCLError::NotFoundError(format!("No Supplement-Type exists for this header: '{}'", supplement)))
        }
    }
}
