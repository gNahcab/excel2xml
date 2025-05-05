use simple_xml_builder::XMLElement;

fn prop_default() -> XMLElement {
    /*
    <permissions id="prop-default">
        <allow group="UnknownUser">V</allow>
        <allow group="KnownUser">V</allow>
        <allow group="Creator">CR</allow>
        <allow group="ProjectAdmin">CR</allow>
        </permissions>
    */
    let mut prop_default = XMLElement::new("permissions");
    let id = "prop-default";
    prop_default.add_attribute("id", id);
    let allow_u = allow("UnknownUser", "V");
    prop_default.add_child(allow_u);
    let allow_k = allow("KnownUser", "V");
    prop_default.add_child(allow_k);
    let allow_cr = allow("Creator", "CR");
    prop_default.add_child(allow_cr);
    let allow_pa = allow("ProjectAdmin", "CR");
    prop_default.add_child(allow_pa);
    prop_default
}

fn prop_restricted() -> XMLElement {
    /*
    <permissions id="prop-restricted">
        <allow group="Creator">M</allow>
        <allow group="ProjectAdmin">D</allow>
     */
    let mut prop_restricted = XMLElement::new("permissions");
    let id = "prop-restricted";
    prop_restricted.add_attribute("id", id);
    let allow_cr = allow("Creator", "M");
    prop_restricted.add_child(allow_cr);
    let allow_pa = allow("ProjectAdmin", "D");
    prop_restricted.add_child(allow_pa);
    prop_restricted
}
fn res_restricted() -> XMLElement {
    /*
        <permissions id="res-restricted">
            <allow group="Creator">M</allow>
            <allow group="ProjectAdmin">D</allow>
        </permissions>
     */
    let mut res_restricted = XMLElement::new("permissions");
    let id = "res-restricted";
    res_restricted.add_attribute("id", id);
    let allow_cr = allow("Creator", "M");
    res_restricted.add_child(allow_cr);
    let allow_pa = allow("ProjectAdmin", "D");
    res_restricted.add_child(allow_pa);
    res_restricted
}
fn res_default() -> XMLElement {
    /*
        <permissions id="res-default">
            <allow group="UnknownUser">V</allow>
            <allow group="KnownUser">V</allow>
            <allow group="Creator">CR</allow>
            <allow group="ProjectAdmin">CR</allow>
        </permissions>
     */
    let mut res_default = XMLElement::new("permissions");
    let id = "res-default";
    res_default.add_attribute("id", id);
    let allow_u = allow("UnknownUser", "V");
    res_default.add_child(allow_u);
    let allow_k = allow("KnownUser", "V");
    res_default.add_child(allow_k);
    let allow_cr = allow("Creator", "CR");
    res_default.add_child(allow_cr);
    let allow_pa = allow("ProjectAdmin", "CR");
    res_default.add_child(allow_pa);
    res_default
}

fn allow(group: &str, value: &str) -> XMLElement {
    let mut group_allow = XMLElement::new("allow");
    group_allow.add_attribute("group", group);
    group_allow.add_text(value);
    group_allow
}
fn default_xml_header() -> XMLElement  {
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
    todo!()
}
