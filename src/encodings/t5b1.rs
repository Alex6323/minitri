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

impl From<T1B1> for T5B1 {
    fn from(trits: T1B1) -> T5B1 {
        if trits.len() % 5 != 0 {
            panic!("Stay calm, this panic is only of temporary nature! <boarding escape pod>");
        }

        let mut bytes = vec![0u8; trits.len() / 5];
        let mut j = 0;

        for i in (0..trits.0.len()).step_by(5) {
            let a = (trits.0[i + 0].value()
                + trits.0[i + 1].value() * 3
                + trits.0[i + 2].value() * 9
                + trits.0[i + 3].value() * 27
                + trits.0[i + 4].value() * 81) as i16;

            let a = if a < 0 { a + 243 } else { a };

            bytes[j] = a as u8;

            j += 1;
        }

        T5B1::from_bytes(&bytes)
    }
}

impl From<T3B1> for T5B1 {
    fn from(trytes: T3B1) -> T5B1 {
        if (trytes.len() * 3) % 5 != 0 {
            panic!("Incompatible lengths");
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
