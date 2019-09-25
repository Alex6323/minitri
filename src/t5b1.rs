use crate::error;
use crate::luts;
use crate::t1b1::{ToT1B1, T1B1};
use crate::trits::RawTrits;

#[derive(Debug)]
pub struct T5B1(Vec<u8>);

pub trait ToT5B1 {
    fn to_t5b1(&self) -> error::Result<T5B1>;
}

impl T5B1 {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        Self(Vec::from(bytes))
    }
}

impl ToT1B1 for T5B1 {
    fn to_t1b1(&self) -> T1B1 {
        let mut trits = vec![0i8; self.len() * 5];

        let mut j = 0;

        for i in 0..self.len() {
            trits[j..(j + 5)].copy_from_slice(&luts::trits_from_byte_internal(self.0[i]));

            j += 5;
        }

        T1B1::from_sbytes(trits)
    }
}

impl RawTrits for T5B1 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add_trits(&mut self, trits: T1B1) -> error::Result<()> {
        let bytes = trits.to_t5b1()?;

        for byte in bytes.0 {
            self.0.push(byte);
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
