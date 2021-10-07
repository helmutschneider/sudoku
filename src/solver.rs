use crate::board::Board;
use crate::cell::Cell;
use crate::digit::Digit;

type StrategyList<'a, const N: usize> = [&'a dyn Strategy; N];

pub struct Solver<'a, const N: usize> {
    strategies: StrategyList<'a, N>,
}

impl<'a, const N: usize> Solver<'a, N> {
    pub fn new(strategies: StrategyList<'a, N>) -> Self {
        return Self { strategies };
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
                        board.set_digit_at_cell(cell, digit);
                        break;
                    }
                }
                if did_find_solution {
                    break;
                }
            }

            if !did_find_solution {
                // we went through all our strategies but did not find a solution anywhere. bad!
                return false;
            }
        }

        return true;
    }
}

pub trait Strategy {
    fn solve_cell(&self, board: &Board, cell: Cell) -> Option<Digit>;
}

pub struct Strategies {}
impl Strategies {
    pub const SinglePossibleDigit: SinglePossibleDigitStrategy = SinglePossibleDigitStrategy {};
}

pub struct SinglePossibleDigitStrategy {}

impl Strategy for SinglePossibleDigitStrategy {
    fn solve_cell(&self, board: &Board, cell: Cell) -> Option<Digit> {
        return Some(Digit::Eight);
    }
}
