use std::collections::HashMap;
use crate::parse_hcl::domain::prop_supplement::PropSupplement;

pub trait Wrapper<T>
{
    fn insert_or_append(&mut self, pos: &usize, value: T);
}

impl<T> Wrapper<T> for HashMap<usize, Vec<T>>
where T: ,
{
    fn insert_or_append(&mut self, pos: &usize, value: T) {
        if self.contains_key(pos) {
            self.get_mut(pos).unwrap().push(value);
        } else {
            let mut values = vec![];
            values.push(value);
            self.insert(pos.to_owned(), values);
        }
    }
}

