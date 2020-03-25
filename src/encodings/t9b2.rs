//! Encoding, where 9 trits are stored using 2 bytes.
//!
//! Advantages:
//!     * Multiples of 3 trytes map to even multiplis of bytes.
//!     * More memory-efficient than T4B1 or T8B2.
//! Disadvantages:
//!     * Less memory-efficient than T5B1. (density: 4.5 trits per byte)

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
