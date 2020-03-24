use std::fmt;

#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BalancedTrit {
    MinusOne = -1,
    Zero = 0,
    PlusOne = 1,
}

impl From<char> for BalancedTrit {
    fn from(c: char) -> Self {
        match c {
            '-' => BalancedTrit::MinusOne,
            '0' => BalancedTrit::Zero,
            '1' => BalancedTrit::PlusOne,
            _ => panic!("Invalid trit character"),
        }
    }
}

impl From<i8> for BalancedTrit {
    fn from(i: i8) -> Self {
        match i {
            -1 => BalancedTrit::MinusOne,
            0 => BalancedTrit::Zero,
            1 => BalancedTrit::PlusOne,
            _ => panic!("Invalid i8 integer"),
        }
    }
}

impl fmt::Display for BalancedTrit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                BalancedTrit::MinusOne => '-',
                BalancedTrit::Zero => '0',
                BalancedTrit::PlusOne => '1',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_trits_from_chars() {
        assert_eq!(BalancedTrit::PlusOne, '1'.into());
        assert_eq!(BalancedTrit::Zero, '0'.into());
        assert_eq!(BalancedTrit::MinusOne, '-'.into());
    }

    #[test]
    fn create_trits_from_values() {
        assert_eq!(BalancedTrit::PlusOne, 1.into());
        assert_eq!(BalancedTrit::Zero, 0.into());
        assert_eq!(BalancedTrit::MinusOne, (-1).into());
    }

    #[test]
    #[should_panic]
    fn fail_for_invalid_char() {
        let _: BalancedTrit = '2'.into();
    }

    #[test]
    #[should_panic]
    fn fail_for_invalid_value() {
        let _: BalancedTrit = 2.into();
    }
}
