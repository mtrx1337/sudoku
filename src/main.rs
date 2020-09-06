mod sudoku;
mod test;

use test::Tests;

fn main() {
    let tests = Tests {};
    tests.test_all();
}
