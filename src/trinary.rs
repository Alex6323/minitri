use crate::encodings::t1b1::T1B1;
use crate::encodings::t3b1::T3B1;
use crate::encodings::Encoding;

pub struct Trinary<T: Encoding = T1B1> {
    encoding: T,
}

impl Trinary {
    pub fn default() -> Self {
        Self {
            encoding: T1B1::new(),
        }
    }
}

impl<T: Encoding> Trinary<T> {
    pub fn new() -> Self {
        Self { encoding: T::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            encoding: T::with_capacity(capacity),
        }
    }

    /// NOTE: make sure to add as many trits as required by the encoding
    pub fn add_trits<S>(&mut self, trits: S)
    where
        S: Into<T1B1>,
    {
        self.encoding.add(trits.into());
    }

    /// NOTE: make sure to add as many trytes as required by the encoding
    pub fn add_trytes<S>(&mut self, trytes: S)
    where
        S: Into<T3B1>,
    {
        let trits: T1B1 = trytes.into();
        self.encoding.add(trits);
    }

    pub fn len(&self) -> usize {
        self.encoding.len()
    }
}
