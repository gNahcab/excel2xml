use crate::parse_xlsx::domain::prop_name::PropName;

#[derive(Clone)]
pub struct RawData {
    prop_name: PropName,
    header: String,
    pos: usize,
    pub(crate) raw_value: String,
}

impl RawData {
    pub(crate) fn new(header: String, prop_name: PropName, data: String, pos: usize) -> Self {
        RawData{
            prop_name,
            header,
            pos,
            raw_value: data,
        }
    }
}