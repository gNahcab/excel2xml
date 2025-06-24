use crate::parse_dm::domain::super_field::SuperField;
use crate::parse_info::domain::resource_supplement::{ResourceSupplType, ResourceSupplement};
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct ResourceSupplData {
    pub iri: Option<String>,
    pub ark: Option<String>,
    pub res_permissions: Option<Permissions>,
    pub bitstream: Option<String>,
    pub bitstream_permissions: Option<Permissions>,
}

impl ResourceSupplData {
    fn new(transient_resource_data: TransientResourceData) -> Self {
        ResourceSupplData {
            iri: transient_resource_data.iri,
            ark: transient_resource_data.ark,
            res_permissions: transient_resource_data.res_permissions,
            bitstream: transient_resource_data.bitstream,
            bitstream_permissions: transient_resource_data.bitstream_permissions,
        }
    }
}
#[derive(Debug)]
struct TransientResourceData {
    iri: Option<String>,
    ark: Option<String>,
    res_permissions: Option<Permissions>,
    bitstream: Option<String>,
    bitstream_permissions: Option<Permissions>,
}

impl TransientResourceData {
    fn new() -> Self {
        Self{
            iri: None,
            ark: None,
            res_permissions: None,
            bitstream: None,
            bitstream_permissions: None,
        }
    }
    pub(crate) fn complete(&mut self, super_field: &SuperField, set_permissions: bool) -> Result<(), ExcelDataError> {
        if self.res_permissions.is_none() {
            if set_permissions {
                self.res_permissions = Some(Permissions::DEFAULT);
            }
            //return Err(ExcelDataError::InputError(format!("Cannot find resource-permissions in resource '{:?}'", self)));
        }
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
                /*
                if self.bitstream_permissions.is_none() {
                    return Err(ExcelDataError::InputError(format!("cannot find bitstream-permissions in resource '{:?}'. But bitstream-permissions is necessary because resource has bitstream.", self)));
                }
                 */
            }
        }
        Ok(())
    }
    pub(crate) fn add_ark(&mut self, ark: String) -> Result<(), ExcelDataError> {
        if self.ark.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple ark-values for resource: First: '{}', Second: '{}'", self.ark.as_ref().unwrap(), ark)));
        }
        self.ark = Some(ark);
        Ok(())
    }
    pub(crate) fn add_iri(&mut self, iri: String) -> Result<(), ExcelDataError> {
        if self.iri.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple iri-values for resource: First: '{}', Second: '{}'", self.iri.as_ref().unwrap(), iri)));
        }
        self.iri = Some(iri);
        Ok(())
    }
    pub(crate) fn add_permissions(&mut self, permissions: Permissions) -> Result<(), ExcelDataError> {
        if self.res_permissions.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple permissions for resource: First: '{}', Second: '{}'", self.res_permissions.as_ref().unwrap(), permissions)));
        }
        self.res_permissions = Some(permissions);
        Ok(())
    }
    pub(crate) fn add_bitstream_permissions(&mut self, permissions: Permissions) -> Result<(), ExcelDataError> {
        if self.bitstream_permissions.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple permissions for bitstream: First: '{}', Second: '{}'", self.bitstream_permissions.as_ref().unwrap(), permissions)));
        }
        self.bitstream_permissions = Some(permissions);
        Ok(())
    }
    pub(crate) fn add_bitstream(&mut self, bitstream: String) -> Result<(), ExcelDataError> {
        if self.bitstream.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple bitstream for resource: First: '{}', Second: '{}'", self.bitstream.as_ref().unwrap(), bitstream)));
        }
        self.bitstream = Some(bitstream);
        Ok(())
    }

}
pub fn to_resource_data(res_suppl_values: &Vec<(ResourceSupplement, String)>, super_field: &SuperField, set_permissions: bool) -> Result<ResourceSupplData, ExcelDataError> {
    let mut transient_resource_suppl = TransientResourceData::new();
    for (res_suppl, value) in res_suppl_values {
        match res_suppl.suppl_type {
            ResourceSupplType::Permissions => {
                let permission = PermissionsWrapper(value.to_string()).to_permissions()?;
                transient_resource_suppl.add_permissions(permission)?;
            }
            ResourceSupplType::Bitstream => {
                transient_resource_suppl.add_bitstream(value.to_owned())?;
            }
            ResourceSupplType::BitstreamPermissions => {
                let permission = PermissionsWrapper(value.to_string()).to_permissions()?;
                transient_resource_suppl.add_bitstream_permissions(permission)?;
            }
            ResourceSupplType::IRI => {
                transient_resource_suppl.add_iri(value.to_string())?
            }
            ResourceSupplType::ARK => {
                transient_resource_suppl.add_ark(value.to_string())?
            }
        }
    }
    transient_resource_suppl.complete(super_field, set_permissions)?;
    Ok(ResourceSupplData::new(transient_resource_suppl))
}