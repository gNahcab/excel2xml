use std::collections::HashMap;
use serde_json::{Map, Value};
use crate::parse_dm::domain::builder::Builder;
use crate::parse_dm::domain::builder::data_model_builder::DataModelBuilder;
use crate::parse_dm::domain::dasch_list::{DaSCHList, DaSCHListWrapper};
use crate::parse_dm::domain::ontology::{separate_ontology_properties_resources, Ontology};
use crate::parse_dm::domain::property::Property;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_dm::errors::{DataModelError};

#[derive(Debug, PartialEq)]
pub struct DataModel {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<DMResource>,
    pub shortcode: String,
    pub shortname: String,
    pub lists: HashMap<String, DaSCHList>,
}
impl DataModel {
    pub(crate) fn new(
        ontologies: Vec<Ontology>,
        properties: Vec<Property>,
        resources: Vec<DMResource>,
        shortname: String,
        shortcode: String,
        lists: HashMap<String, DaSCHList>,

    ) -> Self {

        DataModel {
            ontologies,
            properties,
            resources,
            shortcode,
            shortname,
            lists,
        }
    }
}

struct Project {
    project: String,
    value: Value
}


impl TryFrom<Value> for DataModel {
    type Error = DataModelError;
    fn try_from(json_value: Value) -> Result<Self, Self::Error> {
        let object = json_value.as_object().expect("expecting a json object on top, but not found: data model malformed");
        let project = object.get("project").expect("expecting a project").as_object().expect("project should be a json-object");
        let mut data_model_builder: DataModelBuilder = DataModelBuilder::new();
        let shortcode = extract_string_from_project(project, "shortcode")?;
        data_model_builder.add_shortcode(shortcode);
        let shortname = extract_string_from_project(project, "shortname")?;
        data_model_builder.add_shortname(shortname);


        for (project_name, project) in project.iter() {
            match project_name.as_str() {
                "lists" => {
                    let lists_raw = project.as_array().expect("lists should be a json-array");
                    for list_value in lists_raw.iter() {
                        let list = DaSCHListWrapper(list_value.to_owned()).to_list()?;
                        data_model_builder.add_list(list.name.to_string(), list);
                    }
                }
                "ontologies" => {
                    let ontologies_raw = project.as_array().expect("ontologies should be a json-array");

                    for ontology_value in ontologies_raw.iter() {
                        let onto_object = ontology_value.as_object().expect("ontology should be a json-object");
                        let (ontology, properties, resources) = separate_ontology_properties_resources(onto_object.to_owned())?;
                        data_model_builder.add_to_ontology(ontology);
                        data_model_builder.add_to_properties(properties.iter().map(|property|property.clone()).collect());
                        data_model_builder.add_to_resources(resources);
                    }
                }
                _ => {
                    //do nothing
                }
            }
        }
        data_model_builder.is_complete()?;
        Ok(data_model_builder.build())
    }
}

fn extract_string_from_project(project: &Map<String, Value>, key: &str) -> Result<String, DataModelError> {
    let value = match   project.get(key) {
        None => {return Err(DataModelError::ParsingError(format!("Key '{}' not found in project", key)))}
        Some(value) => {
            match value {
                Value::String(str_value) => {str_value}
                _ => {
                    return Err(DataModelError::ParsingError(format!("Expected a String for '{}' but found something else: '{}'", key, value)));
                }
            }
        }
    };
    Ok(value.to_owned())
}

#[cfg(test)]

mod test {
    use serde_json::json;
    use crate::parse_dm::domain::data_model::DataModel;
    #[test]
    fn test_try_from () {
        todo!()
    }
}
