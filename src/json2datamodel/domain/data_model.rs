use std::collections::HashMap;
use serde_json::{Map, Value};
use crate::transfrom2datamodel::domain::builder::Builder;
use crate::transfrom2datamodel::domain::builder::data_model_builder::DataModelBuilder;
use crate::transfrom2datamodel::domain::dasch_list::{DaSCHList, DaSCHListWrapper};
use crate::transfrom2datamodel::domain::ontology::{separate_ontology_and_properties, Ontology};
use crate::transfrom2datamodel::domain::property::Property;
use crate::transfrom2datamodel::domain::resource::{Resource, ResourceWrapper};
use crate::transfrom2datamodel::errors::{DataModelError};

#[derive(Debug, PartialEq)]
pub struct DataModel {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
    pub lists: HashMap<String, DaSCHList>,
}
impl DataModel {
    pub(crate) fn new(
        ontologies: Vec<Ontology>,
        properties: Vec<Property>,
        resources: Vec<Resource>,
        lists: HashMap<String, DaSCHList>,

    ) -> Self {

        DataModel {
            ontologies,
            properties,
            resources,
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
        let object = json_value.as_object().expect("expecting a json object on top: data model malformed");
        let project = object.get("project").expect("expecting a project").as_object().expect("project should be a json-object");

        let mut data_model_builder: DataModelBuilder = DataModelBuilder::new();
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
                        let (ontology, properties) = separate_ontology_and_properties(onto_object.to_owned())?;
                        data_model_builder.add_to_ontology(ontology);
                        data_model_builder.add_to_properties(properties.iter().map(|property|property.clone()).collect());


                        }
                    }
                "resources" => {
                    let resources_raw = project.as_array().expect("resources should be a json-array");
                    for resources_value in resources_raw.iter() {
                        let resource = ResourceWrapper(resources_value.to_owned()).to_resource()?;
                        data_model_builder.add_to_resources(resource);
                    }
                }
                _ => {
                    //do nothing
                }
            }
        }
        Ok(data_model_builder.build())
    }
}

#[cfg(test)]

mod test {
    use serde_json::json;
    use crate::transfrom2datamodel::domain::data_model::DataModel;
    #[test]
    fn test_try_from () {
        todo!()
    }
}
