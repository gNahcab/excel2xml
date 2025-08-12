use std::collections::HashMap;
use std::fs::File;
use simple_xml_builder::XMLElement;
use crate::parse_dm::domain::data_model::DataModel;
use crate::parse_dm::domain::object::ValueObject;
use crate::parse_xlsx::domain::dasch_value::DaschValue;
use crate::parse_xlsx::domain::dasch_value_field::DaschValueField;
use crate::parse_xlsx::domain::data_container::DataContainer;
use crate::parse_xlsx::domain::instance::Instance;
use crate::write_xml::errors::WriteXMLError;

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
pub fn write_xml(data_containers: &Vec<DataContainer>, data_model: &DataModel) -> Result<(), WriteXMLError> {
    let mut knora = XMLElement::new("knora");
    let hash_to_id_authors = create_authorship_hash_map(data_containers);
    add_authorship_element(&mut knora, &hash_to_id_authors);
    add_default_knora_attributes(&mut knora);
    add_shortcode_default_ontology_attributes(&mut knora, &data_model.shortcode.to_owned(), &data_model.shortname);

    let new_path = new_path(&format!("data_{}_{}",data_model.shortcode, data_model.shortname));
    let file = File::create(new_path.as_str())?;
    for data_container in data_containers {
        let restype = ":".to_string() + data_container.res_name.as_str();
        add_resources(&data_container.resources, &hash_to_id_authors, restype, &data_model, &mut knora);
        // todo allow here returning single files
        // let path = new_path(&data_container.res_name);
    }
    knora.write(file)?;
    println!("wrote-file {:?}", new_path);
    Ok(())
}

fn create_authorship_hash_map(data_containers: &Vec<DataContainer>) -> HashMap<String, (String, Vec<String>)> {
    // authorship-ids have to be created here, because schema.xsd of dsp-tools allows only to add it at the beginning of the xml-file
    let mut hash_to_id_authors: HashMap<String, (String, Vec<String>)> = HashMap::new();
    for container in data_containers {
        for res in &container.resources {
            let authorship = res.authorship.as_ref();
            let mut authorship = if authorship.is_none() {
                continue;
            } else {
                authorship.unwrap().to_vec()
            };
            authorship.sort();
            let hash_string = authorship.join("");
            match hash_to_id_authors.get(&hash_string) {
                Some(_) => {}
                None => {
                    let id = format!("authorship_{}", hash_to_id_authors.len() + 1);
                    hash_to_id_authors.insert(hash_string, (id.to_owned(), authorship.to_vec()));
                }
            };
        }
    }
    hash_to_id_authors
}

fn finish_knora_and_write(mut knora: &mut XMLElement, hash_to_id_authors: HashMap<String, (String, Vec<String>)>, file: File) -> Result<(), WriteXMLError> {
    Ok(())
}

fn add_resources(resources: &Vec<Instance>, hash_to_id_authors: &HashMap<String, (String, Vec<String>)>, restype: String, data_model: &&DataModel, knora: &mut XMLElement) {
    for resource in resources {
        let mut xml_res = xml_resource(&resource, hash_to_id_authors, &restype);
        add_values(&resource.dasch_value_fields, &mut xml_res, data_model);
        knora.add_child(xml_res);
    }
}

fn add_values(dasch_value_fields: &Vec<DaschValueField>, mut xml_res: &mut XMLElement, data_model: &&DataModel) {
    for dasch_value_field in dasch_value_fields.iter() {
        let property_object = &data_model.properties.iter().find(|property| property.name.eq(&dasch_value_field.propname)).unwrap();
        let (xml_object, sub_xml_object) = xml_object_sub_object(&property_object.object);
        let mut prop_container = XMLElement::new(xml_object);
        let propname = ":".to_string() + dasch_value_field.propname.as_str();
        prop_container.add_attribute("name", propname);
        if property_object.object.eq(&ValueObject::ListValue) {
            prop_container.add_attribute("list", property_object.h_list.as_ref().unwrap());
        }
        for dasch_value in dasch_value_field.values.iter() {
            let prop_value = value(dasch_value, &sub_xml_object);
            prop_container.add_child(prop_value);
        }
        xml_res.add_child(prop_container);
    }
}

