mod sudoku;

use sudoku::Sudoku;

fn main() {
    let game = Sudoku::new();
    game.to_string();

    let grid: [[u8; 9]; 9] =
        [
            [2, 0, 5, 0, 0, 9, 0, 0, 4],
            [0, 0, 0, 0, 0, 0, 3, 0, 7],
            [7, 0, 0, 8, 5, 6, 0, 1, 0],
            [4, 5, 0, 7, 0, 0, 0, 0, 0],
            [0, 0, 9, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 8, 5],
            [0, 2, 0, 4, 1, 8, 0, 0, 6],
            [6, 0, 8, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 2, 0, 0, 7, 0, 8]
        ];

    let easy_game = Sudoku::new_from_grid(grid);
    easy_game.solve();
    easy_game.to_string();
}
