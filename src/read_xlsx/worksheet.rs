use calamine::{Data, Range};

#[derive(Clone)]
pub struct Worksheet {
    pub(crate) res_name: String,
    pub(crate) table: Range<Data>,
}

impl Worksheet {
    pub fn new(res_name: String, table: Range<Data>) -> Self {
        Worksheet{res_name, table}
    }
}
