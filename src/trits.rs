use crate::error;
use crate::t1b1::{ToT1B1, T1B1};
use crate::t3b1::T3B1;

use std::convert::TryInto;

pub trait RawTrits {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn add_trits(&mut self, trits: T1B1) -> error::Result<()>;
}

pub struct Trits<T: RawTrits = T1B1> {
    raw: T,
}

impl Trits {
    pub fn default() -> Self {
        Self { raw: T1B1::new() }
    }
}

impl<T: RawTrits> Trits<T> {
    pub fn new() -> Self {
        Self { raw: T::new() }
    }

    /// NOTE: make sure to add as many trits as required by the encoding
    pub fn add_trits<S>(&mut self, trits: S)
    where
        S: TryInto<T1B1>,
        S::Error: std::fmt::Debug,
    {
        let trits = trits.try_into().expect("error parsing trits");

        self.raw
            .add_trits(trits)
            .expect("error adding trits to encoder");
    }

    /// NOTE: make sure to add as many trytes as required by the encoding
    pub fn add_trytes<S>(&mut self, trytes: S)
    where
        S: TryInto<T3B1>,
        S::Error: std::fmt::Debug,
    {
        let trytes = trytes.try_into().expect("error parsing trytes");
        let trits = trytes.to_t1b1();

        self.raw
            .add_trits(trits)
            .expect("error adding trytes to encoder");
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }
}
