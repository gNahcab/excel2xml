use hcl::Expression;
use crate::expression_trait::ExpressionTransform;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::{HeaderMethods, HeaderValue};
use crate::parse_hcl::methods_domain::date_pattern::{DatePattern, WrapperDatePattern};
use crate::parse_hcl::methods_domain::date_type::DateType;
use crate::parse_hcl::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperToDateMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureToDateMethod{
    output: String,
    input: Option<HeaderValue>,
    date_type: Option<String>,
    date_pattern: Vec<DatePattern>,
}

impl TransientStructureToDateMethod {
    fn new(output: String) -> TransientStructureToDateMethod {
        TransientStructureToDateMethod{
            output,
            input: None,
            date_type: None,
            date_pattern: vec![],
        }
    }
    fn add_input(&mut self, input: Expression) -> Result<(), HCLDataError> {
        if self.input.is_some() {
            return Err(HCLDataError::ParsingError(format!("error in to_date-method '{:?}'. 'input'-attribute multiple times provided", self)));
        }
        let input_header_value = input.to_header_value()?;
        self.input = Option::from(input_header_value);
        Ok(())
    }

    fn add_date_type(&mut self, date_type: String) -> Result<(), HCLDataError> {
        if self.date_type.is_some() {
            return Err(HCLDataError::ParsingError(format!("error in to_date-method '{:?}'. 'date_type'-attribute multiple times provided", self))); }
        self.date_type = Option::from(date_type);
        Ok(())
    }
    fn add_date_pattern(&mut self, date_pattern: DatePattern) {
        self.date_pattern.push(date_pattern);
    }
    fn is_consistent(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::ParsingError(format!("error in to_date-method '{:?}'. 'input'-attribute not provided", self)));
        }
        if self.date_type.is_none() {
            return Err(HCLDataError::ParsingError(format!("error in to_date-method '{:?}'. 'date_type'-attribute not provided", self)));
        }
        if self.date_pattern.is_empty() {
            return Err(HCLDataError::ParsingError(format!("error in to_date-method '{:?}'. 'pattern'-attribute not provided", self)));
        }
        Ok(())
    }
}
impl WrapperToDateMethod {
    pub fn to_date_method(&self) -> Result<ToDateMethod, HCLDataError> {
        let mut transient_structure: TransientStructureToDateMethod = TransientStructureToDateMethod::new(self.0.get_output()?);
        let blocks = self.0.blocks();
        if blocks.len() == 0 {
            return Err(HCLDataError::ParsingError(format!("'to_date'-method {:?} should have one or more 'pattern'-blocks, but zero were found", transient_structure.output)))
        }
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_owned())?;
                }
                "calendar_type" => {
                    transient_structure.add_date_type(attribute.expr.to_string_2()?)?;
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found this unknown attribute '{:?}' in method '{:?}'.", attribute, transient_structure.output)));
                }
            }

        }
        for block in blocks {
            match block.identifier.as_str() {
                "pattern" => {
                    let date_pattern = WrapperDatePattern(block.to_owned()).to_pattern()?;
                    transient_structure.add_date_pattern(date_pattern);
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("found unknown block-identifier in 'to_date'-method: {:?}", transient_structure.output)));
                }
            }
        }
        transient_structure.is_consistent()?;
        let to_date_method = ToDateMethod::new(transient_structure)?;
        Ok(to_date_method)
    }
}
#[derive(Debug, Clone)]
pub struct ToDateMethod{
    pub output: String,
    pub input: HeaderValue,
    pub date_type: DateType,
    pub date_patterns: Vec<DatePattern>
}


impl ToDateMethod {
    fn new(transient_structure: TransientStructureToDateMethod) -> Result<ToDateMethod, HCLDataError> {
        let date_type: DateType = DateType::date_type(transient_structure.date_type.unwrap())?;
        Ok(ToDateMethod{
            output: transient_structure.output,
            input: transient_structure.input.unwrap(),
            date_type,
            date_patterns: transient_structure.date_pattern,
        })
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
    use hcl::block;
    use crate::parse_hcl::methods_domain::to_date_method::WrapperToDateMethod;

    #[test]
    fn test_replace_method() {
        let block = block!(to_date "to_date"{
            input = 3
            date_type = "Gregorian" // or "Julian"
        });

        let result = WrapperToDateMethod(block.to_owned()).to_date_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
