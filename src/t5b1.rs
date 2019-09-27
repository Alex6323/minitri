use crate::error;
use crate::luts;
use crate::t1b1::T1B1;
use crate::trits::Encoding;

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

impl Encoding for T5B1 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add(&mut self, trits: T1B1) {
        let bytes: T5B1 = trits.into();

        for byte in bytes.0 {
            self.0.push(byte);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
