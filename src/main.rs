mod board;
mod cell;
mod digit;
mod index;
mod solver;

use board::Board;
use solver::Solver;
use solver::Strategies;

fn main() {
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
    let mut board = Board::from_str(&str);
    let solver = Solver::new([&Strategies::SINGLE_POSSIBLE_DIGIT]);

    // let ok = solver.solve(&mut board);

    // println!("{}", board);
}
