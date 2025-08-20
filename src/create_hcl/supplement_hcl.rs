use std::collections::HashMap;
use crate::create_hcl::errors::CreateHCLError;


#[derive(Debug, Clone)]
pub struct SupplementHCL {
    pub supplement_type: SupplementType,
    pub attached_to_header: AttachedToHeader
}

impl SupplementHCL {
    pub(crate) fn new(supplement_type: SupplementType, attached_to_header: AttachedToHeader) -> Self {
        SupplementHCL{ supplement_type, attached_to_header }
    }
}

pub fn find_attached_type(curr_pos: &usize, headers: &Vec<String>, pos_header_to_propname: &HashMap<usize, String>, pos_header_to_id_label: &HashMap<usize, String>, pos_header_to_hcl_supplement: &HashMap<usize, SupplementHCL>) -> AttachedToHeader {
    for pos in (0usize..curr_pos.to_owned()).rev() {
        if pos_header_to_propname.contains_key(&pos) {
            return AttachedToHeader::Propname(headers.get(pos).unwrap().to_string());
        } else if pos_header_to_id_label.contains_key(&pos) {
            return AttachedToHeader::Resource;
        }
        match pos_header_to_hcl_supplement.get(&pos) {
            None => {
                // continue
            }
            Some(hcl_suppl) => {
                // attached to whatever this hcl-suppl is attached
                return hcl_suppl.attached_to_header.to_owned();
            }
        }
    }
    // back at 0 means -> attached to Resource
    AttachedToHeader::Resource
}

#[derive(Debug, Clone)]
pub enum AttachedToHeader {
    Resource,
    Propname(String)
}
#[derive(Debug, Clone)]
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
