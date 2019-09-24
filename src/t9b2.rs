use crate::error;
use crate::luts;
use crate::t1b1::{ToT1B1, T1B1};
use crate::trits::RawTrits;

#[derive(Debug)]
pub struct T9B2(Vec<u8>);

pub trait ToT9B2 {
    fn to_t9b2(&self) -> error::Result<T9B2>;
}

impl T9B2 {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        Self(Vec::from(bytes))
    }
}

impl ToT1B1 for T9B2 {
    fn to_t1b1(&self) -> T1B1 {
        let mut trits = vec![0i8; (self.len() / 2) * 9];

        let mut j = 0;

        for i in (0..self.len()).step_by(2) {
            let a = self.0[i + 0] as usize;
            let b = self.0[i + 1] as usize;

            trits[(j + 0)..(j + 3)].copy_from_slice(&luts::trits_from_tryteindex_internal(a / 8));
            trits[(j + 3)..(j + 6)].copy_from_slice(&luts::trits_from_tryteindex_internal(b / 8));
            trits[(j + 6)..(j + 9)]
                .copy_from_slice(&luts::trits_from_tryteindex_internal(a % 8 + 8 * (b % 8)));

            j += 9;
        }

        T1B1::from_sbytes(trits)
    }
}

impl RawTrits for T9B2 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add_trits(&mut self, trits: T1B1) -> error::Result<()> {
        let bytes = trits.to_t9b2()?;

        for byte in bytes.0 {
            self.0.push(byte);
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
