mod common;

#[allow(unused_imports)]
use common::setup;
use sudokusolve::*;

#[test]
fn test_listofnine() {
    let list_valid = ListOfNine::new([4, 3, 5, 2, 6, 9, 7, 8, 1]);
    let list_valid_col = ListOfNine::new_column([4, 3, 5, 2, 6, 9, 7, 8, 1]);
    let list_invalid = ListOfNine::new([3, 3, 5, 2, 6, 9, 7, 8, 1]);
    let list_zeros = ListOfNine::new_zeros();
    let list_zeros_col = ListOfNine::new_zeros_column();

    assert_eq!(list_valid.solved(), true);
    assert_eq!(list_valid_col.solved(), true);
    assert_eq!(list_invalid.solved(), false);
    assert_eq!(list_zeros.solved(), false);
    assert_eq!(list_zeros_col.solved(), false);
}

#[test]
fn test_block() {
    let block_valid = Block::new([[4, 3, 5], [6, 8, 2], [1, 9, 7]]);
    let block_invalid = Block::new([[4, 4, 5], [6, 8, 2], [1, 9, 7]]);
    let block_zeros = Block::new_zeros();

    assert_eq!(block_valid.solved(), true);
    assert_eq!(block_invalid.solved(), false);
    assert_eq!(block_zeros.solved(), false);
}

#[test]
fn test_sudoku() {
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

    let unfinished_game = Sudoku::new_from_grid(unfinished_grid, true);
    let finished_game = Sudoku::new_from_grid(finished_grid, true);

    unfinished_game.to_string();
    assert_eq!(unfinished_game.solved(), false);

    finished_game.to_string();
    assert_eq!(finished_game.solved(), true);
    //assert_eq!(unfinished_game.solve(), true);
}
