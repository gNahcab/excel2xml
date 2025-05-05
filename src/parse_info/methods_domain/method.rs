use crate::hcl_info::methods_domain::combine_method::CombineMethod;
use crate::hcl_info::methods_domain::create_method::CreateMethod;
use crate::hcl_info::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
use crate::hcl_info::methods_domain::replace_method::ReplaceMethod;
use crate::hcl_info::methods_domain::to_date_method::ToDateMethod;

#[derive(Debug)]
pub enum Method {
    CombineMethod(CombineMethod),
    ReplaceMethod(ReplaceMethod),
    ToDateMethod(ToDateMethod),
    LowerMethod(LowerMethod),
    UpperMethod(UpperMethod),
    InventMethod(CreateMethod),
}