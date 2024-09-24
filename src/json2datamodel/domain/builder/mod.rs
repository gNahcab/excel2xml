use crate::errors::Excel2XmlError;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::dasch_list::DaSCHList;
use crate::json2datamodel::domain::ontology::Ontology;
use crate::json2datamodel::domain::property::Property;
use crate::json2datamodel::domain::resource::DMResource;
use crate::json2datamodel::errors::DataModelError;

pub mod data_model_builder;

pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn add_to_ontology(&mut self, ontology: Ontology);
    fn add_to_properties(&mut self, properties: Vec<Property>);
    fn add_to_resources(&mut self, resources: Vec<DMResource>);

    fn add_list(&mut self, name: String, list:DaSCHList);
    fn is_complete(&self) -> Result<(), DataModelError>;

    fn build(self) -> DataModel;
}
