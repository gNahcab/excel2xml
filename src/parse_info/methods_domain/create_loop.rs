use std::fmt::format;
use hcl::{BlockLabel, Expression};
use crate::hcl_info::errors::HCLDataError;
use crate::hcl_info::methods_domain::number_trait::NumberTrait;
use crate::hcl_info::methods_domain::step::{Step, WrapperStep};
use crate::hcl_info::methods_domain::wrapper_trait::Wrapper;

#[derive(Debug, Clone)]
pub enum Create {
    IntCreate(IntCreate)
}

#[derive(Clone, Debug)]
pub struct IntCreate {
    pub start: usize,
    pub step: Step,
}

impl IntCreate {
    fn new(transient_int_create: TransientIntCreate) -> Self {
        IntCreate {
            start: transient_int_create.start.unwrap(),
            step: transient_int_create.step.unwrap(),
        }
    }
}
#[derive(Debug)]
struct TransientIntCreate {
    start: Option<usize>,
    step: Option<Step>,
}


impl TransientIntCreate {
    fn new() -> Self {
        TransientIntCreate {
            start: None,
            step: None,
        }
    }
    pub(crate) fn is_complete(&self) -> Result<(), HCLDataError> {
        if self.start.is_none() {
            return Err(HCLDataError::ParsingError(format!("Couldn't find start in create: {:?}", self)));
        }
        if self.step.is_none() {
            return Err(HCLDataError::ParsingError(format!("Couldn't find step in create: {:?}", self)));

        }
        Ok(())
    }

    fn add_start(&mut self, start: usize) -> Result<(), HCLDataError>{
        if self.start.is_some() {
            return Err(HCLDataError::ParsingError(format!("Found multiple times 'start' in integer-create. First: {}, second: {}", self.start.unwrap(), start)));
        }
        self.start = Some(start);
        Ok(())
    }
    fn add_step(&mut self, step: Step) -> Result<(), HCLDataError>{
        if self.step.is_some() {
            return Err(HCLDataError::ParsingError(format!("Found multiple times 'step' in integer-create. First: {:?}, second: {:?}", self.step.as_ref().unwrap(), step)));
        }
        self.step = Some(step);
        Ok(())
    }
}
#[derive(Debug)]
pub struct WrapperCreateLoop(pub(crate) hcl::Block);


impl WrapperCreateLoop {
    pub(crate) fn to_integer(&self) -> Result<Create, HCLDataError> {
        let mut transient_int_create = TransientIntCreate::new();
        for attribute in &self.0.attributes() {
            match attribute.key.as_str() {
               "start" => {
                   match attribute.expr {
                       Expression::Number(start) => {
                           let start= start.as_usize()?;
                           transient_int_create.add_start(start)?;
                       }
                       _ => {
                           return Err(HCLDataError::InputError(format!("Unknown expression for start of invent-loop: '{}'. Add first.", attribute.key.as_str())))
                       }
                   }
               }
                "step" => {
                    match attribute.expr() {
                        Expression::String(step) => {
                            let step = WrapperStep(step.to_owned()).to_step()?;
                            transient_int_create.add_step(step)?;
                        }
                        _ => {
                            return Err(HCLDataError::InputError(format!("Unknown expression for step of invent-loop: '{}'. Add first.", attribute.key.as_str())))
                        }
                    }
                }
                _ => {
                    return Err(HCLDataError::InputError(format!("Unknown attribute for invent-loop: '{}'. Add first.", attribute.key.as_str())))
                }
            }
        }
        transient_int_create.is_complete()?;
        let int_create = IntCreate::new(transient_int_create);
        Ok(Create::IntCreate(int_create))
    }
}

