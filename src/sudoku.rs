extern crate rand;

use rand::prelude::*;

/**
 * returns a 2 dimensional array filled with random numbers
 */
fn generate_grid() -> [[u8; 9]; 9] {
    let mut rng = rand::thread_rng();

    let mut grid: [[u8; 9]; 9] = [[0; 9]; 9];

    for x in grid.iter_mut() {
        for y in x.iter_mut() {
            *y = rng.gen_range(0, 9);
        }
    }

    grid
}

#[derive(Clone)]
pub struct Sudoku {
    grid: [[u8; 9]; 9]
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {
            grid: generate_grid()
        }
    }

    pub fn new_from_grid(new_grid: [[u8; 9]; 9]) -> Sudoku {
        Sudoku {
            grid: new_grid
        }
    }

    pub fn solve(&self) -> Sudoku {
        let solved = self.clone();
        return solved;
    }

    /**
     * prints the game grid in a fancy way
     */
    pub fn to_string(&self) {
        println!("+---+---+---+");
        let mut i: u8 = 1;
        for x in &self.grid {
            print!("|");
            let mut k: u8 = 0;
            for y in x {
                if k % 3 == 0 && k != 0 {
                    print!("|");
                }
                if *y != 0 {
                    print!("{}", y.to_string());
                } else {
                    print!(" ");
                }
                k += 1;
            }
            print!("|\n");
            if i % 3 == 0 && i != 9 {
                print!("|---+---+---|\n");
            }
            i += 1;
        }
        println!("+-----------+");
    }
}
