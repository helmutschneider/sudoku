use crate::board::Board;
use crate::cell::Cell;
use crate::digit::Digit;

const STRATEGIES: [&dyn Strategy; 1] = [&SinglePossibleDigitStrategy {}];

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
        while !board.is_solved() {
            let mut did_find_solution = false;
            for cell in board.data {
                if cell.digit.is_some() {
                    continue;
                }
                for strat in self.strategies {
                    let maybe_digit = strat.solve_cell(board, cell);
                    if let Some(digit) = maybe_digit {
                        did_find_solution = true;
                        board.set_digit(cell.index, digit);
                        break;
                    }
                }
                if did_find_solution {
                    break;
                }
            }

            if !did_find_solution {
                // we went through all our strategies but did not find a solution anywhere. supabad!
                return false;
            }
        }

        return true;
    }
}

pub trait Strategy {
    fn solve_cell(&self, board: &Board, cell: Cell) -> Option<Digit>;
}

pub struct SinglePossibleDigitStrategy {}

impl Strategy for SinglePossibleDigitStrategy {
    fn solve_cell(&self, board: &Board, cell: Cell) -> Option<Digit> {
        let possible = board.get_possible_digits(cell.index);
        if possible.len() == 1 {
            // is there a more efficient way to extract an element from a HashSet?
            for dig in possible {
                return Some(dig);
            }
        }
        return None;
    }
}
