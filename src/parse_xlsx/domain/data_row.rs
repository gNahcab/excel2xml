use calamine::Data;

#[derive(Clone, Debug)]
pub struct DataRow {
    pub row: Vec<String>
}
impl DataRow {
    pub fn new() -> DataRow {
        DataRow{ row: vec![] }
    }
    pub fn add_data(&mut self, data: String) {
        self.row.push(data);
    }
}
pub struct DataRowWrapper (pub(crate) Vec<Data>);

impl DataRowWrapper {
    pub(crate) fn to_data_row(&self) -> DataRow {
        let mut data_row = DataRow::new();
        for data_entry in self.0.iter() {
            // remove whitespace by calling "trim"
            data_row.add_data(data_entry.to_string().trim().to_owned());
        }
        data_row
    }
}