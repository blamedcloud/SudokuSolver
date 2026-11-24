use crate::digit::Digit;
use crate::digit_set::DigitSet;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    digit: Option<Digit>,
    possibilities: DigitSet,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            digit: None,
            possibilities: DigitSet::new(),
        }
    }

    pub fn get_digit(&self) -> Option<Digit> {
        self.digit
    }

    pub fn set_digit(&mut self, digit: Digit) {
        self.digit = Some(digit);
        let mut ds = DigitSet::empty();
        ds.insert(digit);
        self.possibilities = ds;
    }

    pub fn get_possibilities(&self) -> &DigitSet {
        &self.possibilities
    }

    pub fn get_num_possibilities(&self) -> u8 {
        self.possibilities.size()
    }

    pub fn remove_possibility(&mut self, digit: Digit) {
        self.possibilities.remove(digit);
    }

    pub fn remove_possibilities(&mut self, digits: DigitSet) {
        self.possibilities.remove_all(&digits);
    }
}
