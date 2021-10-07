use crate::board::Board;
use crate::cell::Cell;
use crate::cell::CellList;
use crate::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Column(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Row(pub usize);

// a single cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index(pub Row, pub Column);

// a 3x3-section.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Section(pub usize);

impl Index {
    pub fn to_array_index(&self) -> usize {
        return (self.0 .0 * 9) + self.1 .0;
    }

    pub fn section(&self) -> Section {
        let row = (self.0 .0 as f64 / 3.0).floor() as usize;
        let col = (self.1 .0 as f64 / 3.0).floor() as usize;

        return Section((row * 3) + col);
    }
}

pub trait IndexLike {
    type Output;
    fn get(&self, board: &Board) -> Self::Output;
}

impl IndexLike for Column {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        return CellList {
            cells: find_cells_where(board, |c| c.index.1 == *self),
            origin: *self,
        };
    }
}

impl IndexLike for Row {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        return Self::Output {
            cells: find_cells_where(board, |c| c.index.0 == *self),
            origin: *self,
        };
    }
}

impl IndexLike for Index {
    type Output = Cell;

    fn get(&self, board: &Board) -> Self::Output {
        return board.data[self.to_array_index()];
    }
}

impl IndexLike for Section {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        let start_row = ((self.0 as f64 / 3.0).floor() * 3.0) as usize;
        let start_col = ((self.0 % 3) * 3) as usize;

        let mut out = [Cell::EMPTY; 9];
        let mut i: usize = 0;

        for r in 0..3 {
            for c in 0..3 {
                let index = Index(Row(start_row + r), Column(start_col + c));
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

fn find_cells_where<F, const N: usize>(board: &Board, mut f: F) -> [Cell; N]
where
    F: FnMut(Cell) -> bool,
{
    let mut out = [Cell::EMPTY; N];
    let mut i: usize = 0;

    for cell in board.data {
        if f(cell) {
            out[i] = cell;
            i += 1;
        }
    }

    assert_eq!(N, i);

    return out;
}

#[test]
fn test_index_to_array_index() {
    let index = Index(Row(2), Column(2));

    assert_eq!(20, index.to_array_index())
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
    let block = board.get(Section(1));

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

    let block = board.get(Section(5));

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
    let idx = Index(Row(3), Column(4));
    let cell = board.get(idx);

    println!("{:?}", idx.to_array_index());

    assert_eq!(Some(Digit::Five), cell.digit);
    assert_eq!(Section(4), idx.section());
}
