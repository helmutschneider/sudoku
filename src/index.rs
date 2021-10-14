use crate::board::Board;
use crate::cell::Cell;
use crate::cell::CellList;
use crate::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColumnIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowIndex(pub usize);

// a single cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellIndex(pub RowIndex, pub ColumnIndex);

impl CellIndex {
    pub const ZERO: Self = Self(RowIndex(0), ColumnIndex(0));
}

// a 3x3-section.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionIndex(pub usize);

impl CellIndex {
    pub fn section(&self) -> SectionIndex {
        let row = (self.0 .0 as f64 / 3.0).floor() as usize;
        let col = (self.1 .0 as f64 / 3.0).floor() as usize;

        return SectionIndex((row * 3) + col);
    }
}

pub trait IndexLike {
    type Output;
    fn get(&self, board: &Board) -> Self::Output;
}

impl IndexLike for ColumnIndex {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        let mut cells = [Cell::EMPTY; 9];

        for i in 0..9 {
            cells[i] = board.data[i][self.0];
        }

        return CellList {
            cells: cells,
            origin: *self,
        };
    }
}

impl IndexLike for RowIndex {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        return Self::Output {
            cells: board.data[self.0],
            origin: *self,
        };
    }
}

impl IndexLike for CellIndex {
    type Output = Cell;

    fn get(&self, board: &Board) -> Self::Output {
        return board.data[self.0 .0][self.1 .0];
    }
}

impl IndexLike for SectionIndex {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        let start_row = ((self.0 as f64 / 3.0).floor() * 3.0) as usize;
        let start_col = ((self.0 % 3) * 3) as usize;

        let mut out = [Cell::EMPTY; 9];
        let mut i: usize = 0;

        for r in 0..3 {
            for c in 0..3 {
                let index = CellIndex(RowIndex(start_row + r), ColumnIndex(start_col + c));
                out[i] = board.get(index);
                i += 1;
            }
        }

        return CellList {
            cells: out,
            origin: *self,
        };
    }
}

#[test]
fn test_block_get() {
    let str = r#"
    9 - - 8 3 - 1 5 7
    5 - 3 1 - 6 2 8 -
    1 - - 7 4 - - 9 -
    - - - - 5 - 8 3 -
    3 - 1 - - 4 6 7 2
    2 - - - 1 3 - - 9
    - - 2 - 7 - - 1 -
    - - - - - - - 6 -
    - 3 4 - 6 - 9 2 -
    "#;
    let board = Board::from_str(str);
    let block = board.get(SectionIndex(1));

    assert_eq!(
        [
            Some(Digit::Eight),
            Some(Digit::Three),
            None,
            Some(Digit::One),
            None,
            Some(Digit::Six),
            Some(Digit::Seven),
            Some(Digit::Four),
            None,
        ],
        block.cells.map(|c| c.digit)
    );

    let block = board.get(SectionIndex(5));

    assert_eq!(
        [
            Some(Digit::Eight),
            Some(Digit::Three),
            None,
            Some(Digit::Six),
            Some(Digit::Seven),
            Some(Digit::Two),
            None,
            None,
            Some(Digit::Nine),
        ],
        block.cells.map(|c| c.digit)
    );
}

#[test]
fn test_index_get_section() {
    let str = r#"
    9 - - 8 3 - 1 5 7
    5 - 3 1 - 6 2 8 -
    1 - - 7 4 - - 9 -
    - - - - 5 - 8 3 -
    3 - 1 - - 4 6 7 2
    2 - - - 1 3 - - 9
    - - 2 - 7 - - 1 -
    - - - - - - - 6 -
    - 3 4 - 6 - 9 2 -
    "#;
    let board = Board::from_str(str);
    let idx = CellIndex(RowIndex(3), ColumnIndex(4));
    let cell = board.get(idx);

    assert_eq!(Some(Digit::Five), cell.digit);
    assert_eq!(SectionIndex(4), idx.section());
}
