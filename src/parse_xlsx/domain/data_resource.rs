use std::collections::{HashMap, HashSet};
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::property::Property;
use crate::json2datamodel::domain::resource::DMResource;
use crate::parse_xlsx::domain::dasch_value::{ValueField, DaSCHValueWrapper};
use crate::parse_xlsx::domain::data_header::DataHeader;
use crate::parse_xlsx::domain::data_row::DataRow;
use crate::parse_xlsx::domain::data_sheet::{compare_header_to_data_model, DataSheet};
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::domain::header::{Header, HeaderWrapper};
use crate::parse_xlsx::domain::headers::{to_headers, Headers};
use crate::parse_xlsx::errors::ExcelDataError;
use crate::parse_xlsx::errors::ExcelDataError::ParsingError;
use crate::special_propnames::SpecialPropnames;

pub struct DataResource {
    res_name: String,
    header_to_values: HashMap<Header, Vec<String>>
}

pub struct DataResourceWrapper(pub(crate) DataRow);

impl DataResourceWrapper {
    pub(crate) fn to_data_resource(&self, data_model: &DataModel, separator: &String, res_name: &String, headers: &DataHeader) -> Result<DataResource, ExcelDataError> {
        //properties_of_resource_exist(headers, data_model, res_name)?;
        todo!()
    }
}

fn split_field(field: &String, separator: &String) -> Vec<String> {
    match field.contains(separator) {
        true => {
            field.split(separator).map(|splitter| splitter.to_string()).collect()
        }
        false => {
            vec![field.to_owned()]
        }
    }
}

fn properties_of_resource_exist(headers: &Headers, data_model: &DataModel, res_name: &String) -> Result<(), ExcelDataError> {
    let prop_names_of_resource: Vec<&String> =
        match data_model.resources.iter().find(|dm_resource: &&DMResource| dm_resource.name.eq(res_name) ) {
            None => {
                // should never happen
                return Err(ExcelDataError::InputError(format!("cannot find res-name '{}'in resources of data-model", res_name)));
            }
            Some(dm_resource) => {dm_resource.properties.iter().map(|property|&property.propname).collect() }
        };
    let propnames_from_headers:Vec<&String> = headers.propnames.values().collect();
    let missing_propnames: Vec<&&String> = prop_names_of_resource
        .iter()
        .filter(|propname| !propnames_from_headers.contains(propname))
        .collect();
    if missing_propnames.len() != 0 {
        return Err(ExcelDataError::InputError(format!("cannot find all propnames of resource in headers. Missing propnames: '{:?}'.", missing_propnames)));
    }
    Ok(())
}

fn check_field(splitted_field: &Vec<String>, data_model: &DataModel, headers: &String) -> Result<(), ExcelDataError> {
    // check if field corresponds to data-model
    todo!()
}

struct TransientDataResource {
    id: Option<(usize, String)>,
    label: Option<(usize, String)>,
    permissions: Option<Permissions>,
    iri: Option<(usize, String)>,
    ark: Option<(usize, String)>,
    propname_to_values: HashMap<String, ValueField>,
    propname_to_pos: HashMap<String, usize>,
    bitstream: Option<(usize, String)>,
}


