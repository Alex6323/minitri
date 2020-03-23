use crate::luts;

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

impl From<char> for Trit {
    fn from(c: char) -> Self {
        match c {
            '1' => Trit(1),
            '0' => Trit(0),
            '-' => Trit(-1),
            _ => panic!("Invalid trit char"),
        }
    }
}

impl Into<Trit> for i8 {
    fn into(self) -> Trit {
        match self {
            1 => Trit(1),
            0 => Trit(0),
            -1 => Trit(-1),
            _ => panic!("Invalid i8"),
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
    fn be_creatable_from_valid_chars() {
        let trit: Trit = '1'.into();
        assert_eq!(1, trit.value());

        let trit: Trit = '0'.into();
        assert_eq!(0, trit.value());

        let trit: Trit = '-'.into();
        assert_eq!(-1, trit.value());
    }

    #[test]
    fn be_creatable_from_valid_values() {
        let trit: Trit = 1.into();
        assert_eq!(1, trit.value());

        let trit: Trit = 0.into();
        assert_eq!(0, trit.value());

        let trit: Trit = (-1).into();
        assert_eq!(-1, trit.value());
    }

    #[test]
    #[should_panic]
    fn fail_initializing_for_invalid_char() {
        let _: Trit = '2'.into();
    }

    #[test]
    #[should_panic]
    fn fail_initializing_for_invalid_value() {
        let _: Trit = 2.into();
    }
}
