use serde_json::Value;
use crate::parse_dm::domain::label::{Label, LabelWrapper};
use crate::parse_dm::domain::res_property::{ResProperty, ResPropertyWrapper};
use crate::parse_dm::errors::DataModelError;

#[derive(Debug, PartialEq)]
pub struct DMResource {
    pub(crate) name: String,
    labels: Vec<Label>,
    pub super_field: String,
    pub(crate) properties: Vec<ResProperty>,
}

impl DMResource {
    fn new(transient_resource: TransientResource) -> Self {
        DMResource{
            name: transient_resource.name.unwrap(),
            labels: transient_resource.labels,
            super_field: transient_resource.super_.unwrap(),
            properties: transient_resource.res_props,
        }
    }
}

struct TransientResource {
    name: Option<String>,
    labels: Vec<Label>,
    super_: Option<String>,
    res_props: Vec<ResProperty>
}


impl TransientResource {
    fn new() -> Self {
        TransientResource{
            name: None,
            labels: vec![],
            super_: None,
            res_props: vec![],
        }
    }

    fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }
    fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
    fn add_super(&mut self, super_: String) {
        self.super_ = Some(super_);
    }
    fn add_res_prop(&mut self, res_prop: ResProperty) {
        self.res_props.push(res_prop);
    }
    fn is_complete(&self) -> Result<(), DataModelError> {
        // resource is complete if name, label, super, at least one res_prop exist
        if self.name.is_none() {
            return Err(DataModelError::ParsingError(format!("name is required for resource with labels: '{:?}'", self.labels)));
        }
        if self.labels.is_empty() {
            return Err(DataModelError::ParsingError(format!("at least one label is required for resource with name: '{:?}'", self.name)));
        }
        if self.super_.is_none() {
            return Err(DataModelError::ParsingError(format!("one super is required for resource with name: '{:?}'", self.name)));
        }
        if self.res_props.is_empty() {
            return Err(DataModelError::ParsingError(format!("at least one res_prop is required for resource with name: '{:?}'", self.name)));
        }
        Ok(())
    }
}
pub(crate) struct ResourceWrapper(pub(crate) Value);
impl ResourceWrapper{
    pub fn to_resource(&self) -> Result<DMResource, DataModelError> {
        let resource_raw = self.0.as_object().expect("resource should be an object");
        let mut transient_resource = TransientResource::new();
        for (key, value) in resource_raw.iter(){
            match key.as_str() {
                "name" => {
                    let name = match value {
                        Value::String(name) => {name}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("name '{:?}' of resource is not a String.", value)));
                        }
                    };
                    transient_resource.add_name(name.to_owned());
                }
                "labels" => {
                    let labels_raw = match value {
                        Value::Object(labels) => {labels}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("labels '{:?}' of resource with name '{:?}' is not an Object.", value, transient_resource.name)));
                        }
                    };
                    for (key, value) in labels_raw.iter() {
                        let label = LabelWrapper((key.to_owned(), value.to_owned())).to_label()?;
                        transient_resource.add_label(label);
                    }
                }
                "super" => {
                    let super_ = match value {
                        Value::String(super_) => {super_}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("super '{:?}' of resource with name '{:?}' is not a String.", value, transient_resource.name)));
                        }
                    };
                    transient_resource.add_super(super_.to_owned());
                }
                "cardinalities" => {
                    let res_props_raw = match value {
                        Value::Array(res_props_raw) => {res_props_raw}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("cardinalities '{:?}' of resource with name '{:?}' is not a String.", value, transient_resource.name)));
                        }
                    };
                    for res_prop_raw in res_props_raw.iter() {
                        let res_prop = ResPropertyWrapper(res_prop_raw.to_owned()).to_res_prop()?;
                        transient_resource.add_res_prop(res_prop);
                    }
                }
                _ => {

                }
            }
        }
        transient_resource.is_complete()?;
        Ok(DMResource::new(transient_resource))

    }
}
