use crate::parse_hcl::methods_domain::combine_method::CombineMethod;
use crate::parse_hcl::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
use crate::parse_hcl::methods_domain::replace_method::ReplaceMethod;
use crate::parse_hcl::methods_domain::to_date_method::ToDateMethod;

#[derive(Debug)]
pub enum Method {
    CombineMethod(CombineMethod),
    ReplaceMethod(ReplaceMethod),
    ToDateMethod(ToDateMethod),
    LowerMethod(LowerMethod),
    UpperMethod(UpperMethod),
}