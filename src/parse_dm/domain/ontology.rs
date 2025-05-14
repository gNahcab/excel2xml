use serde_json::{Map, Value};
use crate::parse_dm::domain::property::{Property, PropertyWrapper};
use crate::parse_dm::domain::resource::{DMResource, ResourceWrapper};
use crate::parse_dm::errors::DataModelError;
use crate::parse_dm::errors::DataModelError::ParsingError;

#[derive(Debug, PartialEq)]
pub struct Ontology {
    name: String,
    label: String,
}

impl Ontology {

    fn new(transient_ontology: TransientOntology) -> Ontology {
        Ontology{ name: transient_ontology.name.unwrap(), label: transient_ontology.label.unwrap()}
    }
}

struct TransientOntology{
    name: Option<String>,
    label: Option<String>,
    }

impl TransientOntology{
    fn new() -> TransientOntology{
        TransientOntology{ name: None, label: None }
    }
    pub(crate) fn add_label(&mut self, label: String) {
        self.label = Some(label);
    }
    pub(crate) fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub(crate) fn is_complete(&self) -> Result<(), DataModelError> {
        // ontology is complete if it has name and label
        if self.name.is_none(){
            return Err(ParsingError(format!("Ontology with label '{:?}' has no name.", self.label)))
        }
        if self.label.is_none(){
            return Err(ParsingError(format!("Ontology with name '{:?}' has no label.", self.name)))
        }
        Ok(())
    }
}

pub fn separate_ontology_properties_resources(onto_object: Map<String, Value>) -> Result<(Ontology, Vec<Property>, Vec<DMResource>), DataModelError> {
    let mut properties: Vec<Property> = vec![];
    let mut resources: Vec<DMResource> = vec![];
    let mut transient_ontology = TransientOntology::new();
    let name = onto_object.get("name");
    if name.is_none(){
        return Err(ParsingError("Ontology doesn't have a name-tag".to_string()));
    }
    transient_ontology.add_name(name.unwrap().to_string());
    for (key, value) in onto_object.iter() {
        match key.as_str() {
            "label" => {
                let label = match value {
                    Value::String(label) => {label}
                    _ => {return Err(DataModelError::ParsingError(format!("label of ontology should be a string but found something else for: {}.", value)))}
                };
                transient_ontology.add_label(label.to_owned());
            }
            "properties" => {
                let array = value.as_array().expect("properties of ontology must be an array");
                for raw_prop in array.iter() {
                    let property = PropertyWrapper(raw_prop.to_owned()).to_property(transient_ontology.name.as_ref().unwrap().to_string())?;
                    properties.push(property)
                }

            }
            "resources" => {
                        let resources_raw = value.as_array().expect("resources should be a json-array");
                        for resources_value in resources_raw.iter() {
                            let resource = ResourceWrapper(resources_value.to_owned()).to_resource()?;
                            resources.push(resource);
                        }
            }
            &_ => {
                // ignore other keys
            }
        }
        transient_ontology.is_complete()?;

    }
    Ok((Ontology::new(transient_ontology), properties, resources))

}
