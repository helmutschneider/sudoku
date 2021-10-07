use crate::digit::Digit;
use crate::index::Column;
use crate::index::Index;
use crate::index::Row;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub digit: Option<Digit>,
    pub index: Index,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        digit: None,
        index: Index(Row(0), Column(0)),
    };

    pub fn to_string(&self) -> String {
        return match self.digit {
            Some(digit) => digit.to_u8().to_string(),
            _ => String::from(Digit::EMPTY_CHARACTER),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CellList<T> {
    pub cells: [Cell; 9],
    pub origin: T,
}
