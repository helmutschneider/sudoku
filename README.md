# Sudoku solver
Simple and stupid sudoku solver. Does not implement any clever techniques (yet). No brute forcing.

```rust
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
let mut board = Board::from_str(str);
let solver = Solver::new();
let ok = solver.solve(&mut board);

println!("{}", ok);
```
