mod board;
mod cell;
mod digit;
mod index;
mod solver;

use board::Board;
use solver::Solver;

const BOARDS_TO_SOLVE: &[&'static str] = &[
    r#"
    9 - - 8 3 - 1 5 7
    5 - 3 1 - 6 2 8 -
    1 - - 7 4 - - 9 -
    - - - - 5 - 8 3 -
    3 - 1 - - 4 6 7 2
    2 - - - 1 3 - - 9
    - - 2 - 7 - - 1 -
    - - - - - - - 6 -
    - 3 4 - 6 - 9 2 -
        "#,
    r#"
    5 - - - - - - - 9
    - - 9 3 - - - - -
    - 2 7 - - - 1 - -
    4 - - 5 - - 3 - 8
    - 1 - - - 6 - 5 7
    - - 3 - - - 9 - -
    9 - - - 4 5 - - 3
    1 - - - 7 - - - -
    - - - - - - 6 - 5
    "#,
];

fn main() {
    let solver = Solver::new();
    for str in BOARDS_TO_SOLVE {
        let mut board = Board::from_str(str);
        println!("Solving...");
        if solver.solve(&mut board) {
            println!("Ok!\n{}", board);
        } else {
            println!("Mega fail :(");
        }
    }
}
