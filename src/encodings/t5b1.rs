use crate::encodings::{t1b1::T1B1, t3b1::T3B1, Encoding};

#[derive(Debug)]
pub struct T5B1(Vec<u8>);

impl T5B1 {
    pub fn from_u8(input: &[u8]) -> Self {
        Self(Vec::from(input))
    }

    pub fn get(&self, index: usize) -> u8 {
        self.0[index]
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
    fn from(input: T1B1) -> T5B1 {
        let n = input.len();
        if n % 5 != 0 {
            unimplemented!("handle not-multiple-of-5 case");
        }

        let mut bytes = vec![0u8; n / 5];
        let mut j = 0;

        (0..n).step_by(5).for_each(|i| {
            let a = (input.get_as_i8(i + 0)
                + input.get_as_i8(i + 1) * 3
                + input.get_as_i8(i + 2) * 9
                + input.get_as_i8(i + 3) * 27
                + input.get_as_i8(i + 4) * 81) as i16;

            let a = if a < 0 { a + 243 } else { a };

            bytes[j] = a as u8;

            j += 1;
        });

        Self(bytes)
    }
}

impl From<T3B1> for T5B1 {
    fn from(input: T3B1) -> T5B1 {
        let n = input.len();
        if (n * 3) % 5 != 0 {
            unimplemented!("handle times-3-not-multiple-of-5 case");
        }

        let mut bytes = vec![0u8; (n * 3) / 5];
        let mut j = 0;

        for i in (0..n).step_by(5) {
            // NOTE: currently we assume that at least one full iteration is possible
            let trits1 = input.get(i + 0).as_trits();
            let trits2 = input.get(i + 1).as_trits();
            let trits3 = input.get(i + 2).as_trits();
            let trits4 = input.get(i + 3).as_trits();
            let trits5 = input.get(i + 4).as_trits();

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

        Self(bytes)
    }
}

impl IntoIterator for T5B1 {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
