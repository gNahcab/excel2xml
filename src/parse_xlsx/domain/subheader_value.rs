use crate::parse_xlsx::domain::encoding::Encoding;
use crate::parse_xlsx::domain::permissions::Permissions;

pub struct SubheaderValues {
    pub permissions: Option<Vec<Permissions>>,
    pub encodings: Option<Vec<Encoding>>,
    pub comments: Option<Vec<String>>
}

impl SubheaderValues {
    pub(crate) fn new(transient_subheader_values: TransientSubheaderValues) -> Self {
        SubheaderValues {
            permissions: transient_subheader_values.permissions,
            encodings: transient_subheader_values.encodings,
            comments: transient_subheader_values.comments,
        }
    }
}

pub struct TransientSubheaderValues {
    permissions: Option<Vec<Permissions>>,
    encodings: Option<Vec<Encoding>>,
    comments: Option<Vec<String>>

}

impl TransientSubheaderValues {
    pub(crate) fn new() -> TransientSubheaderValues {
        TransientSubheaderValues{
            permissions: None,
            encodings: None,
            comments: None,
        }
    }
    pub(crate) fn add_permissions(&mut self, permissions: Vec<Permissions>) {
        self.permissions = Some(permissions);
    }
    pub(crate) fn add_encodings(&mut self, encodings: Vec<Encoding>) {
        self.encodings = Some(encodings);
    }
    pub(crate) fn add_comments(&mut self, comments: Vec<String>) {
        self.comments = Some(comments);
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.comments.is_none() & self.encodings.is_none() & self.comments.is_none()
    }
}