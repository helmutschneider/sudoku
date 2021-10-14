use crate::board::Board;
use crate::cell::Cell;
use crate::digit::Digit;
use crate::index::{CellIndex, ColumnIndex, RowIndex};
use std::collections::HashSet;

const STRATEGIES: [&dyn Strategy; 1] = [&RelatedCellsStrategy {}];

pub struct Solver<'a> {
    strategies: &'a [&'a dyn Strategy],
}

impl<'a> Solver<'a> {
    pub fn new() -> Self {
        return Self {
            strategies: &STRATEGIES,
        };
    }

    pub fn solve(&self, board: &mut Board) -> bool {
        let data = board.data;
        let cells = data.iter().flatten().collect::<Vec<&Cell>>();

        while !board.is_solved() {
            let mut solution = None;

            for cell in &cells {
                if cell.digit.is_some() {
                    continue;
                }
                let mut possible = PossibleDigits::new();
                for strat in self.strategies {
                    strat.eliminate_digits(board, cell, &mut possible);
                    solution = possible.get_solution();
                    if let Some(d) = solution {
                        board.set_digit(cell.index, d);
                        break;
                    }
                }
                if solution.is_some() {
                    break;
                }
            }

            if solution.is_none() {
                // we went through all our strategies but did not find a solution anywhere. supabad!
                return false;
            }
        }

        return true;
    }
}

#[derive(Debug)]
struct PossibleDigits {
    digits: HashSet<Digit>,
}

impl PossibleDigits {
    fn new() -> Self {
        let mut stuff = HashSet::new();
        stuff.extend(Digit::ALL_DIGITS);

        return Self { digits: stuff };
    }

    fn remove(&mut self, digit: Digit) {
        self.digits.remove(&digit);
    }

    fn get_solution(&self) -> Option<Digit> {
        if self.digits.len() == 1 {
            for digit in &self.digits {
                return Some(*digit);
            }
        }
        return None;
    }
}

trait Strategy {
    fn eliminate_digits(&self, board: &Board, cell: &Cell, possible: &mut PossibleDigits);
}

// this struct implements the simplest possible elimination strategy
// known to any sudoku player. it looks at digits already used in the
// current row, column and 3x3 section and removes those from the
// possible solutions.
struct RelatedCellsStrategy {}

impl Strategy for RelatedCellsStrategy {
    fn eliminate_digits(&self, board: &Board, cell: &Cell, possible: &mut PossibleDigits) {
        assert!(cell.digit.is_none());

        let index = cell.index;
        let mut related_cells = Vec::new();
        related_cells.extend_from_slice(&board.get(index.0).cells);
        related_cells.extend_from_slice(&board.get(index.1).cells);
        related_cells.extend_from_slice(&board.get(index.section()).cells);

        for c in related_cells {
            if let Some(digit) = c.digit {
                possible.remove(digit);
            }
        }
    }
}

#[test]
fn related_cells_strategy_eliminate_digits() {
    let strat = RelatedCellsStrategy {};
    let board_str = r#"
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
    let board = Board::from_str(board_str);
    let mut possible = PossibleDigits::new();
    let cell = board.get(CellIndex(RowIndex(5), ColumnIndex(7)));
    strat.eliminate_digits(&board, &cell, &mut possible);

    assert_eq!(Some(Digit::Four), possible.get_solution());
}
