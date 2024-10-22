use std::collections::HashMap;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::dasch_list::DaSCHList;
use crate::parse_dm::domain::ontology::Ontology;
use crate::parse_dm::domain::property::Property;
use crate::parse_dm::domain::resource::DMResource;
use crate::parse_dm::errors::DataModelError;
use super::Builder;
pub struct DataModelBuilder {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<DMResource>,
    pub shortcode: Option<String>,
    pub lists: HashMap<String, DaSCHList>,
}


impl Builder for DataModelBuilder {
    type OutputType = DataModel;

    fn new() -> Self {
        DataModelBuilder {
            ontologies: vec![],
            properties: vec![],
            resources: vec![],
            shortcode: Option::None,
            lists: HashMap::new(),
        }
    }
    fn add_to_ontology(&mut self, ontology: Ontology) {
        self.ontologies.push(ontology);
    }

    fn add_to_properties(&mut self, properties: Vec<Property>) {
        self.properties.extend(properties);
    }

    fn add_to_resources(&mut self, resource: Vec<DMResource>) {
        self.resources.extend(resource);
    }

    fn add_list(&mut self, name: String, list: DaSCHList) {
        self.lists.insert(name, list);
    }

    fn is_complete(&self) -> Result<(), DataModelError> {
        if self.resources.is_empty() {
            return Err(DataModelError::ParsingError("no resources found".to_string()));
        }
        if self.properties.is_empty() {
            return Err(DataModelError::ParsingError("no properties found".to_string()));
        }
        if self.ontologies.is_empty() {
            return Err(DataModelError::ParsingError("no ontologies found".to_string()));
        }
        Ok(())
    }

    fn build(self) -> DataModel {
        DataModel {
            ontologies: self.ontologies,
            properties: self.properties,
            resources: self.resources,
            shortcode: self.shortcode.unwrap(),
            lists: self.lists,
        }
    }

    fn add_shortcode(&mut self, shortcode: String) {
        self.shortcode = Option::Some(shortcode);
    }
}