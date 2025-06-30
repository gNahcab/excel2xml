use hcl::Expression;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::{HeaderValue};
use crate::parse_hcl::methods_domain::number_trait::NumberTrait;
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;

#[derive(Debug)]
pub struct WrapperLowerUpperMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureLowerUpperMethod{
    output: String,
    input: Option<HeaderValue>,
}

impl TransientStructureLowerUpperMethod {
    fn new(name: String) -> TransientStructureLowerUpperMethod {
        TransientStructureLowerUpperMethod {
            output: name,
            input: None,
        }
    }
    pub(crate) fn add_input(&mut self, input: Expression) -> Result<(), HCLDataError> {
        if self.input.is_some() {
            return Err(HCLDataError::ParsingError(format!("found more than one 'input'-declaration in method '{:?}'.", self.output)));
        }
        let input_header_value = match input {
            Expression::Number(value) => {
                HeaderValue::Number(value.as_u8()?)
            }
            Expression::String(value) => {
                HeaderValue::Name(value)
            }
            _ => {
                return Err(HCLDataError::ParsingError(format!("error in lower-upper-method '{:?}'. 'input'-expression can only be of type 'String' or 'Number' but found this: '{:?}'", self, input)));
            }
        };
        self.input = Option::from(input_header_value);
        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::ParsingError(format!("found no 'input'-declaration in method '{:?}'.", self.output)));
        }
        Ok(())
    }
}
impl WrapperLowerUpperMethod {
    pub fn to_lower_method(&self) -> Result<LowerMethod, HCLDataError> {
        let transient_structure = get_transient_structure(&self)?;
        let lower_method = LowerMethod::new(transient_structure.output, transient_structure.input.unwrap());
        Ok(lower_method)
    }
    pub fn to_upper_method(&self) -> Result<UpperMethod, HCLDataError> {
        let transient_structure = get_transient_structure(&self)?;
        let upper_method = UpperMethod::new(transient_structure.output, transient_structure.input.unwrap());
        Ok(upper_method)
    }
}

fn get_transient_structure(wrapper: &WrapperLowerUpperMethod) -> Result<TransientStructureLowerUpperMethod, HCLDataError> {
    wrapper.0.no_blocks()?;
    let mut transient_structure: TransientStructureLowerUpperMethod = TransientStructureLowerUpperMethod::new( wrapper.0.get_output()?);
    for attribute in wrapper.0.attributes() {
        match attribute.key.as_str() {
            "input" => {
                transient_structure.add_input(attribute.expr.to_owned())?;
            }
            _ => {
                return Err(HCLDataError::ParsingError(format!("found this unknown attribute '{:?}' in method '{:?}'.", attribute, transient_structure.output)));
            } }

    }
    transient_structure.is_complete()?;
    Ok(transient_structure)
}
#[derive(Debug, Clone)]
pub struct LowerMethod{
pub output: String,
    pub input: HeaderValue,
}


impl LowerMethod {
    fn new(output: String, input: HeaderValue) -> LowerMethod {
        LowerMethod{ output, input }
    }
    pub(crate) fn is_correct(&self) -> Result<(), HCLDataError> {
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct UpperMethod{
    pub output: String,
    pub input: HeaderValue,
}


impl UpperMethod {
    fn new(output: String, input: HeaderValue) -> UpperMethod {
        UpperMethod{ output, input }
    }
    pub(crate) fn is_correct(&self) -> Result<(), HCLDataError> {
        if self.input.is_equal(&self.output) {
            return Err(HCLDataError::ParsingError(format!("method has the same in- and output-String, which is forbidden: '{:?}'", self.input)));
        }
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use crate::parse_hcl::methods_domain::lower_upper_method::WrapperLowerUpperMethod;

    #[test]
    fn test_lower_method() {
        let block = hcl::block!(lower "lower"{
            // lower the b-variable
            input = 1
       });
        let result = WrapperLowerUpperMethod(block).to_lower_method();
        assert!(result.is_ok());
    }
    #[test]
    fn test_upper_method() {
        let block = hcl::block!(upper "upper"{
        input = "upper"
   });
        let result = WrapperLowerUpperMethod(block).to_upper_method();
        assert!(result.is_ok());
    }}
