pub struct SupplementHCL {
    pub(crate) supplement_type: SupplementType,
    header: String
}
pub enum SupplementType {
    Authorship,
    License,
    CopyrightHolder,
    Encoding(String),
    Comment(String),
    Bitstream,
    BitstreamPermission,
    Permissions(String),
}
impl SupplementHCL {
    pub(crate) fn new(supplement_type: SupplementType, header: String) -> Self {
        Self{ supplement_type, header }
    }
}