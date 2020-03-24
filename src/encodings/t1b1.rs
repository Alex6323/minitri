use crate::luts;

use super::t3b1::T3B1;
use super::Encoding;

//use super::t5b1::T5B1;
//use super::t9b2::T9B2;

use crate::trit::Trit;
use crate::tryte::Tryte;

use std::ops::Deref;

#[derive(Debug)]
pub struct T1B1(Vec<Trit>);

impl T1B1 {
    pub(crate) fn from_sbytes(sbytes: &[i8]) -> Self {
        let mut trits = Vec::with_capacity(sbytes.len());

        for trit in sbytes {
            trits.push(trit.into());
        }

        Self(trits)
    }

    pub fn push<T>(&mut self, trit: T)
    where
        T: Into<Trit>,
    {
        self.0.push(trit.into());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub(crate) fn push_internal(&mut self, trit: Trit) {
        self.0.push(trit);
    }
}

impl Deref for T1B1 {
    type Target = Vec<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
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

impl<'a> From<&'a str> for T1B1 {
    fn from(s: &'a str) -> Self {
        let mut trits: Vec<i8> = Vec::with_capacity(s.len());

        for c in s.chars() {
            trits.push(c.into());
        }

        Self(trits)
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

/*
impl From<T5B1> for T1B1 {
    fn from(bytes: T5B1) -> T1B1 {
        let mut trits = vec![0i8; bytes.len() * 5];

        let mut j = 0;

        for i in 0..bytes.len() {
            trits[j..(j + 5)].copy_from_slice(&luts::trits_from_byte_internal(bytes.0[i]));

            j += 5;
        }

        T1B1::from_sbytes(trits)
    }
}
*/

/*
impl From<T9B2> for T1B1 {
    fn from(bytes: T9B2) -> T1B1 {
        let mut trits = vec![0i8; (bytes.len() / 2) * 9];

        let mut j = 0;

        for i in (0..bytes.len()).step_by(2) {
            let a = bytes.0[i + 0] as usize;
            let b = bytes.0[i + 1] as usize;

            trits[(j + 0)..(j + 3)].copy_from_slice(&luts::trits_from_tryteindex_internal(a / 8));
            trits[(j + 3)..(j + 6)].copy_from_slice(&luts::trits_from_tryteindex_internal(b / 8));
            trits[(j + 6)..(j + 9)]
                .copy_from_slice(&luts::trits_from_tryteindex_internal(a % 8 + 8 * (b % 8)));

            j += 9;
        }

        T1B1::from_sbytes(trits)
    }
}
*/

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn resize_with_push_and_pop() {
        let mut trits = T1B1::new();
        assert_eq!(0, trits.len());

        trits.push('1');
        trits.push('-');
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
