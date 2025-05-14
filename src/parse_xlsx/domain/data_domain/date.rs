
#[derive(Clone)]
pub struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: usize,
    pub(crate) epoch: Epoch
}

impl Date {
    pub(crate) fn new(day: u8, month: u8, year: usize, epoch: Epoch) -> Self {
        Date{
            day,
            month,
            year,
            epoch,
        }
    }

    pub fn extend_string(mut parsed: String, extend_length: usize) -> String {
        while parsed.len() < extend_length {
            parsed.insert(0, '0');
        }
        parsed
    }
}

#[derive(Clone, Debug)]
pub enum Epoch {
    BC,
    CE
}

pub struct TransientDate {
    pub(crate) day: Option<u8>,
    pub(crate) month: Option<u8>,
    pub(crate) year: Option<usize>,
    pub(crate) epoch: Option<Epoch>
}
impl TransientDate {
    pub(crate) fn new(day: Option<u8>, month: Option<u8>, year: Option<usize>) -> TransientDate {
        TransientDate{
            day,
            month,
            year,
            //todo: discern BC/CE
            epoch: Option::from(Epoch::CE),
        }
    }
}
