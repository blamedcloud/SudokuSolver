use sudoku::digit::Digit;
use sudoku::grid::Grid;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GivenDigit {
    row: Digit,
    col: Digit,
    digit: Digit,
}

impl GivenDigit {
    pub fn new(row: Digit, col: Digit, digit: Digit) -> Self {
        Self { row, col, digit }
    }
}

pub struct Solver {
    given_digits: Vec<GivenDigit>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            given_digits: Vec::new(),
        }
    }

    pub fn from_givens(given_digits: Vec<GivenDigit>) -> Self {
        Self { given_digits }
    }

    pub fn add_given(&mut self, digit: GivenDigit) {
        self.given_digits.push(digit);
    }

    pub fn solve(&self) -> Grid {
        let mut grid = Grid::new();
        //TODO:
        grid
    }
}
