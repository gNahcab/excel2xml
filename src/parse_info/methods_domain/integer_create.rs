use hcl::{Block, Expression};
use crate::expression_trait::ExpressionTransform;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::methods_domain::number_trait::NumberTrait;
use crate::parse_info::methods_domain::step::{Step, WrapperStep};
use crate::parse_info::methods_domain::wrapper_trait_block::Wrapper;

pub struct WrapperIntegerCreate(pub(crate) Block);

#[derive(Clone, Debug)]
pub struct IntegerCreate {
    pub output: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub start: usize,
    pub step: Step
}

impl IntegerCreate {
    fn new(transient_integer_create: TransientIntegerCreate) -> IntegerCreate {
        IntegerCreate{
            output: transient_integer_create.output,
            prefix: transient_integer_create.prefix,
            suffix: transient_integer_create.suffix,
            start: transient_integer_create.start.unwrap(),
            step: transient_integer_create.step.unwrap(),
        }
    }
}
#[derive(Debug)]
struct TransientIntegerCreate {
    prefix: Option<String>,
    suffix: Option<String>,
    start: Option<usize>,
    step: Option<Step>,
    output: String
}

impl TransientIntegerCreate {
    fn new(output: String) -> Self {
        TransientIntegerCreate{
            prefix: None,
            suffix: None,
            start: None,
            step: None,
            output
        }
    }
    fn add_prefix(&mut self, prefix: String) -> Result<(), HCLDataError> {
        if self.prefix.is_some() {
            return Err(HCLDataError::InputError(format!("integer-create-method: multiple prefix: First: '{}', Second: '{}'", self.prefix.as_ref().unwrap(), prefix)));
        }
        self.prefix = Some(prefix);
        Ok(())
    }
    fn add_suffix(&mut self, suffix: String) -> Result<(), HCLDataError> {
        if self.suffix.is_some() {
            return Err(HCLDataError::InputError(format!("integer-create-method: multiple suffix: First: '{}', Second: '{}'", self.suffix.as_ref().unwrap(), suffix)));
        }
        self.suffix = Some(suffix);
        Ok(())
    }
    fn add_step(&mut self, step: Step) -> Result<(), HCLDataError> {
        if self.step.is_some() {
            return Err(HCLDataError::InputError(format!("integer-create-method: multiple step: First: '{:?}', Second: '{:?}'", self.step.as_ref().unwrap(), step)));
        }
        self.step = Some(step);
        Ok(())
    }
    fn add_start(&mut self, start: usize) -> Result<(), HCLDataError> {
        if self.start.is_some() {
            return Err(HCLDataError::InputError(format!("integer-create-method: multiple start: First: '{}', Second: '{}'", self.start.as_ref().unwrap(), start)));
        }
        self.start = Some(start);
        Ok(())
    }
    fn is_complete(&self) -> Result<(), HCLDataError> {
        // prefix and suffix is facultative
        if self.start.is_none() {
            return Err(HCLDataError::InputError(format!("Integer-create-method: Cannot find 'start' in '{:?}'", self)));
        }
        if self.step.is_none() {
            return Err(HCLDataError::InputError(format!("Integer-create-method: Cannot find 'step' in '{:?}'", self)));
        }
        Ok(())
    }
}

impl WrapperIntegerCreate {
    pub(crate) fn to_integer_create_method(&self, output: String) -> Result<IntegerCreate, HCLDataError> {
        self.0.no_blocks()?;
        let mut transient_structure = TransientIntegerCreate::new(output);
        for attribute in self.0.body.attributes() {
            match attribute.key.as_str() {
                "prefix" => {
                    transient_structure.add_prefix(attribute.expr.to_string_2()?)?;
                }
                "suffix" => {
                    transient_structure.add_suffix(attribute.expr.to_string_2()?)?;
                }
                "start" => {
                    let start = match attribute.expr {
                        Expression::Number(number) => { number
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("Integer-create-method: 'start' is not of type Number: '{}'", attribute.expr)));
                        }
                    };
                    let start = start.as_usize()?;
                    transient_structure.add_start(start)?
                }
                "step" => {
                    let step = WrapperStep(attribute.expr.to_string_2()?).to_step()?;
                    transient_structure.add_step(step)?;
                }
                _ => {
                    return Err(HCLDataError::ParsingError(format!("integer-create-method: found this unknown attribute '{:?}' in method '{:?}'.", attribute, self.0.labels)));
                }
            }
        }
        transient_structure.is_complete()?;
        Ok(IntegerCreate::new(transient_structure))
    }
}
