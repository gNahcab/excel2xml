use std::cmp::PartialEq;
use serde_json::Value;
use crate::transfrom2datamodel::domain::label::Label;
use crate::transfrom2datamodel::domain::object::{Object, ObjectWrapper};
use crate::transfrom2datamodel::errors::DataModelError;
use crate::transfrom2datamodel::errors::DataModelError::ParsingError;

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    name: String,
    ontology_name: String,
    object: Object,
    labels: Vec<Label>,
    gui_element: String,
    h_list: Option<String>
}

impl Property {
    fn new(transient_property: TransientProperty) -> Property {
        Property {
            name: transient_property.name.unwrap(),
            ontology_name: transient_property.ontology_name.unwrap(),
            object: transient_property.object.unwrap(),
            labels: transient_property.labels,
            gui_element: transient_property.gui_element.unwrap(),
            h_list: transient_property.hlist,
        }
    }
}

pub struct TransientProperty {
    name: Option<String>,
    ontology_name: Option<String>,
    object: Option<Object>,
    labels: Vec<Label>,
    gui_element: Option<String>,
    hlist: Option<String>,
}

impl TransientProperty {
    fn new() -> Self {
        TransientProperty{
            name: None,
            ontology_name: None,
            object: None,
            labels: vec![],
            gui_element: None,
            hlist: None,
        }
    }
    pub(crate) fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub(crate) fn add_object(&mut self, object: Object) {
        self.object = Some(object);
    }
    pub(crate) fn add_gui_element(&mut self, gui_element: String) {
        self.gui_element = Some(gui_element);
    }
    pub(crate) fn add_hlist(&mut self, hlist: String) {
            self.hlist = Some(hlist);
        }
    pub(crate) fn add_onto_name(&mut self, onto_name: String) {
        self.ontology_name = Option::from(onto_name);
    }
    pub(crate) fn is_complete(&self) -> Result<(), DataModelError> {
        // it is complete if a name, an object and a gui_element exist
        if self.name.is_none() {
            return Err(DataModelError::ParsingError(format!("name is missing for property with labels: {:?}", self.labels)))
        }
        if self.object.is_none() {
            return Err(DataModelError::ParsingError(format!("object is missing for property with name: {:?}", self.name)))
        }
        if self.gui_element.is_none() {
            return Err(DataModelError::ParsingError(format!("gui_element is missing for property with name: {:?}", self.name)))

        }
        // special cases:
        //1 object is a ListValue: condition: a hlist must exist as well
        if self.object.as_ref().unwrap() == &Object::ListValue && self.hlist.is_none() {
            return Err(ParsingError(format!("hlist is missing for ListValue with name: {:?}", self.name)))
        }
        Ok(())
    }
}

pub struct PropertyWrapper (pub(crate) Value);

impl PropertyWrapper {
    pub(crate) fn to_property(&self, onto_name: String) -> Result<Property, DataModelError> {
        let prop_obj = self.0.as_object().expect("property of ontology must be an object");
        let mut transient_property = TransientProperty::new();
        transient_property.add_onto_name(onto_name.to_owned());
        for (key, value) in prop_obj.iter() {
            match key.as_str() {
                "name"=> {
                    let name = value.as_str().expect("name of property value must be a string");
                    transient_property.add_name(name.to_string());
                }
                "object"=>{
                    let object_raw = value.as_str().expect("property of object must be a string");
                    let object = ObjectWrapper(object_raw.to_string()).to_object(onto_name.to_owned())?;

                    transient_property.add_object(object);
                }
                "gui_element"=> {
                    let gui_element = value.as_str().expect("gui_element must be a string");
                    transient_property.add_gui_element(gui_element.to_string());
                }
                "gui_attributes" => {
                    // if hlist exists, else not interesting
                    let gui_attributes = value.as_object().expect("gui_attributes must be an object");
                    let hlist_name = gui_attributes.get("hlist");
                    if hlist_name.is_some() {
                        transient_property.add_hlist(hlist_name.unwrap().as_str().unwrap().to_string());
                    }
                }
                &_ => {
                    // else continue
                } }

        }
        transient_property.is_complete()?;
        Ok({Property::new(transient_property)})
    }
}




