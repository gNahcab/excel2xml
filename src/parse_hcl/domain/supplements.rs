use std::collections::HashMap;
use hcl::{Expression, Structure};
use crate::parse_hcl::domain::prop_supplement::{to_prop_supplement_type, PropSupplement};
use crate::parse_hcl::domain::resource_supplement::{to_res_supplement_type, ResourceSupplement};
use crate::parse_hcl::errors::HCLDataError;



#[derive(Debug, Clone)]
pub struct Supplements {
    pub header_to_res_suppl: HashMap<String, ResourceSupplement>,
    pub header_to_prop_suppl: HashMap<String, PropSupplement>,
}

impl Supplements {
    fn new(header_to_res_suppl: HashMap<String, ResourceSupplement>, header_to_prop_suppl: HashMap<String, PropSupplement>) -> Supplements {
        Supplements{
            header_to_res_suppl,
            header_to_prop_suppl
        }
    }
}

struct TransientSupplements {
    header_to_res_suppl: Option<HashMap<String, ResourceSupplement>>,
    header_to_prop_suppl: HashMap<String, PropSupplement>,
}


impl TransientSupplements {
    fn new() -> Self {
        Self{ header_to_res_suppl: None, header_to_prop_suppl: Default::default() }
    }
    pub(crate) fn add_header_to_res_supplement(&mut self, header_to_res_supplement: HashMap<String, ResourceSupplement>) -> Result<(), HCLDataError> {
        if self.header_to_res_suppl.is_some() {
                return Err(HCLDataError::InputError(format!("multiple header_to_res_supplement: First: '{:?}', Second: '{:?}'", self.header_to_res_suppl.as_ref().unwrap(), header_to_res_supplement)));
        }
        self.header_to_res_suppl = Some(header_to_res_supplement);
        Ok(())
    }
    pub(crate) fn extend_header_to_prop_supplement(&mut self, header_to_prop_supplement: HashMap<String, PropSupplement>) -> Result<(), HCLDataError> {
        for (header, prop_suppl) in header_to_prop_supplement.iter() {
            if self.header_to_prop_suppl.contains_key(header) {
                return Err(HCLDataError::InputError(format!("Tried to use the same header '{}' for multiple prop-supplements. First:{:?}, second: {:?}", header, self.header_to_prop_suppl.get(header).as_ref().unwrap(), prop_suppl)));
            } else {
                self.header_to_prop_suppl.insert(header.to_owned(), prop_suppl.to_owned());
            }
        }
        Ok(())
    }
}

struct SupplementWrapper(pub(crate) hcl::Block);

pub(crate) struct SupplementsWrapper(pub(crate) hcl::Block);
impl SupplementsWrapper {
    pub(crate) fn to_supplements(&self) -> Result<Supplements, HCLDataError> {
        let mut transient_supplements = TransientSupplements::new();
        for suppl_block in self.0.body.blocks() {
            let propname = to_supplements_propname(suppl_block.identifier.as_str())?;
            if !suppl_block.labels.is_empty() {
                return Err(HCLDataError::InputError(format!("Found labels '{:?}' for supplement with identifier'{}'. Labels are not allowed for supplements.", suppl_block.labels, propname)))
            }
            match propname.as_str() {
                "resource" => {
                    let header_to_res_supplement = SupplementWrapper(suppl_block.to_owned()).to_resource_supplements(propname.to_owned())?;
                    transient_supplements.add_header_to_res_supplement(header_to_res_supplement)?;
                }

                &_ => {
                    let header_to_prop_supplements = SupplementWrapper(suppl_block.to_owned()).header_to_prop_supplement(propname.to_owned())?;
                    transient_supplements.extend_header_to_prop_supplement(header_to_prop_supplements)?;
                }
            }
        }
        let header_to_res_suppl = match transient_supplements.header_to_res_suppl {
            None => {
                HashMap::new()
            }
            Some(header_to_res_suppl) => {
                header_to_res_suppl
            }
        };
        Ok(Supplements::new(header_to_res_suppl, transient_supplements.header_to_prop_suppl))
    }
}
fn to_supplements_propname(identifier: &str) -> Result<String, HCLDataError> {
    let propname = identifier.to_string();
    if propname.is_empty() {
        return Err(HCLDataError::InputError("Found empty string for supplement-identifier.".to_string()))
    }
    Ok(propname)
}



impl SupplementWrapper {
    pub(crate) fn to_resource_supplements(&self, resname: String) -> Result<HashMap<String, ResourceSupplement>, HCLDataError> {
        let mut header_to_res_supplement = HashMap::new();
        for s in &self.0.body {
            match s {
                Structure::Attribute(attr) => {
                    let header_in_document = match &attr.expr {
                        Expression::String(header_in_document) => {
                            header_in_document.to_owned()
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("Supplements: Only strings are allowed for now as headers in document. For res-prop-type '{:?}' found '{:?}'", attr.key.as_str(), attr.expr)))
                        }
                    };
                    let suppl_type = to_res_supplement_type(attr.key.as_str())?;

                    header_to_res_supplement.insert(header_in_document, ResourceSupplement::new(resname.to_owned(), suppl_type));
                }
                Structure::Block(block) => {
                    return Err(HCLDataError::InputError(format!("Block not allowed in supplement-body. But found block: '{:?}'", block)))
                }
            }
        }
        Ok(header_to_res_supplement)
    }
    pub fn header_to_prop_supplement(&self, propname: String) -> Result<HashMap<String, PropSupplement>, HCLDataError>{
        let mut header_to_prop_supplement = HashMap::new();
        for s in &self.0.body {
            match s {
                Structure::Attribute(attr) => {
                    let header_in_document = match &attr.expr {
                        Expression::String(header_in_document) => {
                            header_in_document.to_owned()
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("Supplements: Only strings are allowed for now as headers in document. For prop-type '{:?}' found '{:?}'", attr.key.as_str(), attr.expr)))
                        }
                    };
                    let suppl_type = to_prop_supplement_type(attr.key.as_str())?;

                    header_to_prop_supplement.insert(header_in_document, PropSupplement::new(propname.to_owned(), suppl_type));
                }
                Structure::Block(block) => {
                    return Err(HCLDataError::InputError(format!("Block not allowed in supplement-body. But found block: '{:?}'", block)))
                }
            }
        }
        Ok(header_to_prop_supplement)
    }
}

