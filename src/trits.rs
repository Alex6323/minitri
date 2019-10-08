use crate::error;
use crate::t1b1::T1B1;
use crate::t3b1::T3B1;

use std::convert::TryInto;

pub trait Encoding {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn add(&mut self, trits: T1B1); // use bytes
}

pub struct Trits<T: Encoding = T1B1> {
    enc: T,
}

impl Trits {
    pub fn default() -> Self {
        Self { enc: T1B1::new() }
    }
}

impl<T: Encoding> Trits<T> {
    pub fn new() -> Self {
        Self { enc: T::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            enc: T::with_capacity(capacity),
        }
    }

    /// NOTE: make sure to add as many trits as required by the encoding
    pub fn add_trits<S>(&mut self, trits: S) -> error::Result<()>
    where
        S: TryInto<T1B1>,
        S::Error: std::fmt::Debug,
    {
        if let Ok(trits) = trits.try_into() {
            self.enc.add(trits);
        } else {
            return Err(error::TrinaryError::InvalidTritChar);
        };

        Ok(())
    }

    /// NOTE: make sure to add as many trytes as required by the encoding
    pub fn add_trytes<S>(&mut self, trytes: S) -> error::Result<()>
    where
        S: TryInto<T3B1>,
        S::Error: std::fmt::Debug,
    {
        if let Ok(trytes) = trytes.try_into() {
            let trits: T1B1 = trytes.into();
            self.enc.add(trits);
        } else {
            return Err(error::TrinaryError::InvalidTryteChar);
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.enc.len()
    }
}
