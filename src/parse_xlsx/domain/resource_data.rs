use crate::parse_dm::domain::super_field::SuperField;
use crate::parse_hcl::domain::resource_supplement::{ResourceSupplType, ResourceSupplement};
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::errors::ExcelDataError;

pub struct ResourceSupplData {
    pub iri: Option<String>,
    pub ark: Option<String>,
    pub res_permissions: Option<Permissions>,
    pub bitstream: Option<String>,
    pub bitstream_permissions: Option<Permissions>,
    pub authorship: Option<Vec<String>>,
    pub copyright_holder: Option<String>,
    pub license: Option<String>
}

impl ResourceSupplData {
    fn new(transient_resource_data: TransientResourceData) -> Self {
        ResourceSupplData {
            iri: transient_resource_data.iri,
            ark: transient_resource_data.ark,
            res_permissions: transient_resource_data.res_permissions,
            bitstream: transient_resource_data.bitstream,
            bitstream_permissions: transient_resource_data.bitstream_permissions,
            authorship: transient_resource_data.authorship,
            copyright_holder: transient_resource_data.copyright_holder,
            license: transient_resource_data.license,
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
    authorship: Option<Vec<String>>,
    license: Option<String>,
    copyright_holder: Option<String> 
}

impl TransientResourceData {
}

impl TransientResourceData {
    fn new() -> Self {
        Self{
            iri: None,
            ark: None,
            res_permissions: None,
            bitstream: None,
            bitstream_permissions: None,
            authorship: None,
            license: None,
            copyright_holder: None,
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
                if self.authorship.is_none() {
                    return Err(ExcelDataError::InputError(format!("cannot find authorship in resource '{:?}'. But authorship is necessary for this kind of resource.", self)));
                }
                if self.license.is_none() {
                    return Err(ExcelDataError::InputError(format!("cannot find license in resource '{:?}'. But license is necessary for this kind of resource.", self)));
                }
                if self.copyright_holder.is_none() {
                    return Err(ExcelDataError::InputError(format!("cannot find copyright_holder in resource '{:?}'. But copyright_holder is necessary for this kind of resource.", self)));
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
    pub(crate) fn add_authorship(&mut self, authorship: String, separator: &String) -> Result<(), ExcelDataError> {
        if self.authorship.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple authorship-values for resource: First: '{:?}', Second: '{:?}'", self.authorship.as_ref().unwrap(), authorship)));
        }
        let authorships: Vec<String> = authorship.split(separator).into_iter().map(|value|value.to_string()).collect();
        self.authorship = Some(authorships);
        Ok(())
    }
    pub(crate) fn add_license(&mut self, license: String) -> Result<(), ExcelDataError> {
        if self.license.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple license-values for resource: First: '{}', Second: '{}'", self.license.as_ref().unwrap(), license)));
        }
        self.license = Some(license);
        Ok(())
    }
    pub(crate) fn add_copyright_holder(&mut self, copyright_holder: String) -> Result<(), ExcelDataError> {
        if self.copyright_holder.is_some() {
            return Err(ExcelDataError::InputError(format!("multiple copyright_holder-values for resource: First: '{}', Second: '{}'", self.copyright_holder.as_ref().unwrap(), copyright_holder)));
        }
        self.copyright_holder = Some(copyright_holder);
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
pub fn to_resource_data(res_suppl_values: &Vec<(ResourceSupplement, String)>, super_field: &SuperField, set_permissions: bool, separator: &String) -> Result<ResourceSupplData, ExcelDataError> {
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
            ResourceSupplType::Authorship => {
                transient_resource_suppl.add_authorship(value.to_owned(), separator)?;
            }
            ResourceSupplType::License => {
                transient_resource_suppl.add_license(value.to_owned())?;
            }
            ResourceSupplType::CopyrightHolder => {
                transient_resource_suppl.add_copyright_holder(value.to_owned())?;
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