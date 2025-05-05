use regex::Captures;
use crate::parse_info::errors::HCLDataError;
use crate::parse_info::methods_domain::date_pattern::DatePattern;
use crate::parse_info::methods_domain::date_type::DateType;

pub struct DatePeriod {

}

impl DatePeriod {
    fn new(transient: TransientDatePeriod) -> DatePeriod {
        todo!()
    }
}

pub struct DatePeriodWrapper (pub(crate) String);

struct TransientDatePeriod(, &DatePattern, &DateType);

impl TransientDatePeriod {
    fn new(p0: Captures, p1: &DatePattern, p2: &DateType) -> _ {
        todo!()
    }
}

impl DatePeriodWrapper {
    /// five cases:
    /// case 1: year-month-day || day-month-year || month-day-year
    /// case 2: year-month || month-year
    /// case 4: year
    /// case 3: case 1 or case 2 with month written as word (e.g. Jan 1991)
    /// case 5: symbols, words used to convey date is BC or CE -> not implemented
    pub fn to_date_period(&self, date_patterns: &Vec<DatePattern>, date_type: &DateType) -> Result<DatePeriod, HCLDataError> {
        for i in 0..date_patterns.len() {
            let date_pattern = date_patterns.get(i).unwrap();
            let date_period: Option<DatePeriod> = self.date_period(date_pattern, date_type)?;
            if date_period.is_some() {
                return Ok(date_period.unwrap())
            }
        }
        Err(HCLDataError::ParsingError(format!("cannot parse value '{:?}' to a date with existing patterns.", self.0)))
    }
    fn date_period(&self, date_pattern: &DatePattern, date_type: &DateType) -> Result<Option<DatePeriod>, HCLDataError> {
        let regex = date_pattern.to_regex()?;
        let caps = regex.captures(self.1.as_str());
        if caps.is_none() {
            return Ok(None)
        }
        let mut transient = TransientDatePeriod::new(caps.unwrap(), date_type.to_owned());

        transient.complete_dates()?;
        Ok(Some(DatePeriod::new(transient)))
    }
}