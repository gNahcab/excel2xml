#[derive(Clone, Debug)]
pub struct DataColumn {
    pub column: Vec<String>
}
impl DataColumn {
    pub fn new() -> Self {
        DataColumn { column: vec![] }
    }
    pub fn add_data(&mut self, data: String) {
        self.column.push(data);
    }
}