impl TransientDataResource {
    fn new() -> Self {
        TransientDataResource{
            id: None,
            label: None,
            permissions: None,
            iri: None,
            ark: None,
            propname_to_values: Default::default(),
            bitstream: None,
            propname_to_pos: Default::default(),
        }
    }
    pub(crate) fn add_id(&mut self, id: String, pos: &usize) -> Result<(), ExcelDataError>  {
        if self.id.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate id: {}", id) ))
        }
        self.id = Option::from((pos.to_owned(), id));
        Ok(())
    }
    pub(crate) fn add_label(&mut self, label: String, pos: &usize) -> Result<(), ExcelDataError>  {
        if self.label.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate label: {}", label) ))
        }
        self.label = Option::from((pos.to_owned(), label));
        Ok(())
    }
    pub(crate) fn add_permissions(&mut self, permissions: Permissions) -> Result<(), ExcelDataError>  {
        if self.permissions.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate permissions: {:?}", permissions) ))
        }
        self.permissions = Option::from(permissions);
        Ok(())
    }
    pub(crate) fn add_iri(&mut self, iri: String, pos: &usize) -> Result<(), ExcelDataError> {
        if self.iri.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate iri: {}", iri) ))
        }
        self.iri = Option::from((pos.to_owned(), iri));
        Ok(())
    }
    pub(crate) fn add_ark(&mut self, ark: String, pos: &usize) -> Result<(), ExcelDataError>  {
        if self.ark.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate ark: {}", ark) ))
        }
        self.ark = Option::from((pos.to_owned(), ark));
        Ok(())
    }
    pub(crate) fn add_bitstream(&mut self, bitstream: String, pos: &usize) -> Result<(), ExcelDataError> {
        if self.bitstream.is_some() {
            return Err(ExcelDataError::InputError( format!("Duplicate bitstream: {}", bitstream) ))
        }
        self.bitstream = Option::from((pos.to_owned(), bitstream));
        Ok(())
    }
    pub(crate) fn add_values_of_prop(&mut self, prop_name: String, project_values: ValueField, pos: &usize) -> Result<(), ExcelDataError> {
        if self.propname_to_values.contains_key(&prop_name) {
            return Err(ExcelDataError::InputError( format!("Duplicate prop_name found: {}", prop_name) ))
        }
        self.propname_to_values.insert(prop_name.to_owned(), project_values);
        self.propname_to_pos.insert(prop_name, pos.to_owned());
        Ok(())
    }
    pub(crate) fn properties_correct(&self, data_model: &DataModel, res_name: String) -> Result<(), ExcelDataError> {
        let propnames_of_res = data_model.resources.iter().filter(|resource| resource.name == res_name).collect::<Vec<&DMResource>>().first().unwrap()
            .properties.iter().map( |res_property| res_property.propname.as_str()).collect::<Vec<&str>>();

        for (prop_name, prop_values) in self.propname_to_values.iter() {
            // propname must be part of resource
            if !propnames_of_res.contains(&res_name.as_str()) {
                return Err(ParsingError(format!("Property '{}' not found in Resource '{}'. All propnames: {:?}", prop_name, res_name, propnames_of_res)))
            }
            // identify propname as ListValue, DateValue etc. and check accordingly
            prop_values_match_prop_type(prop_name, prop_values, data_model)?;
        }
        Ok(())
    }
    pub(crate) fn label_id_exist(&self) -> Result<(), ExcelDataError> {
        // label, id must exist
        if self.id.is_none() {
            return Err(ExcelDataError::InputError(format!("Missing id for data resource with label: {:?}", self.label.as_ref().unwrap())));
        }
        if self.label.is_none() {
            return Err(ExcelDataError::InputError(format!("Missing label for data resource with id: {:?}", self.id.as_ref().unwrap())));
        }
        Ok(())
    }
    pub(crate) fn positions_correct(&self) -> Result<(), ExcelDataError> {
        // check that id, label, iri, ark are positioned at the beginning and all properties after
        let mut positions: Vec<usize>  = vec![&self.label, &self.id, &self.iri, &self.ark]
            .iter()
            .filter(|res_header|res_header.is_some())
            .map(|res_header|res_header.as_ref().unwrap().0)
            .collect();
        positions.sort();

        let positions_propnames: Vec<&usize> = self.propname_to_pos.iter().map(|(_, pos)|pos).collect();

        if positions.last().unwrap() > positions_propnames.first().unwrap() {

            return Err(ExcelDataError::ParsingError(format!("First header of propnames '{}' is before last header of resource '{}' (id, label, ark, iri)", positions_propnames.first().unwrap(), positions.last().unwrap())));
        }
        Ok(())
    }
    pub(crate) fn add_permissions_comment_encoding(&mut self, pos_to_header: &HashMap<usize, Header>) -> Result<(), ExcelDataError> {
        // to find permissions belonging to the resource: we look between id and bitstream or ark or iri or label (depends on what exists in headers)
        // permissions, encoding, comments belonging to resources: we look if the headers after the header with the properties are called 'permissions', 'encoding' or 'comment'
        // 1 check for permissions of resource: check between id, label, ark, iri, and (if existing) bitstream
        let mut positions: Vec<usize>  = vec![&self.label, &self.id, &self.iri, &self.ark]
            .iter()
            .filter(|res_header|res_header.is_some())
            .map(|res_header|res_header.as_ref().unwrap().0)
            .collect();
        positions.sort();
        let curr = positions.first().unwrap().to_owned();
        while curr <= (positions.last().unwrap() + 1) {
            let curr_header = pos_to_header.get(&curr);
            let header = match curr_header {
                None => {
                    return Err(ExcelDataError::ParsingError("Cannot find header, because position doesn't exist. This should never happen.".to_string()));
                }
                Some(header) => {header}
            };
            match header {
                Header::Permissions => {
                    //self.permissions = Option::Some(curr);
                    todo!()
                }
                _=> {continue}
            }

        }
        // 2 check for permissions of bitstream
        // 3 check for permissions of properties
        todo!()
    }
}

fn prop_values_match_prop_type(prop_name: &String, prop_values: &ValueField, data_model: &DataModel) -> Result<(), ExcelDataError> {
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




