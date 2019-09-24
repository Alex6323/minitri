use crate::error::{self, TrinaryError};
use crate::t3b1::{ToT3B1, T3B1};
use crate::t5b1::{ToT5B1, T5B1};
use crate::t9b2::{ToT9B2, T9B2};
use crate::trit::Trit;
use crate::trits::RawTrits;
use crate::tryte::Tryte;

use std::convert::TryInto;

#[derive(Debug)]
pub struct T1B1(Vec<Trit>);

pub trait ToT1B1 {
    fn to_t1b1(&self) -> T1B1;
}

impl T1B1 {
    pub(crate) fn from_sbytes(sbytes: Vec<i8>) -> Self {
        let mut trits = Vec::with_capacity(sbytes.len());

        for trit in sbytes {
            trits.push(Trit(trit));
        }

        Self(trits)
    }

    pub fn push<T>(&mut self, trit: T) -> Result<(), TrinaryError>
    where
        T: TryInto<Trit>,
        TrinaryError: From<T::Error>,
    {
        self.0.push(trit.try_into()?);

        Ok(())
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub(crate) fn push_internal(&mut self, trit: Trit) {
        self.0.push(trit);
    }
}

impl<'a> TryInto<T1B1> for &'a str {
    type Error = TrinaryError;

    fn try_into(self) -> Result<T1B1, Self::Error> {
        let mut trits = vec![];

        for c in self.chars() {
            trits.push(c.try_into()?);
        }

        Ok(T1B1(trits))
    }
}

impl std::fmt::Display for T1B1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for trit in &self.0 {
            trit.fmt(f)?;
        }
        Ok(())
    }
}

impl ToT3B1 for T1B1 {
    fn to_t3b1(&self) -> error::Result<T3B1> {
        if self.len() % 3 != 0 {
            return Err(TrinaryError::IncompatibleLength);
        }

        let mut trytes = T3B1::with_capacity(self.len() / 3);

        for i in (0..self.0.len()).step_by(3) {
            let a = self.0[i + 0].value();
            let b = self.0[i + 1].value();
            let c = self.0[i + 2].value();

            let v = a + b * 3 + c * 9;

            trytes.push_internal(Tryte(v));
        }

        Ok(trytes)
    }
}

impl ToT5B1 for T1B1 {
    fn to_t5b1(&self) -> error::Result<T5B1> {
        if self.len() % 5 != 0 {
            // TODO: fill remainder with zeros
            return Err(TrinaryError::IncompatibleLength);
        }

        let mut bytes = vec![0u8; self.len() / 5];
        let mut j = 0;

        for i in (0..self.0.len()).step_by(5) {
            let a = (self.0[i + 0].value()
                + self.0[i + 1].value() * 3
                + self.0[i + 2].value() * 9
                + self.0[i + 3].value() * 27
                + self.0[i + 4].value() * 81) as i16;

            let a = if a < 0 { a + 243 } else { a };

            bytes[j] = a as u8;

            j += 1;
        }

        Ok(T5B1::from_bytes(&bytes))
    }
}

impl ToT9B2 for T1B1 {
    fn to_t9b2(&self) -> error::Result<T9B2> {
        if self.len() % 9 != 0 {
            return Err(TrinaryError::IncompatibleLength);
        }

        let mut bytes = vec![0u8; (self.len() / 9) * 2];

        for i in (0..self.0.len()).step_by(9) {
            let a = (self.0[i + 0].value() + 3 * self.0[i + 1].value() + 9 * self.0[i + 2].value())
                as i16;
            let b = (self.0[i + 3].value() + 3 * self.0[i + 4].value() + 9 * self.0[i + 5].value())
                as i16;
            let c = (self.0[i + 6].value() + 3 * self.0[i + 7].value() + 9 * self.0[i + 8].value())
                as i16;

            let a = if a < 0 { a + 27 } else { a };
            let b = if b < 0 { b + 27 } else { b };
            let c = if c < 0 { c + 27 } else { c };

            bytes[2 * i + 0] = (a * 8 + c % 8) as u8;
            bytes[2 * i + 1] = (b * 8 + c / 8) as u8;
        }

        Ok(T9B2::from_bytes(&bytes))
    }
}

impl RawTrits for T1B1 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add_trits(&mut self, trits: T1B1) -> error::Result<()> {
        for trit in trits.0 {
            self.push_internal(trit);
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn resize_with_push_and_pop() {
        let mut trits = T1B1::new();
        assert_eq!(0, trits.len());

        trits.push('1').expect("error pushing trit");
        trits.push('-').expect("error pushing trit");
        assert_eq!(2, trits.len());

        trits.pop();
        assert_eq!(1, trits.len());
    }

    #[test]
    fn initialize_from_str() {
        let trits: T1B1 = "10-01-110".try_into().expect("error parsing trit sequence");

        assert_eq!(9, trits.len());
    }
}
