
pub struct SpecialPropnames {
    pub resource_header:  [String; 3],
    pub bitstream: [String; 2],
    pub properties: [String; 3],
}

impl SpecialPropnames {
    pub fn new() -> SpecialPropnames {
        SpecialPropnames{
            resource_header: ["id".to_string(),"label".to_string(),"permissions".to_string()],
            bitstream: ["bitstream".to_string(), "permissions".to_string()],
            properties: ["permissions".to_string(), "comment".to_string(), "encoding".to_string()] }
    }
}