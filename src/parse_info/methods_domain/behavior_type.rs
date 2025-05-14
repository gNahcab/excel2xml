use crate::parse_info::errors::HCLDataError;

#[derive(Debug, Clone)]
pub enum BehaviorType {
    Lazy,
    Greedy
}
impl BehaviorType {
    pub(crate) fn behavior_type(string: String) -> Result<BehaviorType, HCLDataError>{
        match string.as_str() {
            "greedy" => Ok(BehaviorType::Greedy),
            "lazy" => Ok(BehaviorType::Lazy),
            _ => Err(HCLDataError::ParsingError(format!("unknown value for 'behavior'-attribute: {:?}", string))),
        }

    }
}
