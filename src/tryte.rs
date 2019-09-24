use crate::error::TrinaryError;
use crate::luts;

use std::convert::TryInto;

#[derive(Debug)]
pub struct Tryte(pub(crate) i8);

impl Tryte {
    pub fn value(&self) -> i8 {
        self.0
    }

    pub(crate) fn index(&self) -> usize {
        if self.0 < 0 {
            (27 + self.0) as usize
        } else {
            self.0 as usize
        }
    }

    pub(crate) fn trits(&self) -> [i8; 3] {
        luts::trits_from_tryteindex_internal(self.index())
    }
}

impl TryInto<Tryte> for char {
    type Error = TrinaryError;
    fn try_into(self) -> Result<Tryte, Self::Error> {
        match self {
            '9' => Ok(Tryte(0)),
            n @ 'A'..='M' => Ok(Tryte(1 + (n as i8 - 65))),
            n @ 'N'..='Z' => Ok(Tryte((n as i8 - 90) - 1)),
            _ => Err(TrinaryError::InvalidTryteChar),
        }
    }
}

impl TryInto<Tryte> for i8 {
    type Error = TrinaryError;
    fn try_into(self) -> Result<Tryte, Self::Error> {
        match self {
            -13..=13 => Ok(Tryte(self)),
            _ => Err(TrinaryError::InvalidTryteChar),
        }
    }
}

impl std::fmt::Display for Tryte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", luts::TRYTE_SYMBOLS[self.index()])
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn be_creatable_from_valid_chars() -> Result<(), TrinaryError> {
        let tryte: Tryte = 'A'.try_into()?;
        assert_eq!(1, tryte.value());

        let tryte: Tryte = 'M'.try_into()?;
        assert_eq!(13, tryte.value());

        let tryte: Tryte = '9'.try_into()?;
        assert_eq!(0, tryte.value());

        let tryte: Tryte = 'N'.try_into()?;
        assert_eq!(-13, tryte.value());

        let tryte: Tryte = 'Z'.try_into()?;
        assert_eq!(-1, tryte.value());

        Ok(())
    }

    #[test]
    fn be_creatable_from_valid_values() -> Result<(), TrinaryError> {
        let tryte: Tryte = 1.try_into()?;
        assert_eq!(1, tryte.value());

        let tryte: Tryte = 13.try_into()?;
        assert_eq!(13, tryte.value());

        let tryte: Tryte = 0.try_into()?;
        assert_eq!(0, tryte.value());

        let tryte: Tryte = (-13).try_into()?;
        assert_eq!(-13, tryte.value());

        let tryte: Tryte = (-1).try_into()?;
        assert_eq!(-1, tryte.value());

        Ok(())
    }

    #[test]
    #[should_panic]
    fn fail_initializing_for_invalid_char() {
        let _: Tryte = 'a'.try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_initializing_for_invalid_value() {
        let _: Tryte = 14.try_into().unwrap();
    }
}
