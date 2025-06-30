use hcl::{Attribute, Block};
use crate::parse_hcl::errors::HCLDataError;

pub trait Wrapper {
    fn get_single_label(&self) -> Result<String, HCLDataError>;
    fn no_blocks(&self) -> Result<(), HCLDataError> ;
    fn no_attributes(&self) -> Result<(), HCLDataError>;
    fn blocks(&self) -> Vec<&Block>;
    fn attributes(&self) -> Vec<&Attribute>;
}
impl Wrapper for Block  {
    fn get_single_label(&self) -> Result<String, HCLDataError> {
        // returns the output variable of the method, this is the variable with which we can call the column/row of data
        // error if no label or more than one label was found
        if self.labels.len() == 0 {
            return Err(HCLDataError::InputError(format!("no label found for method: '{:?}'", self)));
        }
        if self.labels.len() > 1 {
            return Err(HCLDataError::InputError(format!("this block '{}' should have one label but has more than one: '{:?}'",self.identifier, self.labels)));
        }
        return Ok(self.labels.get(0).unwrap().as_str().to_string());
    }

    fn no_blocks(&self) -> Result<(), HCLDataError> {
        // check that no block exists within this method-block
        let blocks: Vec<&Block> = self.blocks();
        if blocks.len() != 0 {
            return Err(HCLDataError::InputError(format!("found those blocks '{:?}' in method '{:?}', but blocks are not allowed.", blocks, self)));
        }
        Ok(())
    }
    fn no_attributes(&self) -> Result<(), HCLDataError> {
        // check that no attribute exists within this method-attribute
        let attributes: Vec<&Attribute> = self.attributes();
        if attributes.len() != 0 {
            return Err(HCLDataError::InputError(format!("found those attributes '{:?}' in method '{:?}', but attributes are not allowed.", attributes, self)));
        }
        Ok(())
    }
    fn blocks(&self) -> Vec<&Block> {
        return self.body.blocks().collect();
    }
    fn attributes(&self) -> Vec<&Attribute> {
        return self.body.attributes().collect();
    }
}
