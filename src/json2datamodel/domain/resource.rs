use serde_json::Value;
use crate::transfrom2datamodel::domain::label::Label;
use crate::transfrom2datamodel::domain::ontology::Ontology;
use crate::transfrom2datamodel::domain::res_property::ResProperty;
use crate::transfrom2datamodel::errors::DataModelError;

#[derive(Debug, PartialEq)]
pub struct Resource {
    name: String,
    labels: Vec<Label>,
    properties: Vec<ResProperty>
}
pub(crate) struct ResourceWrapper(pub(crate) Value);
impl ResourceWrapper{
    pub fn to_resource(&self) -> Result<Resource, DataModelError> {
        todo!()
    }
}
