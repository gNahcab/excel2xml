use crate::transfrom2datamodel::domain::data_model::DataModel;
use crate::transfrom2datamodel::domain::dasch_list::DaSCHList;
use crate::transfrom2datamodel::domain::ontology::Ontology;
use crate::transfrom2datamodel::domain::property::Property;
use crate::transfrom2datamodel::domain::resource::Resource;
use crate::transfrom2datamodel::errors::DataModelError;

pub mod data_model_builder;

pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn add_to_ontology(&mut self, ontology: Ontology);
    fn add_to_properties(&mut self, properties: Vec<Property>);
    fn add_to_resources(&mut self, resource:Resource);

    fn add_list(&mut self, name: String, list:DaSCHList);

    fn build(self) -> DataModel;
}
