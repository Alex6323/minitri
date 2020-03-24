use std::fmt;

#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BalancedTryte {
    N = -13,
    O = -12,
    P = -11,
    Q = -10,
    R = -9,
    S = -8,
    T = -7,
    U = -6,
    V = -5,
    W = -4,
    X = -3,
    Y = -2,
    Z = -1,
    Nine = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
}

impl BalancedTryte {
    pub fn as_trits(&self) -> [i8; 3] {
        use BalancedTryte::*;
        match *self {
            A => [1, 0, 0],
            B => [-1, 1, 0],
            C => [0, 1, 0],
            D => [1, 1, 0],
            E => [-1, -1, 1],
            F => [0, -1, 1],
            G => [1, -1, 1],
            H => [-1, 0, 1],
            I => [0, 0, 1],
            J => [1, 0, 1],
            K => [-1, 1, 1],
            L => [0, 1, 1],
            M => [1, 1, 1],
            Nine => [0, 0, 0],
            N => [-1, -1, -1],
            O => [0, -1, -1],
            P => [1, -1, -1],
            Q => [-1, 0, -1],
            R => [0, 0, -1],
            S => [1, 0, -1],
            T => [-1, 1, -1],
            U => [0, 1, -1],
            V => [1, 1, -1],
            W => [-1, -1, 0],
            X => [0, -1, 0],
            Y => [1, -1, 0],
            Z => [-1, 0, 0],
        }
    }
}

impl From<char> for BalancedTryte {
    fn from(c: char) -> Self {
        use BalancedTryte::*;
        match c {
            'N' => N,
            'O' => O,
            'P' => P,
            'Q' => Q,
            'R' => R,
            'S' => S,
            'T' => T,
            'U' => U,
            'V' => V,
            'W' => W,
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            '9' => Nine,
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            'E' => E,
            'F' => F,
            'G' => G,
            'H' => H,
            'I' => I,
            'J' => J,
            'K' => K,
            'L' => L,
            'M' => M,
            _ => panic!("Invalid tryte character"),
        }
    }
}

impl From<i8> for BalancedTryte {
    fn from(i: i8) -> Self {
        use BalancedTryte::*;
        match i {
            -13 => N,
            -12 => O,
            -11 => P,
            -10 => Q,
            -9 => R,
            -8 => S,
            -7 => T,
            -6 => U,
            -5 => V,
            -4 => W,
            -3 => X,
            -2 => Y,
            -1 => Z,
            0 => Nine,
            1 => A,
            2 => B,
            3 => C,
            4 => D,
            5 => E,
            6 => F,
            7 => G,
            8 => H,
            9 => I,
            10 => J,
            11 => K,
            12 => L,
            13 => M,
            _ => panic!("Invalid i8 integer"),
        }
    }
}

impl fmt::Display for BalancedTryte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BalancedTryte::*;
        let c = match *self {
            N => 'N',
            O => 'O',
            P => 'P',
            Q => 'Q',
            R => 'R',
            S => 'S',
            T => 'T',
            U => 'U',
            V => 'V',
            W => 'W',
            X => 'X',
            Y => 'Y',
            Z => 'Z',
            Nine => '9',
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
            K => 'K',
            L => 'L',
            M => 'M',
        };

        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_trytes_from_chars() {
        let tryte: BalancedTryte = 'A'.into();
        assert_eq!(1, tryte as i8);

        let tryte: BalancedTryte = 'M'.into();
        assert_eq!(13, tryte as i8);

        let tryte: BalancedTryte = '9'.into();
        assert_eq!(0, tryte as i8);

        let tryte: BalancedTryte = 'N'.into();
        assert_eq!(-13, tryte as i8);

        let tryte: BalancedTryte = 'Z'.into();
        assert_eq!(-1, tryte as i8);
    }

    #[test]
    fn create_trytes_from_values() {
        assert_eq!(BalancedTryte::A, 1.into());
        assert_eq!(BalancedTryte::M, 13.into());
        assert_eq!(BalancedTryte::Nine, 0.into());
        assert_eq!(BalancedTryte::N, (-13).into());
        assert_eq!(BalancedTryte::Z, (-1).into());
    }

    #[test]
    #[should_panic]
    fn fail_for_invalid_char() {
        let _: BalancedTryte = 'a'.into();
    }

    #[test]
    #[should_panic]
    fn fail_for_invalid_value() {
        let _: BalancedTryte = 14.into();
    }
}
