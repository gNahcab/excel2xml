use std::env::var;
use regex::Captures;
use crate::parse_hcl::errors::HCLDataError;
use crate::parse_hcl::methods_domain::date_pattern::DatePattern;
use crate::parse_hcl::methods_domain::date_type::DateType;
use crate::parse_xlsx::domain::data_domain::date::{Date, TransientDate};

pub struct DatePeriod {
    date1: Date,
    date2: Date,
    date_type: DateType
}

impl DatePeriod {
    fn new(date1: Date, date2: Date, date_type: DateType) -> DatePeriod {
        DatePeriod{
            date1,
            date2,
            date_type,
        }
    }
    pub fn to_date_period_string(&self) -> String {
        // calendar:epoch:yyyy-mm-dd:epoch:yyyy-mm-dd

        let calendar = format!("{:?}", self.date_type).to_uppercase();
        let epoch1 = &self.date1.epoch;

        let day1= Date::extend_string(self.date1.day.to_string(), 2);
        let month1 = Date::extend_string(self.date1.month.to_string(), 2);
        let year1 = Date::extend_string(self.date1.year.to_string(), 4);

        let epoch2 = &self.date2.epoch;
        let day2: String = Date::extend_string(self.date2.day.to_string(), 2);
        let month2: String = Date::extend_string(self.date2.month.to_string(), 2);
        let year2: String = Date::extend_string(self.date2.year.to_string(), 4);
        let date = format!("{}:{:?}:{}:{}:{}:{:?}:{}:{}:{}", calendar, epoch1, year1, month1, day1, epoch2, year2, month2, day2);

        date
    }
}

pub struct DatePeriodWrapper (pub(crate) String);

pub struct TransientDatePeriod {
    pub date1: Option<TransientDate>,
    pub date2: TransientDate,
    pub date_type: DateType
}


struct TransientDatePeriodWrapper (pub(crate)  TransientDate, Option<TransientDate>, DateType);

fn date_option(date1: &Option<TransientDate>, date2: &Date) -> Date {
    if date1.is_some() {
        let TransientDate { mut day, mut month,mut year, epoch} = date1.as_ref().unwrap();
        if date1.as_ref().unwrap().day.is_none() {
            day = Option::from(date2.day);
        }
        if date1.as_ref().unwrap().month.is_none() {
            month = Option::from(date2.month);
        }
        if date1.as_ref().unwrap().year.is_none() {
            year = Option::from(date2.year);
        }
        Date::new(day.unwrap(), month.unwrap(), year.unwrap(), epoch.to_owned().unwrap())
    } else {
        date2.to_owned()
    }
}
fn date(date2: TransientDate) -> Date {
    let TransientDate { mut day, mut month, year, epoch} = date2;
    if day.is_none() {
        day = Option::from(1u8);
    }
    if month.is_none() {
        month = Option::from(1u8);
    }
    Date::new(day.unwrap().to_owned(), month.unwrap().to_owned(), year.unwrap().to_owned(), epoch.unwrap().to_owned())
}

fn date1_date2(date1: Option<TransientDate>, date2: TransientDate) -> (Date, Date){
    // date 2 is mandatory
    let date2: Date = date(date2);
    // date 1 could exist or is a copy of date2
    let date1:Date = date_option(&date1, &date2);
    (date1, date2)
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
        let caps = regex.captures(self.0.as_str());
        if caps.is_none() {
            return Ok(None)
        }
        let caps = caps.unwrap();
        let day1 = _parse_day1(&caps);

        let month1 = _parse_month1(&caps, date_pattern)?;
        let year1 = _parse_year1(&caps);
        let day2:Option<u8> = _parse_day2(&caps);
        let month2 = _parse_month2(&caps, date_pattern)?;

        // year2 is mandatory
        let year2 = &caps["year2"].parse::<usize>().unwrap();
        // todo: add error message if this unwrap fails
        let year2 = Option::from(year2.to_owned());
        let transient_date1 = Option::from(TransientDate::new(day1, month1, year1));
        let transient_date2 = TransientDate::new(day2, month2, year2);
        let (date1, date2) = date1_date2(transient_date1, transient_date2);
        Ok(Some(DatePeriod::new(date1, date2, date_type.to_owned())))
    }
}

