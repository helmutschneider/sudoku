use crate::{
    cell::Cell,
    digit::Digit,
    index::{CellIndex, ColumnIndex, IndexLike, RowIndex},
};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub data: [[Cell; 9]; 9],
}

impl Board {
    pub fn new(data: [[Cell; 9]; 9]) -> Self {
        return Self { data };
    }

    pub fn from_str(value: &str) -> Self {
        let mut data: [[Cell; 9]; 9] = [[Cell::EMPTY; 9]; 9];
        let mut index: usize = 0;

        for ch in value.chars() {
            let column = ColumnIndex(index % 9);
            let row = RowIndex(((index as f64) / 9.0).floor() as usize);
            let out: Option<Cell> = match ch {
                Digit::EMPTY_CHARACTER => Some(Cell {
                    digit: None,
                    index: CellIndex(row, column),
                }),
                _ => {
                    let maybe_digit = Digit::from_char(ch);
                    if maybe_digit.is_some() {
                        Some(Cell {
                            digit: maybe_digit,
                            index: CellIndex(row, column),
                        })
                    } else {
                        None
                    }
                }
            };
            if let Some(cell) = out {
                data[row.0][column.0] = cell;
                index += 1;
            }
        }

        assert_eq!(81, index);

        return Board::new(data);
    }

    pub fn get<T>(&self, index: T) -> T::Output
    where
        T: IndexLike,
    {
        return index.get(self);
    }

    pub fn set_digit(&mut self, index: CellIndex, digit: Digit) {
        let cell = &mut self.data[index.0 .0][index.1 .0];
        cell.digit = Some(digit);
    }

    pub fn is_solved(&self) -> bool {
        return self.data.iter().flatten().all(|c| c.digit.is_some());
    }

    pub fn get_possible_digits(&self, index: CellIndex) -> HashSet<Digit> {
        let mut stuff = HashSet::new();
        let cell = self.get(index);

        if cell.digit.is_some() {
            return stuff;
        }

        for digit in Digit::ALL_DIGITS {
            stuff.insert(digit);
        }

        let mut related_cells = Vec::new();
        related_cells.extend_from_slice(&self.get(index.0).cells);
        related_cells.extend_from_slice(&self.get(index.1).cells);
        related_cells.extend_from_slice(&self.get(index.section()).cells);

        for c in related_cells {
            if let Some(digit) = c.digit {
                stuff.remove(&digit);
            }
        }

        return stuff;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            let row = RowIndex(i);
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

#[test]
fn test_get_column() {
    let str = r#"
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
    let board = Board::from_str(str);
    let digits = board.get(ColumnIndex(4)).cells.map(|c| c.digit);
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
    let str = r#"
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
    let board = Board::from_str(str);
    let digits = board.get(RowIndex(3)).cells.map(|c| c.digit);
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

#[test]
fn test_set_digit() {
    let str = r#"
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
    let mut board = Board::from_str(str);
    let index = CellIndex(RowIndex(2), ColumnIndex(2));
    board.set_digit(index, Digit::Five);

    assert_eq!(Some(Digit::Five), board.get(index).digit);
}

#[test]
fn test_get_possible_digits_one() {
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
    let idx = CellIndex(RowIndex(0), ColumnIndex(1));
    let possible = board.get_possible_digits(idx);
    let mut set = HashSet::new();
    set.insert(Digit::Two);
    set.insert(Digit::Four);
    set.insert(Digit::Six);

    assert_eq!(set, possible);
}

#[test]
fn test_get_possible_digits_two() {
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
    let idx = CellIndex(RowIndex(8), ColumnIndex(8));
    let possible = board.get_possible_digits(idx);
    let mut set = HashSet::new();
    set.insert(Digit::Five);
    set.insert(Digit::Eight);

    assert_eq!(set, possible);
}