fn value(dasch_value: &DaschValue, sub_xml_object: &String) -> XMLElement {
    let mut prop_value = XMLElement::new(&sub_xml_object);
    /*

    if parse_info.set_permissions {
        // is necessary to avoid an xml file with unnecessary permissions
    }
     */
    if dasch_value.permission.is_some() {
        prop_value.add_attribute("permissions", dasch_value.permission.unwrap());
    }
    if dasch_value.comment.is_some() {
        prop_value.add_attribute("comment", dasch_value.comment.as_ref().unwrap(), );
    }
    if dasch_value.encoding.is_some() {
        prop_value.add_attribute("encoding", dasch_value.encoding.as_ref().unwrap());
    }
    prop_value.add_text(&dasch_value.value);
    prop_value
}

fn xml_resource(resource: &Instance, hash_to_id_authors:  &HashMap<String, (String, Vec<String>)>, restype: &String) -> XMLElement {
    let mut xml_res = XMLElement::new("resource");
    xml_res.add_attribute("label", &resource.label);
    xml_res.add_attribute("id", &resource.id);
    xml_res.add_attribute("restype", &restype);

    if resource.res_permissions.is_some() {
        xml_res.add_attribute("permissions", &resource.res_permissions.unwrap());
    }
    if resource.bitstream.is_some() {
        xml_res.add_child(bitstream_child(
            &resource,  hash_to_id_authors,
        ));
    }
    xml_res
}

fn add_authorship_element(knora: &mut XMLElement, hash_to_id_authorship_group: &HashMap<String, (String, Vec<String>)>) {
    /*
    <authorship id="authorship_1">
        <author>Lukas Rosenthaler</author>
        </authorship>
   */
    if hash_to_id_authorship_group.is_empty() {
        return;
    }
    for (id, authorship_group) in hash_to_id_authorship_group.values() {
        let mut authorship = XMLElement::new("authorship");
        authorship.add_attribute("id", id);
        for member in authorship_group {
            let mut author = XMLElement::new("author");
            author.add_text(member);
            authorship.add_child(author);
        }
        knora.add_child(authorship);
    }
}

fn new_path(res_name: &String) -> String {
    res_name.to_owned() + ".xml"
}

fn xml_object_sub_object(value_object: &ValueObject) -> (String, String) {
    match value_object {
        ValueObject::ListValue => {
            ("list-prop".to_string(), "list".to_string())
        }
        ValueObject::TextValue => {
            ("text-prop".to_string(), "text".to_string())
        }
        ValueObject::DateValue => {
            ("date-prop".to_string(), "date".to_string())
        }
        ValueObject::UriValue => {
            ("uri-prop".to_string(), "uri".to_string())
        }
        ValueObject::GeonameValue => {
            ("geoname-prop".to_string(), "geoname".to_string())
        }
        ValueObject::DecimalValue => {
            ("decimal-prop".to_string(), "decimal".to_string())
        }
        ValueObject::ColorValue => {
            ("color-prop".to_string(), "color".to_string())
        }
        ValueObject::IntValue => {
            ("integer-prop".to_string(), "integer".to_string())
        }
        ValueObject::BooleanValue => {
            ("boolean-prop".to_string(), "boolean".to_string())
        }
        ValueObject::TimeValue => {
            ("time-prop".to_string(), "time".to_string())
        }
        ValueObject::Representation => {
            ("resptr-prop".to_string(), "resptr".to_string())
        }
        ValueObject::ResLinkValue(k) => {
            ("resptr-prop".to_string(), "resptr".to_string())
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


fn bitstream_child(resource: &Instance, hash_to_id_authors: &HashMap<String, (String, Vec<String>)>) -> XMLElement {
    // hash vector entries
    let mut values =  resource.authorship.as_ref().unwrap().to_vec();
    // sort so that we don't have any duplicates, like Vec<a,b,c> and Vec<b, a, c> etc.
    values.sort();
    let hash_id = resource.authorship.as_ref().unwrap().join("");
    let (id, _) =  hash_to_id_authors.get(&hash_id).unwrap();
    let mut bitstream = XMLElement::new("bitstream");
    bitstream.add_attribute("copyright-holder", &resource.copyright_holder.as_ref().unwrap());
    bitstream.add_attribute("authorship-id", id);
    bitstream.add_attribute("license", &resource.license.as_ref().unwrap().rdfh_str());
    bitstream.add_text(resource.bitstream.as_ref().unwrap());
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


