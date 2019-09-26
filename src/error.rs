use crate::t1b1::T1B1;

pub type Result<T> = std::result::Result<T, TrinaryError>;

#[derive(Debug)]
pub enum TrinaryError {
    InvalidTryteChar,
    InvalidTritChar,
    InvalidT5B1Byte,
    InvalidTryteIndex,
    IncompatibleLength,
}

//impl std::convert::From<<S as std::convert::TryInto<T1B1>>::Error> for TrinaryError {
//
//}
