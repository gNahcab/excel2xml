use crate::parse_xlsx::domain::encoding::Encoding;
use crate::parse_xlsx::domain::permissions::Permissions;

pub struct DaschValue {
    pub value: String,
    pub permission: Permissions,
    pub encoding: Option<Encoding>,
    pub comment: Option<String>,
}


impl DaschValue {
    pub(crate) fn add_encoding(&mut self, encoding: Encoding) {
    self.encoding = Some(encoding);
    }
    pub(crate) fn add_comment(&mut self, comment: String) {
        self.comment = Some(comment);
    }
}

impl DaschValue {
    pub(crate) fn new(value: String, permission: Permissions) -> Self {
        DaschValue {
            value,
            permission,
            encoding: None,
            comment: None,
        }
    }
}