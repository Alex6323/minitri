use crate::error::TrinaryError;
use crate::luts;

use std::convert::TryInto;

#[derive(Debug)]
pub struct Trit(pub(crate) i8);

impl Trit {
    pub fn value(&self) -> i8 {
        self.0
    }
    pub(crate) fn index(&self) -> usize {
        match self.0 {
            0 | 1 => self.0 as usize,
            -1 => 2 as usize,
            _ => unreachable!(),
        }
    }
}

impl TryInto<Trit> for char {
    type Error = TrinaryError;
    fn try_into(self) -> Result<Trit, Self::Error> {
        match self {
            '1' => Ok(Trit(1)),
            '0' => Ok(Trit(0)),
            '-' => Ok(Trit(-1)),
            _ => Err(TrinaryError::InvalidTritChar),
        }
    }
}

impl TryInto<Trit> for i8 {
    type Error = TrinaryError;
    fn try_into(self) -> Result<Trit, Self::Error> {
        match self {
            1 | 0 | -1 => Ok(Trit(self)),
            _ => Err(TrinaryError::InvalidTritChar),
        }
    }
}

impl std::fmt::Display for Trit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", luts::TRIT_SYMBOLS[self.index()])
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn be_creatable_from_valid_chars() -> Result<(), TrinaryError> {
        let trit: Trit = '1'.try_into()?;
        assert_eq!(1, trit.value());

        let trit: Trit = '0'.try_into()?;
        assert_eq!(0, trit.value());

        let trit: Trit = '-'.try_into()?;
        assert_eq!(-1, trit.value());

        Ok(())
    }

    #[test]
    fn be_creatable_from_valid_values() -> Result<(), TrinaryError> {
        let trit: Trit = 1.try_into()?;
        assert_eq!(1, trit.value());

        let trit: Trit = 0.try_into()?;
        assert_eq!(0, trit.value());

        let trit: Trit = (-1).try_into()?;
        assert_eq!(-1, trit.value());

        Ok(())
    }

    #[test]
    #[should_panic]
    fn fail_initializing_for_invalid_char() {
        let _: Trit = '2'.try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_initializing_for_invalid_value() {
        let _: Trit = 2.try_into().unwrap();
    }
}
