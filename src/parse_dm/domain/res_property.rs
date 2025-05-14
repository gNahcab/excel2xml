use serde_json::Value;
use crate::parse_dm::domain::cardinality::{to_cardinality, Cardinality};
use crate::parse_dm::errors::DataModelError;

#[derive(Debug, PartialEq)]
pub struct ResProperty {
    pub(crate) propname: String,
    pub (crate) cardinality: Cardinality,
}

impl ResProperty {
    fn new(transient_res_property: TransientResProperty) -> ResProperty {
        ResProperty{
            propname: transient_res_property.propname.unwrap(),
            cardinality: transient_res_property.cardinality.unwrap(),
        }
    }
}

struct TransientResProperty {
    propname: Option<String>,
    cardinality: Option<Cardinality>,
}
impl TransientResProperty {
    fn new() -> Self {
        TransientResProperty{ propname: None, cardinality: None }
    }
    fn add_propname(&mut self, name: &str) {
        self.propname = Option::from(name.to_string());
    }
    fn add_cardinality(&mut self, name: Cardinality) {
        self.cardinality = Option::from(name);
    }
    fn is_complete(&self) -> Result<(), DataModelError> {
        // complete if propname and cardinality exist
        if self.propname.is_none() {
            return Err(DataModelError::ParsingError(format!("propname not found for res-property")));
        }
        if self.cardinality.is_none() {
            return Err(DataModelError::ParsingError(format!("cardinality not found for res-property: {:?}", self.cardinality.as_ref().unwrap())));
        }
        Ok(())
    }
}
pub(crate) struct ResPropertyWrapper(pub(crate) Value);

impl ResPropertyWrapper {
    pub fn to_res_prop(&self) -> Result<ResProperty, DataModelError> {
        let res_prop_raw = self.0.as_object().expect("Res property should be an object");
        let mut transient = TransientResProperty::new();
        for (key, value) in res_prop_raw.iter() {
            match key.as_str() {
                "propname" => {
                    let propname_raw = value.as_str().expect("propname should be a string");
                    let (propname, ontology) = separate_ontology_and_propname(propname_raw)?;
                    //todo: add ontology to prop_name of resource: can be added later
                    transient.add_propname(propname);
                }
                "cardinality" => {
                    let cardinality = value.as_str().expect("cardinality should be a string");
                    let cardinality = to_cardinality(cardinality)?;
                    transient.add_cardinality(cardinality);
                }
                _ => {
                    // ingnore
                } }
        }
        transient.is_complete()?;
        Ok(ResProperty::new(transient))
    }
}

fn separate_ontology_and_propname(propname_raw: &str) -> Result<(&str, &str), DataModelError> {
    let position_colon =  match propname_raw.find(':') {
        None => {return Err(DataModelError::ParsingError(format!("not found colon in prop_name {}.", propname_raw)))}
        Some(position) => {
            position}
    };
    Ok((&propname_raw[position_colon+1..], &propname_raw[..position_colon]))
}
