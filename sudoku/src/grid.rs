use std::fmt::Display;
use crate::cell::Cell;
use crate::cell_set::CellSet;
use crate::digit::Digit;
use crate::digit_set::DigitSet;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Grid {
    cells: [Cell; 81],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: std::array::from_fn(|_| Cell::new())
        }
    }

    fn get_index(row: Digit, column: Digit) -> usize {
        9*(row.number() as usize - 1) + (column.number() as usize - 1)
    }

    pub fn get_cell(&self, row: Digit, column: Digit) -> &Cell {
        &self.cells[Self::get_index(row, column)]
    }

    fn get_cell_by_index(&self, index: usize) -> Option<&Cell> {
        self.cells.get(index)
    }

    pub fn get_cell_mut(&mut self, row: Digit, column: Digit) -> &mut Cell {
        &mut self.cells[Self::get_index(row, column)]
    }

    pub fn is_fully_marked(&self) -> bool {
        for i in 0..81 {
            if self.cells[i].get_digit().is_none() {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        for digit in Digit::iterator() {
            if self.has_duplicates(CellSet::new_row(digit)) {
                return false;
            }
            if self.has_duplicates(CellSet::new_column(digit)) {
                return false;
            }
            if self.has_duplicates(CellSet::new_box(digit)) {
                return false;
            }
        }
        true
    }

    fn has_duplicates(&self, cells: CellSet) -> bool {
        let mut digits = DigitSet::empty();
        for (row, col) in cells.into_iter() {
            let digit = self.get_cell(row, col).get_digit();
            if digit.is_some() {
                if digits.contains(digit.unwrap()) {
                    return true;
                } else {
                    digits.insert(digit.unwrap());
                }
            }
        }
        false
    }

    pub fn is_complete(&self) -> bool {
        self.is_fully_marked() && self.is_valid()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..=8 {
            for column in 0..=8 {
                let index = 9*row + column;
                let cell = self.get_cell_by_index(index).unwrap();
                if cell.get_digit().is_some() {
                    write!(f, "{}", cell.get_digit().unwrap().number())?;
                } else {
                    write!(f, " ")?;
                }
                if column == 2 || column == 5 {
                    write!(f, "|")?;
                }
            }
            write!(f, "\n")?;
            if row == 2 || row == 5 {
                write!(f, "---+---+---\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_set::CellSet;
    use crate::digit::Digit;
    use crate::grid::Grid;

    static SHOW: bool = true;

    #[test]
    fn test_get_index() {
        assert_eq!(Grid::get_index(Digit::One, Digit::One), 0);
        assert_eq!(Grid::get_index(Digit::Nine, Digit::Nine), 80);
        assert_eq!(Grid::get_index(Digit::Three, Digit::Seven), 24);
    }

    #[test]
    fn test_display() {
        let grid = Grid::new();
        if SHOW {
            println!("{}", grid);
        }
        assert!(grid.is_valid());
        assert!(!grid.is_fully_marked());
        assert!(!grid.is_complete());
    }

    #[test]
    fn test_invalid() {
        let mut grid1 = Grid::new();
        grid1.get_cell_mut(Digit::Three, Digit::Two).set_digit(Digit::One);
        grid1.get_cell_mut(Digit::Three, Digit::Seven).set_digit(Digit::One);
        if SHOW {
            println!("{}", grid1);
        }
        assert!(grid1.has_duplicates(CellSet::new_row(Digit::Three)));
        assert!(!grid1.is_valid());

        let mut grid2 = Grid::new();
        grid2.get_cell_mut(Digit::Six, Digit::Five).set_digit(Digit::Four);
        grid2.get_cell_mut(Digit::Nine, Digit::Five).set_digit(Digit::Four);
        if SHOW {
            println!("{}", grid2);
        }
        assert!(grid2.has_duplicates(CellSet::new_column(Digit::Five)));
        assert!(!grid2.is_valid());

        let mut grid3 = Grid::new();
        grid3.get_cell_mut(Digit::Seven, Digit::Seven).set_digit(Digit::Nine);
        grid3.get_cell_mut(Digit::Nine, Digit::Eight).set_digit(Digit::Nine);
        if SHOW {
            println!("{}", grid3);
        }
        assert!(grid3.has_duplicates(CellSet::new_box(Digit::Nine)));
        assert!(!grid3.is_valid());
    }

    #[test]
    fn test_complete() {
        let mut grid = Grid::new();
        grid.get_cell_mut(Digit::One, Digit::One).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::One, Digit::Two).set_digit(Digit::One);
        grid.get_cell_mut(Digit::One, Digit::Three).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::One, Digit::Four).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::One, Digit::Five).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::One, Digit::Six).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::One, Digit::Seven).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::One, Digit::Eight).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::One, Digit::Nine).set_digit(Digit::Nine);

        grid.get_cell_mut(Digit::Two, Digit::One).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Two, Digit::Two).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Two, Digit::Three).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Two, Digit::Four).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Two, Digit::Five).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Two, Digit::Six).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Two, Digit::Seven).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Two, Digit::Eight).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Two, Digit::Nine).set_digit(Digit::Three);

        grid.get_cell_mut(Digit::Three, Digit::One).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Three, Digit::Two).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Three, Digit::Three).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Three, Digit::Four).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Three, Digit::Five).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Three, Digit::Six).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Three, Digit::Seven).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Three, Digit::Eight).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Three, Digit::Nine).set_digit(Digit::Eight);

        grid.get_cell_mut(Digit::Four, Digit::One).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Four, Digit::Two).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Four, Digit::Three).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Four, Digit::Four).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Four, Digit::Five).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Four, Digit::Six).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Four, Digit::Seven).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Four, Digit::Eight).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Four, Digit::Nine).set_digit(Digit::Four);

        grid.get_cell_mut(Digit::Five, Digit::One).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Five, Digit::Two).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Five, Digit::Three).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Five, Digit::Four).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Five, Digit::Five).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Five, Digit::Six).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Five, Digit::Seven).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Five, Digit::Eight).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Five, Digit::Nine).set_digit(Digit::Two);

        grid.get_cell_mut(Digit::Six, Digit::One).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Six, Digit::Two).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Six, Digit::Three).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Six, Digit::Four).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Six, Digit::Five).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Six, Digit::Six).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Six, Digit::Seven).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Six, Digit::Eight).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Six, Digit::Nine).set_digit(Digit::Six);

        grid.get_cell_mut(Digit::Seven, Digit::One).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Seven, Digit::Two).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Seven, Digit::Three).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Seven, Digit::Four).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Seven, Digit::Five).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Seven, Digit::Six).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Seven, Digit::Seven).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Seven, Digit::Eight).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Seven, Digit::Nine).set_digit(Digit::One);

        grid.get_cell_mut(Digit::Eight, Digit::One).set_digit(Digit::Seven);
        grid.get_cell_mut(Digit::Eight, Digit::Two).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Eight, Digit::Three).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Eight, Digit::Four).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Eight, Digit::Five).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Eight, Digit::Six).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Eight, Digit::Seven).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Eight, Digit::Eight).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Eight, Digit::Nine).set_digit(Digit::Five);

        grid.get_cell_mut(Digit::Nine, Digit::One).set_digit(Digit::Three);
        grid.get_cell_mut(Digit::Nine, Digit::Two).set_digit(Digit::Four);
        grid.get_cell_mut(Digit::Nine, Digit::Three).set_digit(Digit::Nine);
        grid.get_cell_mut(Digit::Nine, Digit::Four).set_digit(Digit::Six);
        grid.get_cell_mut(Digit::Nine, Digit::Five).set_digit(Digit::One);
        grid.get_cell_mut(Digit::Nine, Digit::Six).set_digit(Digit::Five);
        grid.get_cell_mut(Digit::Nine, Digit::Seven).set_digit(Digit::Eight);
        grid.get_cell_mut(Digit::Nine, Digit::Eight).set_digit(Digit::Two);
        grid.get_cell_mut(Digit::Nine, Digit::Nine).set_digit(Digit::Seven);

        if SHOW {
            println!("{}", grid);
        }
        assert!(grid.is_complete());

        let mut bad_grid = grid.clone();
        bad_grid.get_cell_mut(Digit::Nine, Digit::Nine).set_digit(Digit::One);
        assert!(bad_grid.is_fully_marked());
        assert!(!bad_grid.is_valid());
    }
}
