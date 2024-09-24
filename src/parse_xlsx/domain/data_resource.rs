use std::collections::HashMap;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::property::Property;
use crate::json2datamodel::domain::resource::DMResource;
use crate::parse_xlsx::domain::permissions::{Permissions, PermissionsWrapper};
use crate::parse_xlsx::domain::prop_name::PropName;
use crate::parse_xlsx::domain::raw_resource_data::RawResourceData;
use crate::parse_xlsx::errors::ExcelDataError;
use crate::parse_xlsx::errors::ExcelDataError::ParsingError;

pub struct DataResource {
    res_name: String,
    propname_to_values: HashMap<PropName, Vec<String>>
}

pub struct DataResourceWrapper(pub(crate) RawResourceData);

impl DataResourceWrapper {
    pub(crate) fn to_data_resource(&self, data_model: &DataModel, separator: String) -> Result<DataResource, ExcelDataError> {
        let mut transient_data_resource = TransientDataResource::new();
        for (prop_name, raw_data) in self.0.raw_data.iter() {
            match prop_name {
                PropName::ID => {
                    transient_data_resource.add_id(raw_data.raw_value.to_owned())?;
                }
                PropName::LABEL => {
                    transient_data_resource.add_label(raw_data.raw_value.to_owned())?;
                }
                PropName::PERMISSIONS => {
                    let permission = PermissionsWrapper(raw_data.raw_value.to_owned()).to_permissions()?;
                    transient_data_resource.add_permissions(permission)?;
                }
                PropName::ARK => {
                    transient_data_resource.add_ark(raw_data.raw_value.to_owned())?;
                }
                PropName::IRI => {
                    transient_data_resource.add_iri(raw_data.raw_value.to_owned())?;
                }
                PropName::BITSTREAM => {
                    transient_data_resource.add_ark(raw_data.raw_value.to_owned())?;
                }
                PropName::ProjectProp(prop_name) => {
                    transient_data_resource.project_prop(prop_name.to_string() ,raw_data.raw_value.to_owned(), separator.as_str())?;
                }
            }
        }
        transient_data_resource.label_id_exist(data_model)?;
        transient_data_resource.properties_correct(data_model, self.0.res_name.to_string())?;
        Ok(DataResource {
            res_name: "".to_string(),
            propname_to_values: Default::default(),
        })

    }
}
struct TransientDataResource {
    id: Option<String>,
    label: Option<String>,
    permissions: Option<Permissions>,
    iri: Option<String>,
    ark: Option<String>,
    project_prop: HashMap<String, Vec<String>>,
}

impl TransientDataResource {
}

impl TransientDataResource {
    pub(crate) fn properties_correct(&self, data_model: &DataModel, res_name: String) -> Result<(), ExcelDataError> {
        let propnames_of_res = data_model.resources.iter().filter(|resource| resource.name == res_name).collect::<Vec<&DMResource>>().first().unwrap()
            .properties.iter().map( |res_property| res_property.propname.as_str()).collect::<Vec<&str>>();

        for (prop_name, prop_values) in self.project_prop.iter() {
            // propname must be part of resource
            if !propnames_of_res.contains(&res_name.as_str()) {
                return Err(ParsingError(format!("Property '{}' not found in Resource '{}'. All propnames: {:?}", prop_name, res_name, propnames_of_res)))
            }
            // identify propname as ListValue, DateValue etc. and check accordingly
            prop_values_match_prop_type(prop_name, prop_values, data_model)?;
        }
        Ok(())
    }
}

fn prop_values_match_prop_type(prop_name: &String, prop_values: &Vec<String>, data_model: &DataModel) -> Result<(), ExcelDataError> {
    let property = data_model.properties.iter().filter(|property|property.name.as_str() == prop_name).collect::<Vec<&Property>>().first();
    // project-prop:
    // - if prop is list, then check if value(s) exist in list
    // - if prop is int, then check if value(s) is valid integer
    // - if prop is float, then check if value(s) is valid float
    // - if prop is date, then check if value(s) is valid date
    // - if prop is color, then check if value(s) is valid color
    // - ....
    todo!()
}

impl TransientDataResource {
    pub(crate) fn label_id_exist(&self, data_model: &DataModel) -> Result<(), ExcelDataError> {
        // label, id must exist
        if self.id.is_none() {
            return Err(ExcelDataError::InputError(format!("Missing id for data resource with label: {}", self.label.as_ref().unwrap())));
        }
        if self.label.is_none() {
            return Err(ExcelDataError::InputError(format!("Missing label for data resource with id: {}", self.id.as_ref().unwrap())));
        }
        Ok(())
    }
}


impl TransientDataResource {
    fn new() -> Self {
        TransientDataResource{
            id: None,
            label: None,
            permissions: None,
            iri: None,
            ark: None,
            project_prop: Default::default(),
        }
    }
    pub(crate) fn add_id(&mut self, id: String) -> Result<(), ExcelDataError>  {
        if self.id.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate id: {}", id) ))
        }
        self.id = Option::from(id);
        Ok(())
    }
    pub(crate) fn add_label(&mut self, label: String) -> Result<(), ExcelDataError>  {
        if self.label.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate label: {}", label) ))
        }
        self.label = Option::from(label);
        Ok(())
    }
    pub(crate) fn add_permissions(&mut self, permissions: Permissions) -> Result<(), ExcelDataError>  {
        if self.permissions.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate permissions: {:?}", permissions) ))
        }
        self.permissions = Option::from(permissions);
        Ok(())
    }
    pub(crate) fn add_iri(&mut self, iri: String) -> Result<(), ExcelDataError> {
        if self.iri.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate iri: {}", iri) ))
        }
        self.iri = Option::from(iri);
        Ok(())
    }
    pub(crate) fn add_ark(&mut self, ark: String) -> Result<(), ExcelDataError>  {
        if self.ark.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate ark: {}", ark) ))
        }
        self.ark = Option::from(ark);
        Ok(())
    }
    pub(crate) fn project_prop(&mut self, prop_name: String, project_value: String, separator: &str) -> Result<(), ExcelDataError> {
        if self.project_prop.contains_key(&prop_name) {
            return Err(ExcelDataError::InputError( format!("Duplicate prop_name: {}", prop_name) ))
        }
        let project_values =project_value.split(separator).map(|s| s.to_string()).collect();
        self.project_prop.insert(prop_name, project_values);
        Ok(())
    }
}


