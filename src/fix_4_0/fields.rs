trait Field {
    fn tag(&self) -> u8;
}

#[derive(Debug)]
pub enum PossDupFlag {
    YES,
    NO,
}

#[derive(Debug)]
pub struct Name {
    field: f32,
}

impl Field for PossDupFlag {
    fn tag(&self) -> u8 {
        return 43;
    }
}

impl PossDupFlag {
    fn value(&self) -> char {
        match *self {
            PossDupFlag::YES => 'Y',
            PossDupFlag::NO => 'N',
        }
    }
}