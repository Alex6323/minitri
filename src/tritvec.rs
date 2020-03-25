use crate::encodings::t1b1::T1B1;
use crate::encodings::Encoding;

pub struct TritVec<T: Encoding> {
    encoding: T,
}

/*
impl TritVec {
    pub fn default() -> Self {
        Self {
            encoding: T1B1::new(),
        }
    }
}
*/

impl<T: Encoding> TritVec<T> {
    pub fn new() -> Self {
        Self { encoding: T::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            encoding: T::with_capacity(capacity),
        }
    }

    /// NOTE: make sure to add as many trits as required by the encoding
    pub fn push(&mut self, trits: impl Into<T1B1>) {
        self.encoding.add(trits.into());
    }

    pub fn len(&self) -> usize {
        self.encoding.len()
    }
}
