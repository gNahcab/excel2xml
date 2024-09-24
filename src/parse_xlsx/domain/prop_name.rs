#[derive(Clone, Eq, Hash, PartialEq)]
pub enum PropName {
    ID,
    LABEL,
    PERMISSIONS,
    ARK,
    IRI,
    BITSTREAM,
    ProjectProp(String)
}