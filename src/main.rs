mod sudoku;

use sudoku::Sudoku;

fn main() {
    let unfinished_grid: [[u8; 9]; 9] = [
        [2, 0, 5, 0, 0, 9, 0, 0, 4],
        [0, 0, 0, 0, 0, 0, 3, 0, 7],
        [7, 0, 0, 8, 5, 6, 0, 1, 0],
        [4, 5, 0, 7, 0, 0, 0, 0, 0],
        [0, 0, 9, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 8, 5],
        [0, 2, 0, 4, 1, 8, 0, 0, 6],
        [6, 0, 8, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 2, 0, 0, 7, 0, 8],
    ];

    let finished_grid: [[u8; 9]; 9] = [
        [4, 3, 5, 2, 6, 9, 7, 8, 1],
        [6, 8, 2, 5, 7, 1, 4, 9, 3],
        [1, 9, 7, 8, 3, 4, 5, 6, 2],
        [8, 2, 6, 1, 9, 5, 3, 4, 7],
        [3, 7, 4, 6, 8, 2, 9, 1, 5],
        [9, 5, 1, 7, 4, 3, 6, 2, 8],
        [5, 1, 9, 3, 2, 6, 8, 7, 4],
        [2, 4, 8, 9, 5, 7, 1, 3, 6],
        [7, 6, 3, 4, 1, 8, 2, 5, 9],
    ];

    let mut unfinished_game = Sudoku::new_from_grid(unfinished_grid, false);
    let mut finished_game = Sudoku::new_from_grid(finished_grid, true);

    println!("Is this unfinished game solved? It should not be...");
    if finished_game.solved() {
        println!("unfinished_game is solved");
    } else {
        println!("unfinished_game is not solved");
    }

    println!("Trying to solve the unfinished_game...");
    //unfinished_game.solve();
    println!("Is this finished game solved? It should be...");
    if Sudoku::solved(&unfinished_game) {
        println!("finished_game is solved");
    } else {
        println!("finished_game is not solved");
    }
}
