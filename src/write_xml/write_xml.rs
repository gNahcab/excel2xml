use std::fs::File;
use simple_xml_builder::XMLElement;
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

pub fn write_xml() {
    let file = File::create("sample.xml").unwrap();

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
}