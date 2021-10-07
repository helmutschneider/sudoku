use crate::board::Board;
use crate::cell::Cell;
use crate::cell::CellList;
use crate::digit::Digit;

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

pub trait Index {
    type Output;
    fn get(&self, board: &Board) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Column(pub usize);

impl Index for Column {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        return CellList {
            cells: find_cells_where(board, |c| c.column == *self),
            origin: *self,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Row(pub usize);

impl Index for Row {
    type Output = CellList<Self>;

    fn get(&self, board: &Board) -> Self::Output {
        return Self::Output {
            cells: find_cells_where(board, |c| c.row == *self),
            origin: *self,
        };
    }
}
