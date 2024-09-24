use std::collections::HashMap;
use crate::transfrom2datamodel::domain::data_model::DataModel;
use crate::transfrom2datamodel::domain::dasch_list::DaSCHList;
use crate::transfrom2datamodel::domain::ontology::Ontology;
use crate::transfrom2datamodel::domain::property::Property;
use crate::transfrom2datamodel::domain::resource::Resource;
use super::Builder;
pub struct DataModelBuilder {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
    pub lists: HashMap<String, DaSCHList>,
}
impl Builder for DataModelBuilder {
    type OutputType = DataModel;

    fn new() -> Self {
        DataModelBuilder {
            ontologies: vec![],
            properties: vec![],
            resources: vec![],
            lists: HashMap::new(),
        }
    }

    fn add_to_ontology(&mut self, ontology: Ontology) {
        self.ontologies.push(ontology);
    }

    fn add_to_properties(&mut self, properties: Vec<Property>) {
        self.properties.extend(properties);
    }

    fn add_to_resources(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    fn add_list(&mut self, name: String, list: DaSCHList) {
        self.lists.insert(name, list);
    }

    fn build(self) -> DataModel {
        DataModel {
            ontologies: self.ontologies,
            properties: self.properties,
            resources: self.resources,
            lists: self.lists,
        }
    }
}