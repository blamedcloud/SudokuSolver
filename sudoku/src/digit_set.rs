use crate::digit::Digit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DigitSet {
    // essentially a bitvector
    data: u16,
    len: u8,
}

impl DigitSet {
    pub fn new() -> Self {
        Self {
            // 2^1 + 2^2 + ... + 2^9
            data: 1022,
            len: 9,
        }
    }

    pub fn empty() -> Self {
        Self {
            data: 0,
            len: 0,
        }
    }

    pub fn insert(&mut self, digit: Digit) {
        if !self.contains(digit) {
            self.data |= 1 << digit.number();
            self.len += 1;
        }
    }

    pub fn insert_all(&mut self, digits: &DigitSet) {
        self.data |= digits.get_data();
        self.len = self.data.count_ones() as u8;
    }

    pub fn remove(&mut self, digit: Digit) {
        if self.contains(digit) {
            self.data &= !(1 << digit.number());
            self.len -= 1;
        }
    }

    pub fn remove_all(&mut self, digits: &DigitSet) {
        self.data = self.data & (self.data ^ digits.get_data());
        self.len = self.data.count_ones() as u8;
    }

    pub fn contains(&self, digit: Digit) -> bool {
        // checks if the correct bit is set.
        ((self.data >> digit.number()) & 1) == 1
    }

    fn get_data(&self) -> u16 {
        self.data
    }

    pub fn size(&self) -> u8 {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use crate::digit::Digit;
    use crate::digit_set::DigitSet;

    #[test]
    fn test_new() {
        let ds = DigitSet::new();
        assert_eq!(ds.get_data(), 1022);
        assert_eq!(ds.size(), 9);

        let empty = DigitSet::empty();
        assert_eq!(empty.get_data(), 0);
        assert_eq!(empty.size(), 0);
    }

    #[test]
    fn test_contains() {
        let ds = DigitSet::new();
        assert!(ds.contains(Digit::One));
        assert!(ds.contains(Digit::Two));
        assert!(ds.contains(Digit::Three));
        assert!(ds.contains(Digit::Four));
        assert!(ds.contains(Digit::Five));
        assert!(ds.contains(Digit::Six));
        assert!(ds.contains(Digit::Seven));
        assert!(ds.contains(Digit::Eight));
        assert!(ds.contains(Digit::Nine));

        let ds = DigitSet::empty();
        assert!(!ds.contains(Digit::One));
        assert!(!ds.contains(Digit::Two));
        assert!(!ds.contains(Digit::Three));
        assert!(!ds.contains(Digit::Four));
        assert!(!ds.contains(Digit::Five));
        assert!(!ds.contains(Digit::Six));
        assert!(!ds.contains(Digit::Seven));
        assert!(!ds.contains(Digit::Eight));
        assert!(!ds.contains(Digit::Nine));
    }

    #[test]
    fn test_insert() {
        let mut ds = DigitSet::empty();

        ds.insert(Digit::Four);
        assert!(ds.contains(Digit::Four));
        assert_eq!(ds.size(), 1);
        assert_eq!(ds.get_data(), 16);

        ds.insert(Digit::Seven);
        assert!(ds.contains(Digit::Four));
        assert!(ds.contains(Digit::Seven));
        assert_eq!(ds.size(), 2);
        assert_eq!(ds.get_data(), 144);

        ds.insert(Digit::Four);
        assert!(ds.contains(Digit::Four));
        assert!(ds.contains(Digit::Seven));
        assert_eq!(ds.size(), 2);
        assert_eq!(ds.get_data(), 144);

        ds.insert(Digit::One);
        assert!(ds.contains(Digit::One));
        assert!(ds.contains(Digit::Four));
        assert!(ds.contains(Digit::Seven));
        assert_eq!(ds.size(), 3);
        assert_eq!(ds.get_data(), 146);
    }

    #[test]
    fn test_insert_all() {
        let mut ds1 = DigitSet::empty();

        let mut ds2 = DigitSet::empty();
        ds2.insert(Digit::One);
        ds2.insert(Digit::Four);
        ds2.insert(Digit::Seven);

        ds1.insert_all(&ds2);
        assert_eq!(ds1.size(), 3);
        assert_eq!(ds1.get_data(), 146);

        let mut ds3 = DigitSet::empty();
        ds3.insert(Digit::Four);
        ds3.insert(Digit::Five);
        ds3.insert(Digit::Six);

        ds1.insert_all(&ds3);
        assert_eq!(ds1.size(), 5);
        assert_eq!(ds1.get_data(), 242);
    }

    #[test]
    fn test_remove() {
        let mut ds = DigitSet::new();

        ds.remove(Digit::Nine);
        assert!(!ds.contains(Digit::Nine));
        assert_eq!(ds.size(), 8);
        assert_eq!(ds.get_data(), 510);

        ds.remove(Digit::Six);
        assert!(!ds.contains(Digit::Six));
        assert_eq!(ds.size(), 7);
        assert_eq!(ds.get_data(), 446);

        ds.remove(Digit::Nine);
        assert!(!ds.contains(Digit::Nine));
        assert_eq!(ds.size(), 7);
        assert_eq!(ds.get_data(), 446);
    }

    #[test]
    fn test_remove_all() {
        let mut ds = DigitSet::new();

        let mut ds2 = DigitSet::empty();
        ds2.insert(Digit::Two);
        ds2.insert(Digit::Five);
        ds2.insert(Digit::Eight);

        ds.remove_all(&ds2);
        assert_eq!(ds.size(), 6);
        assert_eq!(ds.get_data(), 730);

        let mut ds3 = DigitSet::empty();
        ds3.insert(Digit::One);
        ds3.insert(Digit::Two);
        ds3.insert(Digit::Three);

        ds.remove_all(&ds3);
        assert_eq!(ds.size(), 4);
        assert_eq!(ds.get_data(), 720);
    }
}
