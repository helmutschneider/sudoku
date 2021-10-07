use crate::digit::Digit;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub digit: Option<Digit>,
    pub column: usize,
    pub row: usize,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        digit: None,
        column: 0,
        row: 0,
    };

    pub fn to_string(&self) -> String {
        return match self.digit {
            Some(digit) => digit.to_u8().to_string(),
            _ => String::from(Digit::EMPTY_CHARACTER),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CellList<'a>(pub [&'a Cell; 9]);
