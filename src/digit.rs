#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    pub const ALL_DIGITS: [Digit; 9] = [
        Digit::One,
        Digit::Two,
        Digit::Three,
        Digit::Four,
        Digit::Five,
        Digit::Six,
        Digit::Seven,
        Digit::Eight,
        Digit::Nine,
    ];

    pub const EMPTY_CHARACTER: char = '-';

    pub fn to_u8(&self) -> u8 {
        for i in 0..Self::ALL_DIGITS.len() {
            let digit = Self::ALL_DIGITS[i];
            if digit == *self {
                return (i + 1) as u8;
            }
        }
        panic!("Unknown digit {:?}.", self);
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        if let Some(index) = value.checked_sub(1) {
            return Self::ALL_DIGITS.get(index as usize).map(|x| *x);
        }
        return None;
    }

    pub fn from_char(value: char) -> Option<Self> {
        if value.is_ascii_digit() {
            return Self::from_u8((value as u8) - 48);
        }
        return None;
    }
}

#[test]
fn test_from_char() {
    assert_eq!(Digit::One, Digit::from_char('1').unwrap());
    assert_eq!(Digit::Nine, Digit::from_char('9').unwrap());
}

#[test]
fn test_from_u8() {
    assert_eq!(Digit::One, Digit::from_u8(1).unwrap());
    assert_eq!(Digit::Nine, Digit::from_u8(9).unwrap());
    assert_eq!(None, Digit::from_u8(10));
}

#[test]
fn test_to_u8() {
    assert_eq!(1, Digit::One.to_u8());
    assert_eq!(9, Digit::Nine.to_u8());
}
