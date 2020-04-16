extern crate rand;

use rand::prelude::*;
use std::vec::Vec;

/**
 * returns a 2 dimensional array filled with random numbers
 */
fn generate_grid() -> [[u8; 9]; 9] {
    let mut rng = rand::thread_rng();

    let mut grid: [[u8; 9]; 9] = [[0; 9]; 9];

    for x in grid.iter_mut() {
        for y in x.iter_mut() {
            *y = rng.gen_range(1, 9);
        }
    }

    grid
}

pub struct Block {
    grid: [[u8; 3]; 3],
}

impl Block {
    pub fn solved(&self) -> bool {
        let mut grid: Vec<u8> = vec![0 as u8; 9];
        let mut i = 0;
        for x in self.grid.iter() {
            for y in x.iter() {
                grid[i] = *y;
                i += 1;
            }
        }

        // thanks floppy!
        let rev_grid: Vec<u8> = grid.iter().copied().rev().collect();

        for x in grid.iter() {
            let mut i: u8 = 0;
            for y in rev_grid.iter() {
                if x == y {
                    i += 1;
                    if i == 2 {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn to_string(&self) {
        for x in self.grid.iter() {
            for y in x.iter() {
                print!("{}", y.to_string());
            }
            println!();
        }
    }
}

#[derive(Clone)]
pub struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {
            grid: generate_grid(),
        }
    }

    pub fn new_from_grid(new_grid: [[u8; 9]; 9]) -> Sudoku {
        Sudoku { grid: new_grid }
    }

    ///**
    // * Solves the Sudoku.
    // */
    //pub fn solve(self) {
    //    let iterations: u64 = 0;
    //    Sudoku::solve_rec(self, iterations);
    //}

    ///**
    // * A recursive helper method for solve()
    // */
    //fn solve_rec(sudoku: mut Sudoku, mut iterations: u64) -> bool {
    //    let mut current_number = 1;
    //    for x in sudoku.grid.iter_mut() {
    //        for y in x.iter_mut() {
    //            if *y == 0 {
    //                *y = current_number;
    //            } else {
    //                iterations += 1;
    //                if Sudoku::solved(){
    //                    current_number += 1;
    //                    *y = current_number;
    //                    Sudoku::solve_rec(sudoku.clone(), iterations);
    //                } else {
    //                    return true;
    //                }
    //            }
    //        }
    //    }
    //}

    /**
     * takes three from column from index i and stores them in a Block
     */
    pub fn take_block(&self, x: usize, y: usize) -> Block {
        let take_three_from_index =
            |col: [u8; 9], i: usize| [col[i], col[i + 1], col[i + 2]];

        // takes block of 3x3 from coordinates
        // x = width coord
        // y = height coord
        let new_block = [
            take_three_from_index(self.grid[y], x),
            take_three_from_index(self.grid[y + 1], x),
            take_three_from_index(self.grid[y + 2], x),
        ];
        Block { grid: new_block }
    }

    /**
     * Tests if a sudoku is solved or not.
     */
    pub fn solved(sudoku: Sudoku) -> bool {
        // we need to part the game field in 9 boxes.
        // those we test first
        // every row, column, and box needs to have every single number from
        // 1 - 9 in them without duplicates
        let blocks: [Block; 9] = [
            sudoku.take_block(0, 0),
            sudoku.take_block(3, 0),
            sudoku.take_block(6, 0),
            sudoku.take_block(0, 3),
            sudoku.take_block(3, 3),
            sudoku.take_block(6, 3),
            sudoku.take_block(0, 6),
            sudoku.take_block(3, 6),
            sudoku.take_block(6, 6),
        ];

        for block in blocks.iter() {
            block.to_string();
            if !block.solved() {
                println!("Block not solved.");
                return false;
            }
        }

        true
    }

    /**
     * prints the game grid in a fancy way
     */
    pub fn to_string(&self) {
        println!("+---+---+---+");
        let mut i: u8 = 1;
        for x in &self.grid {
            print!("|");
            for (k, y) in x.iter().enumerate() {
                if k % 3 == 0 && k != 0 {
                    print!("|");
                }
                if *y != 0 {
                    print!("{}", y.to_string());
                } else {
                    print!(" ");
                }
            }
            println!("|");
            if i % 3 == 0 && i != 9 {
                println!("|---+---+---|");
            }
            i += 1;
        }
        println!("+-----------+");
    }
}
