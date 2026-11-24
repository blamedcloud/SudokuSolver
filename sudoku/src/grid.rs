use std::fmt::Display;
use crate::cell::Cell;
use crate::digit::Digit;

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
        todo!()
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
    use crate::digit::Digit;
    use crate::grid::Grid;

    #[test]
    fn test_get_index() {
        assert_eq!(Grid::get_index(Digit::One, Digit::One), 0);
        assert_eq!(Grid::get_index(Digit::Nine, Digit::Nine), 80);
        assert_eq!(Grid::get_index(Digit::Three, Digit::Seven), 24);
    }

    #[test]
    fn test_display() {
        let grid = Grid::new();
        println!("{}", grid);
    }
}
