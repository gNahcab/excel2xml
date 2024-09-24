use calamine::Data;

pub struct DataRow {
    rows: Vec<String>
}
impl DataRow {
    fn new() -> DataRow {
        DataRow{ rows: vec![] }
    }
    fn add_data(&mut self, data: String) {
        self.rows.push(data);
    }
}
pub struct DataRowWrapper (pub(crate) Vec<Data>);

impl DataRowWrapper {
    fn to_data_row(&self) -> DataRow {
        let mut data_row = DataRow::new();
        for data_entry in self.0.iter() {
            data_row.add_data(data_entry.to_string());
        }
        data_row
    }
}