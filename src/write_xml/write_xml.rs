use std::fs::File;
use simple_xml_builder::XMLElement;
use crate::errors::Excel2XmlError;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::object::ValueObject;
use crate::parse_info::domain::parse_info::ParseInformation;
use crate::parse_xlsx::domain::data_container::DataContainer;
use crate::parse_xlsx::domain::instance::Instance;
use crate::parse_xlsx::errors::ExcelDataError;
use crate::write_xml::xml_permissions::add_default_permissions;

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

pub fn write_xml(data_container: &DataContainer, data_model: &DataModel, parse_info: &ParseInformation) -> Result<(), Excel2XmlError> {
    let path = new_path(&data_container.res_name);
    let file = File::create(path.as_str())?;
    let mut knora = XMLElement::new("knora");
    add_default_knora_attributes(&mut knora);
    add_shortcode_default_ontology_attributes(&mut knora, &data_model.shortcode.to_owned(), &data_model.shortname);
    if parse_info.set_permissions {
        add_default_permissions(&mut knora);
    }
    let restype = ":".to_string() + data_container.res_name.as_str();
    for resource in data_container.resources.iter() {
        let mut xml_res = XMLElement::new("resource");
        xml_res.add_attribute("label", &resource.label);
        xml_res.add_attribute("id", &resource.id);
        xml_res.add_attribute("restype", &restype);

        if parse_info.set_permissions {
            xml_res.add_attribute("permissions", &resource.res_permissions);
        }
        if resource.bitstream.is_some() {
            xml_res.add_child(bitstream_child(&resource));
        }
        for dasch_value_field in resource.dasch_value_fields.iter() {
            let property_object = &data_model.properties.iter().find(|property|property.name.eq(&dasch_value_field.propname)).unwrap().object;
            let (xml_object, sub_xml_object) = xml_object_sub_object(property_object)?;
            let mut prop_container = XMLElement::new(xml_object);
            let propname = ":".to_string() + dasch_value_field.propname.as_str();
            prop_container.add_attribute("name", propname);
            for dasch_value in dasch_value_field.values.iter() {
                let mut prop_value = XMLElement::new(&sub_xml_object);
                if parse_info.set_permissions {
                    // is necessary to avoid an xml file with unnecessary permissions
                    prop_value.add_attribute("permissions", dasch_value.permission);
                }
                if dasch_value.comment.is_some() {
                    prop_value.add_attribute("comment", dasch_value.comment.as_ref().unwrap(), );
                }
                if dasch_value.encoding.is_some() {
                    prop_value.add_attribute("encoding", dasch_value.encoding.as_ref().unwrap());
                }
                prop_value.add_text(&dasch_value.value);
                prop_container.add_child(prop_value);
            }
            xml_res.add_child(prop_container);
        }
        knora.add_child(xml_res);
    }
    knora.write(file)?;
    println!("wrote-file {:?}", path);
    Ok(())
}

fn new_path(res_name: &String) -> String {
    res_name.to_owned() + ".xml"
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
            Ok(("geoname-prop".to_string(), "geoname".to_string()))
        }
        ValueObject::DecimalValue => {
            Ok(("decimal-prop".to_string(), "decimal".to_string()))
        }
        ValueObject::ColorValue => {
            Ok(("color-prop".to_string(), "color".to_string()))
        }
        ValueObject::IntValue => {
            Ok(("integer-prop".to_string(), "integer".to_string()))
        }
        ValueObject::BooleanValue => {
            Ok(("boolean-prop".to_string(), "boolean".to_string()))
        }
        ValueObject::TimeValue => {
            Ok(("time-prop".to_string(), "time".to_string()))
        }
        ValueObject::Representation => {
            Ok(("resptr-prop".to_string(), "resptr".to_string()))
        }
        ValueObject::ResLinkValue(k) => {
            Ok(("resptr-prop".to_string(), "resptr".to_string()))
        }
    }
}
fn add_default_knora_attributes(knora: &mut XMLElement) {
    knora.add_attribute("xmlns", "https://dasch.swiss/schema");
    knora.add_attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance" );
    knora.add_attribute("xsi:schemaLocation", "https://dasch.swiss/schema https://raw.githubusercontent.com/dasch-swiss/dsp-tools/main/src/dsp_tools/resources/schema/data.xsd");
}

fn add_shortcode_default_ontology_attributes(knora: &mut XMLElement, shortcode: &String, default_ontology: &String) {
    knora.add_attribute("shortcode", shortcode);
    knora.add_attribute("default-ontology", default_ontology);
}



fn bitstream_child(resource: &Instance) -> XMLElement {
    let mut bitstream = XMLElement::new("bitstream");
    if resource.bitstream_permissions.is_some() {
        bitstream.add_attribute("permissions",resource.bitstream_permissions.as_ref().unwrap())
    }
    bitstream
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use simple_xml_builder::XMLElement;
    use crate::write_xml::write_xml::{add_default_knora_attributes, add_shortcode_default_ontology_attributes};
    use crate::write_xml::xml_permissions::add_default_permissions;

    #[test]
    fn test_default() {
        let file = File::create("sample.xml").unwrap();
        let mut knora = XMLElement::new("knora");
        add_default_knora_attributes(&mut knora);
        add_shortcode_default_ontology_attributes(&mut knora, &"0828".to_string(), &"biz".to_string());
        add_default_permissions(&mut knora);
        knora.write(file).unwrap();
        /*
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xmlns="https://dasch.swiss/schema"
            xsi:schemaLocation="https://dasch.swiss/schema https://raw.githubusercontent.com/dasch-swiss/dsp-tools/main/src/dsp_tools/resources/schema/data.xsd"
            shortcode="082E"
            default-ontology="rosetta">

            <!-- :permissions see https://docs.dasch.swiss/latest/DSP-API/05-internals/design/api-admin/administration/#permissions -->
            <permissions id="res-default">
                <allow group="UnknownUser">V</allow>
                <allow group="KnownUser">V</allow>
                <allow group="Creator">CR</allow>
                <allow group="ProjectAdmin">CR</allow>
            </permissions>
            <permissions id="res-restricted">
                <allow group="Creator">M</allow>
                <allow group="ProjectAdmin">D</allow>
            </permissions>
            <permissions id="prop-default">
                <allow group="UnknownUser">V</allow>
                <allow group="KnownUser">V</allow>
                <allow group="Creator">CR</allow>
                <allow group="ProjectAdmin">CR</allow>
            </permissions>
            <permissions id="prop-restricted">
                <allow group="Creator">M</allow>
                <allow group="ProjectAdmin">D</allow>
            </permissions>

         */
    }

}


