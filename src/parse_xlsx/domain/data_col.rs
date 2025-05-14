use calamine::Data;
#[derive(Clone, Debug)]
pub struct DataCol {
    pub head: String,
    pub col: Vec<String>
}

impl DataCol {
    pub fn new(col: Vec<String>, head: String) -> DataCol {
        DataCol{ head, col }
    }
}

pub struct TransientDataCol {
    pub head: Option<String>,
    pub col: Option<Vec<String>>

}
impl TransientDataCol {
    pub fn new() -> Self {
        TransientDataCol{ head: None, col: None }
    }
    pub fn add_head(&mut self, head: String) {
        self.head = Some(head);
    }
    pub fn add_col(&mut self, col: Vec<String>) {
        self.col = Some(col);
    }

}
