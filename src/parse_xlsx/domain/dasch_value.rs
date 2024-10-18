use crate::parse_xlsx::domain::encoding::Encoding;
use crate::parse_xlsx::domain::permissions::Permissions;
use crate::parse_xlsx::errors::ExcelDataError;

pub struct DaschValue {
    pub value: String,
    pub permission: Option<Permissions>,
    pub encoding: Option<Encoding>,
    pub comment: Option<String>,
}

impl DaschValue {
    pub(crate) fn add_permission(&mut self,permission: Permissions) {
        self.permission = Some(permission);
    }
    pub(crate) fn add_encoding(&mut self, encoding: Encoding) {
    self.encoding = Some(encoding);
    }
    pub(crate) fn add_comments(&mut self, comment: String) {
        self.comment = Some(comment);
    }
}

impl DaschValue {
    pub(crate) fn new(value: String) -> Self {
        DaschValue {
            value,
            permission: None,
            encoding: None,
            comment: None,
        }
    }
}