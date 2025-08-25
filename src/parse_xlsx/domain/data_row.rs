use calamine::Data;

#[derive(Clone, Debug)]
pub struct DataRow {
    pub row: Vec<Vec<String>>
}
impl DataRow {
    pub fn new() -> DataRow {
        DataRow{ row: vec![] }
    }
    pub fn add_data(&mut self, data: Vec<String>) {
        self.row.push(data);
    }
}
