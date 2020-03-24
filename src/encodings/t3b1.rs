use super::t1b1::T1B1;
use crate::luts;

use super::Encoding;
//use super::t5b1::T5B1;
//use super::t9b2::T9B2;

use crate::trit::Trit;
use crate::tryte::Tryte;

#[derive(Debug)]
pub struct T3B1(Vec<Tryte>);

impl T3B1 {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push<T>(&mut self, tryte: T)
    where
        T: Into<Tryte>,
    {
        self.0.push(tryte.into());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub(crate) fn push_internal(&mut self, tryte: Tryte) {
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

impl std::fmt::Display for T3B1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for tryte in &self.0 {
            tryte.fmt(f)?;
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

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn resize_with_push_and_pop() {
        let mut trytes = T3B1::new();
        assert_eq!(0, trytes.len());

        trytes.push('9');
        trytes.push('A');
        assert_eq!(2, trytes.len());

        trytes.pop();
        assert_eq!(1, trytes.len());
    }

    #[test]
    fn initialize_from_str() {
        let trytes: T3B1 = "I9LUV9BEE".into();

        assert_eq!(9, trytes.len());
    }
}
