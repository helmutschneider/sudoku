use crate::{cell::Cell, cell::CellList, digit::Digit};
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub data: [Cell; 81],
}

fn get_flattened_index(row: usize, column: usize) -> usize {
    return (row * 9) + column;
}

impl Board {
    pub fn new(data: [Cell; 81]) -> Self {
        return Self { data };
    }

    pub fn from_str(value: &str) -> Self {
        let mut data: [Cell; 81] = [Cell::EMPTY; 81];
        let mut index: usize = 0;

        for ch in value.chars() {
            let column: usize = index % 9;
            let row: usize = ((index as f64) / 9.0).floor() as usize;
            let out: Option<Cell> = match ch {
                Digit::EMPTY_CHARACTER => Some(Cell {
                    digit: None,
                    column,
                    row,
                }),
                _ => {
                    let maybe_digit = Digit::from_char(ch);
                    if maybe_digit.is_some() {
                        Some(Cell {
                            digit: maybe_digit,
                            column: column,
                            row: row,
                        })
                    } else {
                        None
                    }
                }
            };
            if let Some(cell) = out {
                data[index] = cell;
                index += 1;
            }
        }

        assert_eq!(81, index);

        return Board::new(data);
    }

    pub fn column(&self, index: usize) -> CellList {
        return CellList(find_cells_where(self, |c| c.column == index));
    }

    pub fn row(&self, index: usize) -> CellList {
        return CellList(find_cells_where(self, |c| c.row == index));
    }

    pub fn set_digit_at_cell(&mut self, cell: Cell, digit: Digit) {
        let idx = get_flattened_index(cell.row, cell.column);
        let cell = &mut self.data[idx];
        cell.digit = Some(digit);
    }

    pub fn is_solved(&self) -> bool {
        return self.data.iter().all(|c| c.digit.is_some());
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            let str = self.row(i).0.map(Cell::to_string).join(" ");
            if i < 8 {
                writeln!(f, "{}", str)?;
            } else {
                write!(f, "{}", str)?;
            }
        }
        return fmt::Result::Ok(());
    }
}

fn find_cells_where<'a, const N: usize>(
    board: &'a Board,
    mut f: impl FnMut(&Cell) -> bool,
) -> [&Cell; N] {
    let mut out = [&Cell::EMPTY; N];
    let mut out_index: usize = 0;

    for cell in &board.data {
        if f(cell) {
            out[out_index] = cell;
            out_index += 1;
        }
    }

    assert_eq!(N, out_index);

    return out;
}

const TEST_BOARD: &'static str = r#"
- - - - - - - - -
- - - - - - - - -
- - - - - - - - -
6 - - - 5 - - - 7
- - - - - - - - -
- - - - - - - - -
- - - - 4 - - - -
- - - - - - - - -
- - - - 1 - - - -
"#;

#[test]
fn test_get_column() {
    let board = Board::from_str(TEST_BOARD);
    let digits = board.column(4).0.map(|c| c.digit);
    assert_eq!(
        [
            None,
            None,
            None,
            Some(Digit::Five),
            None,
            None,
            Some(Digit::Four),
            None,
            Some(Digit::One),
        ],
        digits
    );
}

#[test]
fn test_get_row() {
    let board = Board::from_str(TEST_BOARD);
    let digits = board.row(3).0.map(|c| c.digit);
    assert_eq!(
        [
            Some(Digit::Six),
            None,
            None,
            None,
            Some(Digit::Five),
            None,
            None,
            None,
            Some(Digit::Seven),
        ],
        digits
    );
}
