use std::collections::HashMap;
use calamine::Data;
use crate::parse_xlsx::domain::prop_name::PropName;
use crate::parse_xlsx::domain::raw_data::RawData;

#[derive(Clone)]
pub struct RawResourceData {
    pub(crate) res_name: String,
    pub raw_data: Vec<(PropName, RawData)>
}
impl RawResourceData {
    fn new(resource_name: String) -> Self {
        RawResourceData {
            res_name: resource_name,
            raw_data: Default::default()}
    }
    fn add_value(&mut self, prop_name: PropName, raw_data: RawData) {
        self.raw_data.push((prop_name, raw_data));
    }
}
pub struct ResourceDataWrapper (pub(crate) Vec<Data>);

impl ResourceDataWrapper {
    pub(crate) fn to_raw_resource_data(self, pos_to_headers_propname: HashMap<usize, (String, PropName)>, res_name: String) -> RawResourceData {
        let mut raw_resource_data = RawResourceData::new(res_name);
        for (pos, data) in self.0.iter().enumerate() {
                match pos_to_headers_propname.get(&pos) {
                    None => {
                        // continue
                    }
                    Some((header, propname)) => {
                        let raw_data = RawData::new(header.to_owned(), propname.to_owned(), data.to_string(), pos);
                        raw_resource_data.add_value(propname.to_owned(), raw_data);
                    }
                }
        }
        raw_resource_data
    }
}

