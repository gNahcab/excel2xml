use serde_json::Value;
use crate::transfrom2datamodel::errors::DataModelError;

#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    // can be a Label or a Comment
    language: String,
    label: String,
}
pub(crate) struct LabelWrapper(pub(crate) (String, Value));
impl LabelWrapper {
    pub fn to_label(&self) -> Result<Label, DataModelError> {
        let (language, label_raw) = self.0.to_owned();
        let label = label_raw.as_str().expect("label must be a string");
        Ok(Label { language, label: label.to_string() })
    }
}
