use crate::error::{self, TrinaryError};
use crate::luts;
use crate::t1b1::T1B1;
use crate::t5b1::T5B1;
use crate::t9b2::T9B2;
use crate::trit::Trit;
use crate::trits::Encoding;
use crate::tryte::Tryte;

use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct T3B1(Vec<Tryte>);

impl T3B1 {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push<T>(&mut self, tryte: T) -> error::Result<()>
    where
        T: TryInto<Tryte>,
        TrinaryError: From<T::Error>,
    {
        self.0.push(tryte.try_into()?);
        Ok(())
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub(crate) fn push_internal(&mut self, tryte: Tryte) {
        self.0.push(tryte);
    }
}

impl From<T3B1> for T1B1 {
    fn from(trytes: T3B1) -> T1B1 {
        let mut trits = T1B1::with_capacity(3 * trytes.len());

        for tryte in &trytes.0 {
            let trits_in_tryte = luts::trits_from_tryteindex_internal(tryte.index());
            for trit in &trits_in_tryte {
                trits.push_internal(Trit(*trit));
            }
        }

        trits
    }
}

impl<'a> TryFrom<&'a str> for T3B1 {
    type Error = TrinaryError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut trytes = vec![];

        for c in s.chars() {
            trytes.push(c.try_into()?);
        }

        Ok(T3B1(trytes))
    }
}

impl std::fmt::Display for T3B1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for tryte in &self.0 {
            tryte.fmt(f)?;
        }
        Ok(())
    }
}

impl From<T3B1> for T5B1 {
    fn from(trytes: T3B1) -> T5B1 {
        if (trytes.len() * 3) % 5 != 0 {
            panic!("Stay calm, this panic is only of temporary nature! <boarding escape pod>");
        }

        let mut bytes = vec![0u8; (trytes.len() * 3) / 5];
        let mut j = 0;

        for i in (0..trytes.0.len()).step_by(5) {
            // NOTE: currently we assume that at least one full iteration is possible
            let trits1 = trytes.0[i + 0].trits();
            let trits2 = trytes.0[i + 1].trits();
            let trits3 = trytes.0[i + 2].trits();
            let trits4 = trytes.0[i + 3].trits();
            let trits5 = trytes.0[i + 4].trits();

            let a = (trits1[0] + trits1[1] * 3 + trits1[2] * 9 + trits2[0] * 27 + trits2[1] * 81)
                as i16;
            let b = (trits2[2] + trits3[0] * 3 + trits3[1] * 9 + trits3[2] * 27 + trits4[0] * 81)
                as i16;
            let c = (trits4[1] + trits4[2] * 3 + trits5[0] * 9 + trits5[1] * 27 + trits5[2] * 81)
                as i16;

            let a = if a < 0 { a + 243 } else { a };
            let b = if b < 0 { b + 243 } else { b };
            let c = if c < 0 { c + 243 } else { c };

            bytes[j + 0] = a as u8;
            bytes[j + 1] = b as u8;
            bytes[j + 2] = c as u8;

            j += 3;
        }

        T5B1::from_bytes(&bytes)
    }
}

impl From<T3B1> for T9B2 {
    fn from(trytes: T3B1) -> T9B2 {
        if trytes.len() % 3 != 0 {
            panic!("Stay calm, this panic is only of temporary nature! <boarding escape pod>");
        }

        let mut bytes = vec![0u8; (trytes.len() / 3) * 2];
        let mut j = 0;

        for i in (0..trytes.0.len()).step_by(3) {
            let a = trytes.0[i + 0].value();
            let b = trytes.0[i + 1].value();
            let c = trytes.0[i + 2].value();

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

impl Encoding for T3B1 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add(&mut self, trits: T1B1) {
        let trytes: T3B1 = trits.into();

        for tryte in trytes.0 {
            self.push_internal(tryte);
        }
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
        let mut trytes = T3B1::new();
        assert_eq!(0, trytes.len());

        trytes.push('9').expect("error pushing tryte");
        trytes.push('A').expect("error pushing tryte");
        assert_eq!(2, trytes.len());

        trytes.pop();
        assert_eq!(1, trytes.len());
    }

    #[test]
    fn initialize_from_str() {
        let trytes: T3B1 = "I9LUV9BEE"
            .try_into()
            .expect("error parsing tryte sequence");

        assert_eq!(9, trytes.len());
    }
}
