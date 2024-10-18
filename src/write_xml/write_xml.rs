use std::fs::File;
use simple_xml_builder::XMLElement;
use crate::json2datamodel::domain::data_model::DataModel;
use crate::json2datamodel::domain::object::ValueObject;
use crate::parse_xlsx::domain::data_container::DataContainer;
use crate::parse_xlsx::domain::data_resource::DataResource;
use crate::parse_xlsx::errors::ExcelDataError;

pub fn write_xml_example() {
    let file = File::create("sample.xml").unwrap();

    let mut person = XMLElement::new("person");
    person.add_attribute("id", "232");
    let mut name = XMLElement::new("name");
    name.add_text("Joe Schmoe");
    person.add_child(name);
    let mut age = XMLElement::new("age");
    age.add_text("24");
    person.add_child(age);
    let hobbies = XMLElement::new("hobbies");
    person.add_child(hobbies);

    person.write(file).unwrap();
}

pub fn write_xml(data_container: &DataContainer, data_model: &DataModel) -> Result<(), ExcelDataError> {
    let file = File::create("test.xml").unwrap();
    for resource in data_container.resources.iter() {
        let mut xml_res = XMLElement::new("resource");
        xml_res.add_attribute("id", &resource.id);
        xml_res.add_attribute("label", &resource.label);
        xml_res.add_attribute("permissions", &resource.res_permissions);
        if resource.bitstream.is_some() {
            xml_res.add_child(bitstream_child(&resource));
        }
        for (propname, value_field) in resource.propname_to_values.iter() {
            let property_object = &data_model.properties.iter().find(|property|property.name.eq(propname)).unwrap().object;
            let (xml_object, sub_xml_object) = xml_object_sub_object(property_object)?;
            let mut child = XMLElement::new(xml_object);
            for dasch_value in value_field.values.iter() {
                let mut sub_child = XMLElement::new(&sub_xml_object);
                if dasch_value.permission.is_some() {
                    child.add_attribute("permissions", dasch_value.permission.as_ref().unwrap());
                }
                if dasch_value.encoding.is_some() {
                    child.add_attribute("encoding", dasch_value.encoding.as_ref().unwrap());
                }
                if dasch_value.comment.is_some() {
                    child.add_attribute("comment", dasch_value.comment.as_ref().unwrap(), );
                }
                sub_child.add_text(&dasch_value.value);
            }


        }

    }

    /*

    // the resource
    let mut person = XMLElement::new("resource");
    // add to resource-level
    person.add_attribute("id", "geoarch_1234");
    person.add_attribute("label", "a geoarch resource");
    // the property
    let mut text_prop = XMLElement::new(":hasTextValue");
    text_prop.add_attribute("encoding", "UTF-8");
    // add value
    text_prop.add_text("Super Text here");
    person.add_child(text_prop);

    person.write(file).unwrap();

     */
    todo!()
}


fn xml_object_sub_object(value_object: &ValueObject) -> Result<(String, String), ExcelDataError> {
    match value_object {
        ValueObject::ListValue => {
            Ok(("list-prop".to_string(), "list".to_string()))
        }
        ValueObject::TextValue => {
            Ok(("text-prop".to_string(), "text".to_string()))
        }
        ValueObject::DateValue => {
            Ok(("date-prop".to_string(), "date".to_string()))
        }
        ValueObject::UriValue => {
            Ok(("uri-prop".to_string(), "uri".to_string()))
        }
        ValueObject::GeonameValue => {
            todo!()
        }
        ValueObject::DecimalValue => {
            todo!()
        }
        ValueObject::ColorValue => {
            todo!()
        }
        ValueObject::IntValue => {
            todo!()
        }
        ValueObject::BooleanValue => {
            todo!()
        }
        ValueObject::TimeValue => {
            todo!()
        }
        ValueObject::Representation => {
            todo!()
        }
        ValueObject::ResLinkValue(k) => {
            todo!()
        }
    }
}

fn bitstream_child(resource: &DataResource) -> XMLElement {
    let mut bitstream = XMLElement::new("bitstream");
    if resource.bitstream_permissions.is_some() {
        bitstream.add_attribute("permissions",resource.bitstream_permissions.as_ref().unwrap())
    }
    bitstream
}