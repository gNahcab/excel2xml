use serde_json::Value;
use crate::transfrom2datamodel::errors::DataModelError;

#[derive(Debug, PartialEq)]
pub struct Comment {
    // can be a Comment or a Comment
    language: String,
    comment: String,
}
pub(crate) struct CommentWrapper(pub(crate) (String, Value));
impl CommentWrapper {
    pub fn to_comment(&self) -> Result<Comment, DataModelError> {
        let (language, comment_raw) = self.0.to_owned();
        let comment = comment_raw.as_str().expect("comment must be a string");
        Ok(Comment { language, comment: comment.to_string() })
    }
}
