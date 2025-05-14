use crate::parse_dm::errors::DataModelError;

#[derive(Debug, Clone, PartialEq)]
pub enum  GUIElement {
    RICHTEXT,
    SIMPLETEXT,
    LIST,
    DATE,
    SEARCHBOX,
    GEONAMES,
    TEXTAREA,
}

pub fn to_gui_element(gui_element: &String) -> Result<GUIElement, DataModelError> {
    match gui_element.to_lowercase().as_str() {
        "richtext" => {Ok(GUIElement::RICHTEXT)}
        "simpletext" => {Ok(GUIElement::SIMPLETEXT)}
        "list" => {Ok(GUIElement::LIST)}
        "date" => {Ok(GUIElement::DATE)}
        "searchbox" => {Ok(GUIElement::SEARCHBOX)}
        "geonames" => {Ok(GUIElement::GEONAMES)}
        "textarea" => {Ok(GUIElement::TEXTAREA)}
        _ => Err(DataModelError::ParsingError(format!("Gui-Element '{}' not found in Enum. Add first.", gui_element)))
    }
}