fn _parse_month2(caps: &Captures, date_pattern: &DatePattern) -> Result<Option<u8>, HCLDataError> {
    if caps.name("month2").is_some() {
        if date_pattern.date.month_word.unwrap() == true {
            let name = &caps["month2"].to_owned();
            Ok(Option::from(parse_month_to_number(name)?))
        } else {
            let number: &u8 = &caps["month2"].to_owned().parse::<u8>().unwrap();
            Ok(Option::from(number.to_owned()))
        }
    } else {
        Ok(None)
    }
}

fn parse_month_to_number(name: &String) -> Result<u8, HCLDataError>  {
    //todo: allow utf-8; at the moment only ASCII-Characters can be parsed
    let januars = ["January", "Jan", "Jän", "Janv",  "Januar", "Janvier", "Gennaio", "Genn",];
    let februarys = ["February", "Feb", "Februar", "Février", "Fevrier", "Févr", "Fevr", "Febbraio", "Febbr",];
    let marchs = ["March", "Mar", "März", "Mars", "Marzo", "Mar",];
    let aprils = ["April", "Apr", "Avril", "Aprile",];
    let mays = ["May", "Mai", "Maggio", "Magg",];
    let junes = ["June", "Juni", "Juin", "Giugno",];
    let julys = ["July", "Juli", "Juillet", "Juil", "Luglio",];
    let augusts = ["August","Aug", "Août", "Aout", "Agosto", "Ag",];
    let septembers = ["September", "Sept", "Septembre", "Settembre", "Sett",];
    let octobres = ["October", "Oct", "Octobre", "Ottobre", "Ott",];
    let novembers = ["November", "Nov", "Novembre", "Novembre",];
    let decembers = ["December", "Dec", "Dezember", "Dez", "Décembre", "Decembre", "Déc", "Dicembre",];

    if januars.contains(&&**name) {
        return Ok(1u8);
    }
    if februarys.contains(&&**name) {
        return Ok(2u8);
    }
    if marchs.contains(&&**name) {
        return Ok(3u8);
    }
    if aprils.contains(&&**name) {
        return Ok(4u8);
    }
    if mays.contains(&&**name) {
        return Ok(5u8);
    }
    if junes.contains(&&**name) {
        return Ok(6u8);
    }
    if julys.contains(&&**name) {
        return Ok(7u8);
    }
    if augusts.contains(&&**name) {
        return Ok(8u8);
    }
    if septembers.contains(&&**name) {
        return Ok(9u8);
    }
    if octobres.contains(&&**name) {
        return Ok(10u8);
    }
    if novembers.contains(&&**name) {
        return Ok(11u8);
    }
    if decembers.contains(&&**name) {
        return Ok(12u8);
    }
    return Err(HCLDataError::ParsingError(format!("couldn't find a matching month for '{:?}'. Either it is not a month or missing.", name)));
}

fn _parse_day2(caps: &Captures) -> Option<u8> {
    if caps.name("day2").is_some() {
        let number: &u8 = &caps["day2"].to_owned().parse::<u8>().unwrap();
        Option::from(number.to_owned())
    } else {
        None
    }
}

fn _parse_year1(caps: &Captures) -> Option<usize> {
    if caps.name("year1").is_some() {
        let number: &usize = &caps["year1"].to_owned().parse::<usize>().unwrap();
        Option::from(number.to_owned())
    } else {
        None
    }
}

fn _parse_month1(caps: &Captures, date_pattern: &DatePattern) -> Result<Option<u8>, HCLDataError> {
    if caps.name("month1").is_some() {
        if date_pattern.first_date.as_ref().unwrap().month_word.unwrap() == true {
            let name = &caps["month1"].to_owned();
            Ok(Option::from(parse_month_to_number(name)?))
        } else {
            let number: &u8 = &caps["month1"].to_owned().parse::<u8>().unwrap();
            Ok(Option::from(number.to_owned()))
        }
    } else {
        Ok(None)
    }
}

fn _parse_day1(caps: &Captures) -> Option<u8> {
    if caps.name("day1").is_some() {
        let number: &u8 = &caps["day1"].to_owned().parse::<u8>().unwrap();
        Option::from(number.to_owned())
    } else {
        None
    }
}
