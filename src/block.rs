#![allow(dead_code)]

pub struct Block {
    pub grid: [[u8; 3]; 3],
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
