use crate::{
    cell::{Cell, CellList},
    digit::Digit,
    index::{Column, Index, Row},
};
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub data: [Cell; 81],
}

fn get_flattened_index(row: Row, column: Column) -> usize {
    return (row.0 * 9) + column.0;
}

impl Board {
    pub fn new(data: [Cell; 81]) -> Self {
        return Self { data };
    }

    pub fn from_str(value: &str) -> Self {
        let mut data: [Cell; 81] = [Cell::EMPTY; 81];
        let mut index: usize = 0;

        for ch in value.chars() {
            let column = Column(index % 9);
            let row = Row(((index as f64) / 9.0).floor() as usize);
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

    pub fn get<T>(&self, index: T) -> T::Output
    where
        T: Index,
    {
        return index.get(self);
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
            let row = Row(i);
            let str = self.get(row).cells.map(|c| c.to_string()).join(" ");
            if i < 8 {
                writeln!(f, "{}", str)?;
            } else {
                write!(f, "{}", str)?;
            }
        }
        return fmt::Result::Ok(());
    }
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
    let digits = board.get(Column(4)).cells.map(|c| c.digit);
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
    let digits = board.get(Row(3)).cells.map(|c| c.digit);
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
