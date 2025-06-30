use std::num::ParseIntError;
use crate::parse_hcl::errors::HCLDataError;

#[derive(Clone, Debug)]
pub struct Step {
    pub step_rate: usize,
    pub step_method: StepMethod,
}

impl Step {
    fn new(step_method: StepMethod, step_rate: usize) -> Self {
        Step{ step_rate, step_method }
    }
}

#[derive(Clone, Debug)]
pub enum StepMethod {
    Plus,
    Multiplication
}
impl StepMethod {
    fn step_method(step_method: &str) -> Result<StepMethod, HCLDataError> {
        match step_method {
            "+" => {
                Ok(StepMethod::Plus)
            }
            "*" => {
                 Ok(StepMethod::Multiplication)
            }
            _ => {
                Err(HCLDataError::ParsingError(format!("Found unknown step-method '{}'. Add first.", step_method)))
            }
        }
    }
}

pub struct WrapperStep(pub(crate) String);

impl WrapperStep {
    pub(crate) fn to_step(&self) -> Result<Step, HCLDataError> {
        // first element: StepMethod
        // after first element: StepRate

        let (step_method, step_rate) = self.0.split_at(1);
        let step_method = StepMethod::step_method(step_method)?;
        let step_rate = match step_rate.to_owned().parse::<usize>() {
            Ok(step_rate) => {step_rate}
            Err(_) => {
                return Err(HCLDataError::ParseInt(format!("Couldn't parse step-rate '{}' to integer.", step_rate)))
            }
        };

        Ok(Step::new(step_method, step_rate))
    }
}

