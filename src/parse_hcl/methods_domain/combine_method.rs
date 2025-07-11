use hcl::{Block, Expression};
use crate::expression_trait::ExpressionTransform;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::{HeaderMethods, HeaderValue};
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperCombineMethod (pub(crate) Block);
#[derive(Debug)]
struct TransientStructureCombineMethod {
    input: Option<Vec<HeaderValue>>,
    output: String,
    separator: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl TransientStructureCombineMethod {
    fn new(output: String) -> TransientStructureCombineMethod {
        TransientStructureCombineMethod{
            input: None,
            output,
            separator: None,
            prefix: None,
            suffix: None,
        }
    }
    pub(crate) fn add_input(&mut self, input: Vec<HeaderValue>) -> Result<(), HCLDataError> {
        if self.input.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple input-attributes", self)));
        }
        if input.len() != 2 {
            return Err(HCLDataError::ParsingError(format!("error in combine-method: Input array '{:?}' doesn't have exactly two entries.", input)))
        }
        self.input = Option::from(input);
        Ok(())
    }
    pub(crate) fn add_separator(&mut self, separator: String) -> Result<(), HCLDataError>{
        if self.separator.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple separator-attributes", self)));
        }
        self.separator = Option::from(separator);
        Ok(())
    }
    pub(crate) fn add_prefix(&mut self, prefix: String) -> Result<(), HCLDataError>{
        if self.prefix.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple prefix-attributes", self)));
        }
        self.prefix = Option::from(prefix);
        Ok(())
    }
    pub(crate) fn add_suffix(&mut self, suffix: String) -> Result<(), HCLDataError>{
        if self.suffix.is_some() {
            return Err(HCLDataError::ParsingError(format!("method: '{:?}' has multiple suffix-attributes", self)));
        }
        self.suffix = Option::from(suffix);
        Ok(())
    }

    pub(crate) fn is_consistent(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::ParsingError(format!("combine-method: '{:?}' doesn't have an input-attribute provided", self)));
        }
        if self.separator.is_none() {
            return Err(HCLDataError::ParsingError(format!("combine-method: '{:?}' doesn't have a separator provided", self)));
        }
        // suffix, prefix are optional
        Ok(())
    }
}


impl WrapperCombineMethod {

    pub(crate) fn to_combine_method(&self) -> Result<CombineMethod, HCLDataError> {
        let mut transient_structure = TransientStructureCombineMethod::new(self.0.get_output()?);
        self.0.no_blocks()?;
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    let input_vec = parse_input(attribute.expr().to_owned())?;
                    transient_structure.add_input(input_vec)?;
                }
                "separator" => {
                    transient_structure.add_separator(attribute.expr.to_string_2()?)?;
                }
                "prefix" => {
                    transient_structure.add_prefix(attribute.expr.to_string_2()?)?;
                }
                "suffix" => {
                    transient_structure.add_suffix(attribute.expr.to_string_2()?)?;
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found this unknown attribute '{:?}' in method '{:?}'.", attribute, transient_structure.output)));
                }
            }

        }
        transient_structure.is_consistent()?;
        let combine_method = CombineMethod::new(transient_structure);
        Ok(combine_method)
    }
}

fn parse_input(input: Expression) -> Result<Vec<HeaderValue>, HCLDataError>{
    match input {
        Expression::Array(array) => {
            let str_vec:Vec<HeaderValue> = array.iter().map(|expr|expr.to_header_value().unwrap()).collect();

            Ok(str_vec)
        }
        _ => {
            Err(HCLDataError::ParsingError(format!("combine-methods: '{:?}' input-attribute is not an array", input)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct CombineMethod{
    pub input: Vec<HeaderValue>,
    pub output: String,
    pub separator: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}


impl CombineMethod {
    fn new(transient_structure: TransientStructureCombineMethod) -> CombineMethod {
        CombineMethod {
        input: transient_structure.input.unwrap(),
        output: transient_structure.output,
        separator: transient_structure.separator,
        prefix: transient_structure.prefix,
        suffix: transient_structure.suffix,
        }
    }
    pub(crate) fn is_correct(&self) -> Result<(), HCLDataError> {
        let identical_input: Vec<&HeaderValue> = self.input.iter().filter(|value|value.is_equal(&self.output)).collect();
        if identical_input.len() != 0 {
            return Err(HCLDataError::ParsingError(format!("at least one input-String is identical with the output-String, which is forbidden: '{:?}'", self.input)));
        }
        Ok(())

    }
}

#[cfg(test)]
mod test {

    use hcl::block;
    use crate::parse_hcl::methods_domain::combine_method::WrapperCombineMethod;

    #[test]
    fn test_combine_method() {
        let block = block!(combine "new_ID"{
            input = [0, "lower"]//"{$0}{$lower}"
            separator = "_"
            prefix = "BIZ_"
            suffix = "_ZIP"});
        let result = WrapperCombineMethod(block.to_owned()).to_combine_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}