use crate::error;
use crate::luts;
use crate::t1b1::T1B1;
use crate::trits::Encoding;

#[derive(Debug)]
pub struct T9B2(Vec<u8>);

impl T9B2 {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        Self(Vec::from(bytes))
    }
}

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

impl Encoding for T9B2 {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add(&mut self, trits: T1B1) {
        let bytes: T9B2 = trits.into();

        for byte in bytes.0 {
            self.0.push(byte);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
