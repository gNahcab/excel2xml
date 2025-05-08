use calamine::Data;
#[derive(Clone, Debug)]
pub struct DataCol {
    pub col: Vec<String>

}

pub struct DataRow {
}
impl DataCol {
    pub fn new() -> DataCol {
        DataCol{ col: vec![] }
    }
    pub fn add_data(&mut self, data: String) {
        self.col.push(data);
    }
}
pub struct DataColWrapper (pub(crate) Vec<Data>);

impl DataColWrapper {
    pub(crate) fn to_data_col(&self) -> DataCol {
        /*
        let mut data_col = DataCol::new();
        for data_entry in self.0.iter() {
            // remove whitespace by calling "trim"
            data_col.add_data(data_entry.to_string().trim().to_owned());
        }
        data_col
         */
        todo!()
    }
}
