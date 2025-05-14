use crate::parse_info::methods_domain::combine_method::CombineMethod;
use crate::parse_info::methods_domain::create_method::old_CreateMethod;
use crate::parse_info::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
use crate::parse_info::methods_domain::replace_method::ReplaceMethod;
use crate::parse_info::methods_domain::to_date_method::ToDateMethod;

#[derive(Debug)]
pub enum Method {
    CombineMethod(CombineMethod),
    ReplaceMethod(ReplaceMethod),
    ToDateMethod(ToDateMethod),
    LowerMethod(LowerMethod),
    UpperMethod(UpperMethod),
    InventMethod(old_CreateMethod),
}