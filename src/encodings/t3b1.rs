//! Encoding, where 3 trits are stored in a single byte.
//!
//! Advantages:
//!     * Convenient mapping to the tryte alphabet ('9', 'A' ... 'Z')
//!     * Better memory-efficiency than T1B1.
//! Disadvantages:
//!     * Still relatively memory-inefficient. (density: 3 trits per byte)

use super::t1b1::T1B1;
use super::Encoding;

use crate::tryte::BalancedTryte;

use std::fmt;

#[derive(Clone, Debug)]
pub struct T3B1(Vec<BalancedTryte>);

impl T3B1 {
    pub fn from_i8(input: &[i8]) -> Self {
        let mut trytes: Vec<BalancedTryte> = Vec::with_capacity(input.len());

        for tryte in input {
            trytes.push((*tryte).into());
        }

        Self(trytes)
    }

    pub fn get(&self, index: usize) -> BalancedTryte {
        self.0[index]
    }

    pub fn get_as_i8(&self, index: usize) -> i8 {
        self.0[index] as i8
    }

    pub fn push<T>(&mut self, tryte: T)
    where
        T: Into<BalancedTryte>,
    {
        self.0.push(tryte.into());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub(crate) fn push_internal(&mut self, tryte: BalancedTryte) {
        self.0.push(tryte);
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

impl<'a> From<&'a str> for T3B1 {
    fn from(s: &'a str) -> Self {
        let mut trytes = Vec::with_capacity(s.len());

        for c in s.chars() {
            trytes.push(c.into());
        }

        Self(trytes)
    }
}

impl fmt::Display for T3B1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tryte in &self.0 {
            tryte.fmt(f)?;
        }
        Ok(())
    }
}

impl From<T1B1> for T3B1 {
    fn from(input: T1B1) -> T3B1 {
        let n = input.len();
        if n % 3 != 0 {
            unimplemented!("handle not-mulitple-of-3 case");
        }

        let mut trytes = T3B1::with_capacity(n / 3);

        (0..n).step_by(3).for_each(|i| {
            let a = input.get(i + 0) as i8;
            let b = input.get(i + 1) as i8;
            let c = input.get(i + 2) as i8;

            let v = a + b * 3 + c * 9;

            trytes.push_internal(v.into());
        });

        trytes
    }
}

impl IntoIterator for T3B1 {
    type Item = BalancedTryte;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_t3b1() {
        let _ = T3B1::new();
    }

    #[test]
    fn t3b1_from_i8() {
        let input = vec![13, 9, -13, 9, -7, -9, 9];
        let _ = T3B1::from_i8(&input);
    }

    #[test]
    fn new_t3b1_from_str() {
        let trytes: T3B1 = "MINI9TRI".into();

        assert_eq!(8, trytes.len());
    }

    #[test]
    fn push_and_pop() {
        let mut trytes = T3B1::new();
        assert_eq!(0, trytes.len());

        trytes.push('9');
        trytes.push('A');
        assert_eq!(2, trytes.len());

        trytes.pop();
        assert_eq!(1, trytes.len());
    }

    #[test]
    fn display_t3b1() {
        let trytes: T3B1 = "MINI9TRI".into();

        assert_eq!("MINI9TRI", trytes.to_string());
    }
}
