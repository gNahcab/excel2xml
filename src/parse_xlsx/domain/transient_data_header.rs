use std::collections::{HashMap};
use std::hash::Hash;
use crate::parse_dm::domain::super_field::SuperField;
use crate::parse_info::domain::prop_supplement::PropSupplement;
use crate::parse_xlsx::errors::ExcelDataError;


#[derive(Debug)]
pub struct TransientDataHeader {
    pub(crate) id: Option<usize>,
    pub(crate) label: Option<usize>,
    pub res_permissions: Option<usize>,
    pub iri: Option<usize>,
    pub ark: Option<usize>,
    pub(crate) bitstream: Option<usize>,
    pub(crate) bitstream_permissions: Option<usize>,
    pub(crate) propname_to_pos: HashMap<String, usize>,
    pub(crate) propname_to_pos_prop_supplement: HashMap<String, Vec<(usize, PropSupplement)>>,
}

impl TransientDataHeader {
    pub(crate) fn new() -> Self {
        TransientDataHeader {
            id: None,
            label: None,
            res_permissions: None,
            iri: None,
            ark: None,
            bitstream: None,
            bitstream_permissions: None,
            propname_to_pos: Default::default(),
            propname_to_pos_prop_supplement: Default::default(),
        }
    }
    pub fn add_iri_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.iri.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple iri-positions. First: {}, second: {}", self.iri.as_ref().unwrap(), pos)))
        }
        self.iri = Some(pos);
        Ok(())
    }
    pub fn add_ark_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.ark.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple ark-positions. First: {}, second: {}", self.ark.as_ref().unwrap(), pos)))
        }
        self.ark = Some(pos);
        Ok(())
    }
    pub fn add_id_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.id.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple id-positions. First: {}, second: {}", self.id.as_ref().unwrap(), pos)))
        }
        self.id = Some(pos);
        Ok(())
    }
    pub fn add_label_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.label.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple label-positions. First: {}, second: {}", self.label.as_ref().unwrap(), pos)))
        }
        self.label = Some(pos);
        Ok(())
    }
    pub fn add_bitstream_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.bitstream.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple bitstream-positions. First: {}, second: {}", self.bitstream.as_ref().unwrap(), pos)))
        }
        self.bitstream = Some(pos);
        Ok(())
    }
    pub fn add_bitstream_permissions_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.bitstream_permissions.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple bitstream-permissions-positions. First: {}, second: {}", self.bitstream_permissions.as_ref().unwrap(), pos)))
        }
        self.bitstream_permissions = Some(pos);
        Ok(())
    }
    pub fn add_permissions_pos(&mut self, pos: usize) -> Result<(), ExcelDataError> {
        if self.res_permissions.is_some() {
            return Err(ExcelDataError::InputError(format!("TransientDataHeader: Found multiple permissions-positions. First: {}, second: {}", self.res_permissions.as_ref().unwrap(), pos)))
        }
        self.res_permissions = Some(pos);
        Ok(())
    }
    pub fn add_prop_suppl(&mut self, prop_suppl: PropSupplement, pos: usize) {
        if !self.propname_to_pos_prop_supplement.contains_key(&prop_suppl.part_of) {
            self.propname_to_pos_prop_supplement.insert(prop_suppl.part_of.to_owned(), vec![]);
        }
        self.propname_to_pos_prop_supplement.get_mut(&prop_suppl.part_of).unwrap().push((pos, prop_suppl))
    }
    pub(crate) fn add_propname(&mut self, propname: String, pos: usize) -> Result<(), ExcelDataError> {
        if self.propname_to_pos.contains_key(&propname) {
            return Err(ExcelDataError::ParsingError(format!("found duplicate propname in headers: '{}'", propname)));
        }
        self.propname_to_pos.insert(propname, pos);
        Ok(())
    }
    fn check_field(&self, super_field: &SuperField) -> Result<(), ExcelDataError> {
        match super_field {
            SuperField::Resource => {
                // bitstream not necessary
            }
            SuperField::MovingImageRepresentation |
            SuperField::StillImageRepresentation |
            SuperField::AudioRepresentation => {
                if self.bitstream.is_none() {
                    return Err(ExcelDataError::InputError(format!("cannot find bitstream in resource '{:?}'. But bitstream is necessary for this kind of resource.", self)));
                }
                if self.bitstream_permissions.is_none() {
                    return Err(ExcelDataError::InputError(format!("cannot find bitstream-permissions in resource '{:?}'. But bitstream-permissions is necessary because resource has bitstream.", self)));
                }
            }
        }
        Ok(())

    }
    pub(crate) fn is_complete(&self, super_field: &SuperField) -> Result<(), ExcelDataError> {
        if self.id.is_none() {
            return Err(ExcelDataError::InputError(format!("Cannot find id for transient-data-header: {:?}", self)));
        }
        if self.label.is_none() {
            return Err(ExcelDataError::InputError(format!("Cannot find label for transient-data-header: {:?}", self)));
        }
        self.check_field(super_field)?;
        Ok(())

    }
}









