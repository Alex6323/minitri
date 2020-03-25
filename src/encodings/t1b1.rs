//! Encoding, where 1 trit is stored in a single 1 byte.
//!
//! Advantages:
//!     * Can be converted into from all other encodings easily.
//!     * Single trits can easily be added.
//! Disadvantages:
//!     * Very memory-inefficient. (density: 1 trit per byte)

use super::t3b1::T3B1;
use super::t5b1::T5B1;
use super::Encoding;
//use super::t9b2::T9B2;

use crate::trit::BalancedTrit;

use std::fmt;

#[derive(Clone, Debug)]
pub struct T1B1(Vec<BalancedTrit>);

impl T1B1 {
    pub fn from_i8(input: &[i8]) -> Self {
        let mut trits: Vec<BalancedTrit> = Vec::with_capacity(input.len());

        for trit in input {
            trits.push((*trit).into());
        }

        Self(trits)
    }

    pub fn get(&self, index: usize) -> BalancedTrit {
        self.0[index]
    }

    pub fn get_as_i8(&self, index: usize) -> i8 {
        self.0[index] as i8
    }

    pub fn push<T>(&mut self, trit: T)
    where
        T: Into<BalancedTrit>,
    {
        self.0.push(trit.into());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub(crate) fn push_internal(&mut self, trit: BalancedTrit) {
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

impl<'a> From<&'a str> for T1B1 {
    fn from(s: &'a str) -> Self {
        let mut trits: Vec<BalancedTrit> = Vec::with_capacity(s.len());

        for c in s.chars() {
            trits.push(c.into());
        }

        Self(trits)
    }
}

impl fmt::Display for T1B1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for trit in &self.0 {
            trit.fmt(f)?;
        }
        Ok(())
    }
}

impl From<T3B1> for T1B1 {
    fn from(input: T3B1) -> T1B1 {
        let n = input.len();
        let mut trits = Self::with_capacity(n * 3);

        for tryte in input {
            let sub_trits = tryte.as_trits();
            for trit in &sub_trits {
                trits.push_internal((*trit).into());
            }
        }

        trits
    }
}

impl From<T5B1> for T1B1 {
    fn from(input: T5B1) -> T1B1 {
        let n = input.len();
        let mut trits = vec![0i8; n * 5];

        let mut j = 0;

        (0..n).for_each(|i| {
            trits[j..(j + 5)].copy_from_slice(&LUT[input.get(i) as usize]);

            j += 5;
        });

        Self::from_i8(&trits)
    }
}

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

impl IntoIterator for T1B1 {
    type Item = BalancedTrit;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

const LUT: [[i8; 5]; 243] = [
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [-1, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [-1, -1, 1, 0, 0],
    [0, -1, 1, 0, 0],
    [1, -1, 1, 0, 0],
    [-1, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [1, 0, 1, 0, 0],
    [-1, 1, 1, 0, 0],
    [0, 1, 1, 0, 0],
    [1, 1, 1, 0, 0],
    [-1, -1, -1, 1, 0],
    [0, -1, -1, 1, 0],
    [1, -1, -1, 1, 0],
    [-1, 0, -1, 1, 0],
    [0, 0, -1, 1, 0],
    [1, 0, -1, 1, 0],
    [-1, 1, -1, 1, 0],
    [0, 1, -1, 1, 0],
    [1, 1, -1, 1, 0],
    [-1, -1, 0, 1, 0],
    [0, -1, 0, 1, 0],
    [1, -1, 0, 1, 0],
    [-1, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [1, 0, 0, 1, 0],
    [-1, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [1, 1, 0, 1, 0],
    [-1, -1, 1, 1, 0],
    [0, -1, 1, 1, 0],
    [1, -1, 1, 1, 0],
    [-1, 0, 1, 1, 0],
    [0, 0, 1, 1, 0],
    [1, 0, 1, 1, 0],
    [-1, 1, 1, 1, 0],
    [0, 1, 1, 1, 0],
    [1, 1, 1, 1, 0],
    [-1, -1, -1, -1, 1],
    [0, -1, -1, -1, 1],
    [1, -1, -1, -1, 1],
    [-1, 0, -1, -1, 1],
    [0, 0, -1, -1, 1],
    [1, 0, -1, -1, 1],
    [-1, 1, -1, -1, 1],
    [0, 1, -1, -1, 1],
    [1, 1, -1, -1, 1],
    [-1, -1, 0, -1, 1],
    [0, -1, 0, -1, 1],
    [1, -1, 0, -1, 1],
    [-1, 0, 0, -1, 1],
    [0, 0, 0, -1, 1],
    [1, 0, 0, -1, 1],
    [-1, 1, 0, -1, 1],
    [0, 1, 0, -1, 1],
    [1, 1, 0, -1, 1],
    [-1, -1, 1, -1, 1],
    [0, -1, 1, -1, 1],
    [1, -1, 1, -1, 1],
    [-1, 0, 1, -1, 1],
    [0, 0, 1, -1, 1],
    [1, 0, 1, -1, 1],
    [-1, 1, 1, -1, 1],
    [0, 1, 1, -1, 1],
    [1, 1, 1, -1, 1],
    [-1, -1, -1, 0, 1],
    [0, -1, -1, 0, 1],
    [1, -1, -1, 0, 1],
    [-1, 0, -1, 0, 1],
    [0, 0, -1, 0, 1],
    [1, 0, -1, 0, 1],
    [-1, 1, -1, 0, 1],
    [0, 1, -1, 0, 1],
    [1, 1, -1, 0, 1],
    [-1, -1, 0, 0, 1],
    [0, -1, 0, 0, 1],
    [1, -1, 0, 0, 1],
    [-1, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [-1, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [1, 1, 0, 0, 1],
    [-1, -1, 1, 0, 1],
    [0, -1, 1, 0, 1],
    [1, -1, 1, 0, 1],
    [-1, 0, 1, 0, 1],
    [0, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [-1, 1, 1, 0, 1],
    [0, 1, 1, 0, 1],
    [1, 1, 1, 0, 1],
    [-1, -1, -1, 1, 1],
    [0, -1, -1, 1, 1],
    [1, -1, -1, 1, 1],
    [-1, 0, -1, 1, 1],
    [0, 0, -1, 1, 1],
    [1, 0, -1, 1, 1],
    [-1, 1, -1, 1, 1],
    [0, 1, -1, 1, 1],
    [1, 1, -1, 1, 1],
    [-1, -1, 0, 1, 1],
    [0, -1, 0, 1, 1],
    [1, -1, 0, 1, 1],
    [-1, 0, 0, 1, 1],
    [0, 0, 0, 1, 1],
    [1, 0, 0, 1, 1],
    [-1, 1, 0, 1, 1],
    [0, 1, 0, 1, 1],
    [1, 1, 0, 1, 1],
    [-1, -1, 1, 1, 1],
    [0, -1, 1, 1, 1],
    [1, -1, 1, 1, 1],
    [-1, 0, 1, 1, 1],
    [0, 0, 1, 1, 1],
    [1, 0, 1, 1, 1],
    [-1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [-1, -1, -1, -1, -1],
    [0, -1, -1, -1, -1],
    [1, -1, -1, -1, -1],
    [-1, 0, -1, -1, -1],
    [0, 0, -1, -1, -1],
    [1, 0, -1, -1, -1],
    [-1, 1, -1, -1, -1],
    [0, 1, -1, -1, -1],
    [1, 1, -1, -1, -1],
    [-1, -1, 0, -1, -1],
    [0, -1, 0, -1, -1],
    [1, -1, 0, -1, -1],
    [-1, 0, 0, -1, -1],
    [0, 0, 0, -1, -1],
    [1, 0, 0, -1, -1],
    [-1, 1, 0, -1, -1],
    [0, 1, 0, -1, -1],
    [1, 1, 0, -1, -1],
    [-1, -1, 1, -1, -1],
    [0, -1, 1, -1, -1],
    [1, -1, 1, -1, -1],
    [-1, 0, 1, -1, -1],
    [0, 0, 1, -1, -1],
    [1, 0, 1, -1, -1],
    [-1, 1, 1, -1, -1],
    [0, 1, 1, -1, -1],
    [1, 1, 1, -1, -1],
    [-1, -1, -1, 0, -1],
    [0, -1, -1, 0, -1],
    [1, -1, -1, 0, -1],
    [-1, 0, -1, 0, -1],
    [0, 0, -1, 0, -1],
    [1, 0, -1, 0, -1],
    [-1, 1, -1, 0, -1],
    [0, 1, -1, 0, -1],
    [1, 1, -1, 0, -1],
    [-1, -1, 0, 0, -1],
    [0, -1, 0, 0, -1],
    [1, -1, 0, 0, -1],
    [-1, 0, 0, 0, -1],
    [0, 0, 0, 0, -1],
    [1, 0, 0, 0, -1],
    [-1, 1, 0, 0, -1],
    [0, 1, 0, 0, -1],
    [1, 1, 0, 0, -1],
    [-1, -1, 1, 0, -1],
    [0, -1, 1, 0, -1],
    [1, -1, 1, 0, -1],
    [-1, 0, 1, 0, -1],
    [0, 0, 1, 0, -1],
    [1, 0, 1, 0, -1],
    [-1, 1, 1, 0, -1],
    [0, 1, 1, 0, -1],
    [1, 1, 1, 0, -1],
    [-1, -1, -1, 1, -1],
    [0, -1, -1, 1, -1],
    [1, -1, -1, 1, -1],
    [-1, 0, -1, 1, -1],
    [0, 0, -1, 1, -1],
    [1, 0, -1, 1, -1],
    [-1, 1, -1, 1, -1],
    [0, 1, -1, 1, -1],
    [1, 1, -1, 1, -1],
    [-1, -1, 0, 1, -1],
    [0, -1, 0, 1, -1],
    [1, -1, 0, 1, -1],
    [-1, 0, 0, 1, -1],
    [0, 0, 0, 1, -1],
    [1, 0, 0, 1, -1],
    [-1, 1, 0, 1, -1],
    [0, 1, 0, 1, -1],
    [1, 1, 0, 1, -1],
    [-1, -1, 1, 1, -1],
    [0, -1, 1, 1, -1],
    [1, -1, 1, 1, -1],
    [-1, 0, 1, 1, -1],
    [0, 0, 1, 1, -1],
    [1, 0, 1, 1, -1],
    [-1, 1, 1, 1, -1],
    [0, 1, 1, 1, -1],
    [1, 1, 1, 1, -1],
    [-1, -1, -1, -1, 0],
    [0, -1, -1, -1, 0],
    [1, -1, -1, -1, 0],
    [-1, 0, -1, -1, 0],
    [0, 0, -1, -1, 0],
    [1, 0, -1, -1, 0],
    [-1, 1, -1, -1, 0],
    [0, 1, -1, -1, 0],
    [1, 1, -1, -1, 0],
    [-1, -1, 0, -1, 0],
    [0, -1, 0, -1, 0],
    [1, -1, 0, -1, 0],
    [-1, 0, 0, -1, 0],
    [0, 0, 0, -1, 0],
    [1, 0, 0, -1, 0],
    [-1, 1, 0, -1, 0],
    [0, 1, 0, -1, 0],
    [1, 1, 0, -1, 0],
    [-1, -1, 1, -1, 0],
    [0, -1, 1, -1, 0],
    [1, -1, 1, -1, 0],
    [-1, 0, 1, -1, 0],
    [0, 0, 1, -1, 0],
    [1, 0, 1, -1, 0],
    [-1, 1, 1, -1, 0],
    [0, 1, 1, -1, 0],
    [1, 1, 1, -1, 0],
    [-1, -1, -1, 0, 0],
    [0, -1, -1, 0, 0],
    [1, -1, -1, 0, 0],
    [-1, 0, -1, 0, 0],
    [0, 0, -1, 0, 0],
    [1, 0, -1, 0, 0],
    [-1, 1, -1, 0, 0],
    [0, 1, -1, 0, 0],
    [1, 1, -1, 0, 0],
    [-1, -1, 0, 0, 0],
    [0, -1, 0, 0, 0],
    [1, -1, 0, 0, 0],
    [-1, 0, 0, 0, 0],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encodings::t3b1::T3B1;

    #[test]
    fn new_t1b1() {
        let _ = T1B1::new();
    }

    #[test]
    fn t1b1_from_i8() {
        let input = vec![-1, -1, 1, 0, -1, 1];
        let _ = T1B1::from_i8(&input);
    }

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
        let trits: T1B1 = "10-01-110".into();

        assert_eq!(9, trits.len());
    }

    #[test]
    fn display_t1b1() {
        let trits: T1B1 = "10-01-110".into();

        assert_eq!("10-01-110", trits.to_string());
    }

    #[test]
    fn from_t3b1() {
        let trytes: T3B1 = "MINI9TRI".into();
        let trits: T1B1 = trytes.into();

        assert_eq!(24, trits.len());
    }
}
