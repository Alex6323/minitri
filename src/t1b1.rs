use crate::error::{self, TrinaryError};
use crate::t3b1::T3B1;
use crate::t5b1::T5B1;
use crate::t9b2::T9B2;
use crate::trit::Trit;
use crate::trits::Encoding;
use crate::tryte::Tryte;

use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct T1B1(Vec<Trit>);

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

impl Encoding for T1B1 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add(&mut self, trits: T1B1) {
        for trit in trits.0 {
            self.push_internal(trit);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> TryFrom<&'a str> for T1B1 {
    type Error = TrinaryError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut trits = vec![];

        for c in s.chars() {
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

impl From<T1B1> for T3B1 {
    fn from(trits: T1B1) -> T3B1 {
        if trits.len() % 3 != 0 {
            panic!("Stay calm, this panic is only of temporary nature! <boarding escape pod>");
        }

        let mut trytes = T3B1::with_capacity(trits.len() / 3);

        for i in (0..trits.0.len()).step_by(3) {
            let a = trits.0[i + 0].value();
            let b = trits.0[i + 1].value();
            let c = trits.0[i + 2].value();

            let v = a + b * 3 + c * 9;

            trytes.push_internal(Tryte(v));
        }

        trytes
    }
}

impl From<T1B1> for T5B1 {
    fn from(trits: T1B1) -> T5B1 {
        if trits.len() % 5 != 0 {
            panic!("Stay calm, this panic is only of temporary nature! <boarding escape pod>");
        }

        let mut bytes = vec![0u8; trits.len() / 5];
        let mut j = 0;

        for i in (0..trits.0.len()).step_by(5) {
            let a = (trits.0[i + 0].value()
                + trits.0[i + 1].value() * 3
                + trits.0[i + 2].value() * 9
                + trits.0[i + 3].value() * 27
                + trits.0[i + 4].value() * 81) as i16;

            let a = if a < 0 { a + 243 } else { a };

            bytes[j] = a as u8;

            j += 1;
        }

        T5B1::from_bytes(&bytes)
    }
}

impl From<T1B1> for T9B2 {
    fn from(trits: T1B1) -> T9B2 {
        if trits.len() % 9 != 0 {
            panic!("Stay calm, this panic is only of temporary nature! <boarding escape pod>");
        }

        let mut bytes = vec![0u8; (trits.len() / 9) * 2];
        let mut j = 0;

        for i in (0..trits.0.len()).step_by(9) {
            let a = (trits.0[i + 0].value()
                + 3 * trits.0[i + 1].value()
                + 9 * trits.0[i + 2].value()) as i16;
            let b = (trits.0[i + 3].value()
                + 3 * trits.0[i + 4].value()
                + 9 * trits.0[i + 5].value()) as i16;
            let c = (trits.0[i + 6].value()
                + 3 * trits.0[i + 7].value()
                + 9 * trits.0[i + 8].value()) as i16;

            let a = if a < 0 { a + 27 } else { a };
            let b = if b < 0 { b + 27 } else { b };
            let c = if c < 0 { c + 27 } else { c };

            bytes[j + 0] = (a * 8 + c % 8) as u8;
            bytes[j + 1] = (b * 8 + c / 8) as u8;

            j += 2;
        }

        T9B2::from_bytes(&bytes)
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
