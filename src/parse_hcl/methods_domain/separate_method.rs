use crate::expression_trait::ExpressionTransform;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::header_value::{HeaderMethods, HeaderValue};
use crate::parse_hcl::wrapper_trait::Wrapper;

#[derive(Debug, Clone)]
pub struct SeparateMethod {
    pub(crate) input: HeaderValue,
    pub(crate) outputs: Vec<String>,
    pub(crate) separator: String
}

impl SeparateMethod {
    fn new(transient_separate_method: TransientSeparateMethod) -> SeparateMethod {
        Self{
            input: transient_separate_method.input.unwrap(),
            outputs: transient_separate_method.outputs.unwrap(),
            separator: transient_separate_method.separator.unwrap(),
        }
    }
}

#[derive(Debug)]
struct TransientSeparateMethod {
    input: Option<HeaderValue>,
    outputs: Option<Vec<String>>,
    separator: Option<String>,
}

impl TransientSeparateMethod {
    pub(crate) fn is_complete(&self) -> Result<(), HCLDataError> {
        if self.input.is_none() {
            return Err(HCLDataError::InputError(format!("Separate-Method: missing 'input' value, transient-structure: {:?}", self)));
        }
        if self.outputs.is_none() {
            return Err(HCLDataError::InputError(format!("Separate-Method: missing 'outputs' value, transient-structure: {:?}", self)));
        }
        if self.separator.is_none() {
            return Err(HCLDataError::InputError(format!("Separate-Method: missing 'separator' value, transient-structure: {:?}", self)));
        }
        Ok(())
    }
}

impl TransientSeparateMethod {
    fn new() -> Self {
        Self{
            input: None,
            outputs: None,
            separator: None,
        }
    }
    pub(crate) fn add_input(&mut self, input: HeaderValue) -> Result<(), HCLDataError>{
        if self.input.is_some() {
            return Err(HCLDataError::InputError(format!("Defined multiple input for separate-method. First input: '{:?}', next input: '{:?}'", self.input.as_ref().unwrap(), input)));
        }
        self.input = Option::Some(input);
        Ok(())
    }
    pub(crate) fn add_outputs(&mut self, outputs: Vec<String>) -> Result<(), HCLDataError>{
        if self.outputs.is_some() {
            return Err(HCLDataError::InputError(format!("Defined multiple outputs for separate-method. First outputs: '{:?}', next outputs: '{:?}'", self.outputs.as_ref().unwrap(), outputs)));
        }
        self.outputs = Some(outputs);
        Ok(())
    }
    pub(crate) fn add_separator(&mut self, separator: String) -> Result<(), HCLDataError>{
        if self.separator.is_some() {
            return Err(HCLDataError::InputError(format!("Defined multiple separator for separate-method. First separator: '{}', next separator: '{}'", self.separator.as_ref().unwrap(), separator)));
        }
        self.separator = Option::Some(separator);
        Ok(())
    }
}
#[derive(Debug)]
pub struct WrapperSeparateMethod(pub(crate) hcl::Block);

impl WrapperSeparateMethod {
    pub(crate) fn to_separate_method(&self) -> Result<SeparateMethod, HCLDataError> {
        let mut transient_structure = TransientSeparateMethod::new();
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    let header_value = attribute.expr.to_owned().to_header_value()?;
                    transient_structure.add_input(header_value)?;
                }
                "outputs" => {
                    let outputs_expr = attribute.expr.to_vec()?;
                    let mut outputs = vec![];
                    for output_expr in outputs_expr {
                        let output = output_expr.to_string_2()?;
                        if outputs.contains(&output) {
                            return Err(HCLDataError::InputError(format!("Defined same outputs-header '{}' multiple times for separate-method: '{:?}'", output, &self)));
                        }
                        outputs.push(output);

                    }
                    transient_structure.add_outputs(outputs)?;
                }
                "separator" => {
                    let separator = attribute.expr.to_string_2()?;
                    transient_structure.add_separator(separator)?;
                }
                &_ => {
                    return Err(HCLDataError::InputError(format!("Unknown Input: '{}'.Cannot find this attribute in '{:?}'", attribute.key, self)))
                } }
        }
        transient_structure.is_complete()?;
        Ok(SeparateMethod::new(transient_structure))
    }
}
