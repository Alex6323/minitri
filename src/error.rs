pub type Result<T> = std::result::Result<T, TrinaryError>;

#[derive(Debug)]
pub enum TrinaryError {
    InvalidTryteChar,
    InvalidTritChar,
    InvalidT5B1Byte,
    InvalidTryteIndex,
    IncompatibleLength,
}
