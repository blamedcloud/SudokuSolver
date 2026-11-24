use crate::digit::{Digit, Entropy, Mod3};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CellSet {
    data: u128,
    len: u8,
}

const ROW1_DATA: u128 = 511;
const COL1_DATA: u128 = 1 + (1<<9) + (1<<18) + (1<<27) + (1<<36) + (1<<45) + (1<<54) + (1<<63) + (1<<72);
const BOX1_DATA: u128 = 1 + (1<<1) + (1<<2) + (1<<9) + (1<<10) + (1<<11) + (1<<18) + (1<<19) + (1<<20);

impl CellSet {
    pub fn new() -> Self {
        // unlike the digit set, the base case is the empty set
        Self {
            data: 0,
            len: 0,
        }
    }

    pub fn new_row(row: Digit) -> Self {
        let offset = 9*(row.number() as u128 - 1);
        let mut data = ROW1_DATA;
        data = data << offset;
        Self {
            data,
            len: 9,
        }
    }

    pub fn new_column(column: Digit) -> Self {
        let offset = column.number() as u128 - 1;
        let mut data = COL1_DATA;
        data = data << offset;
        Self {
            data,
            len: 9,
        }
    }

    pub fn new_box(box_num: Digit) -> Self {
        let entropy_offset = match box_num.into() {
            Entropy::Low => 0,
            Entropy::Middle => 27,
            Entropy::High => 54,
        };
        let mod_offset = match box_num.into() {
            Mod3::One => 0,
            Mod3::Two => 3,
            Mod3::Zero => 6,
        };
        let mut data = BOX1_DATA;
        data = data << (entropy_offset + mod_offset);
        Self {
            data,
            len: 9,
        }
    }

    pub fn get_default_vision(row: Digit, column: Digit) -> Self {
        let mut cells = CellSet::new_box(Digit::get_box_num(row, column));
        cells.insert_all(&CellSet::new_row(row));
        cells.insert_all(&CellSet::new_column(column));
        cells
    }

    fn get_index(row: Digit, column: Digit) -> u128 {
        9*(row.number() as u128 - 1) + (column.number() as u128 - 1)
    }

    pub fn insert(&mut self, row: Digit, column: Digit) {
        if !self.contains(row, column) {
            let index = Self::get_index(row, column);
            self.data |= 1 << index;
            self.len += 1;
        }
    }

    pub fn insert_all(&mut self, cells: &CellSet) {
        self.data |= cells.get_data();
        self.len = self.data.count_ones() as u8;
    }

    pub fn contains(&self, row: Digit, col: Digit) -> bool {
        let index = Self::get_index(row, col);
        ((self.data >> index) & 1) == 1
    }

    pub fn remove(&mut self, row: Digit, col: Digit) {
        if self.contains(row, col) {
            let index = Self::get_index(row, col);
            self.data &= !(1 << index);
            self.len -= 1;
        }
    }

    pub fn remove_all(&mut self, cells: &CellSet) {
        self.data = self.data & (self.data ^ cells.get_data());
        self.len = self.data.count_ones() as u8;
    }

    fn get_data(&self) -> u128 {
        self.data
    }

    pub fn size(&self) -> u8 {
        self.len
    }

    pub fn union(left: &CellSet, right: &CellSet) -> Self {
        let data = left.get_data() | right.get_data();
        Self {
            data,
            len: data.count_ones() as u8,
        }
    }

    pub fn intersection(left: &CellSet, right: &CellSet) -> Self {
        let data = left.get_data() & right.get_data();
        Self {
            data,
            len: data.count_ones() as u8,
        }
    }

    // gives the cells that are only in (left, right)
    pub fn unique_split(left: &CellSet, right: &CellSet) -> (Self, Self) {
        let intersection = CellSet::intersection(left, right);
        let mut new_left = left.clone();
        new_left.remove_all(&intersection);
        let mut new_right = right.clone();
        new_right.remove_all(&intersection);
        (new_left, new_right)
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_set::{CellSet, COL1_DATA, ROW1_DATA, BOX1_DATA};
    use crate::digit::Digit;

    #[test]
    fn test_constructors() {
        let cs = CellSet::new();
        assert_eq!(cs.get_data(), 0);
        assert_eq!(cs.size(), 0);

        let row1 = CellSet::new_row(Digit::One);
        assert_eq!(row1.get_data(), ROW1_DATA);
        assert_eq!(row1.size(), 9);
        assert_eq!(row1.get_data().count_ones() as u8, row1.size());
        assert!(row1.contains(Digit::One, Digit::One));
        assert!(row1.contains(Digit::One, Digit::Two));
        assert!(row1.contains(Digit::One, Digit::Three));
        assert!(row1.contains(Digit::One, Digit::Four));
        assert!(row1.contains(Digit::One, Digit::Five));
        assert!(row1.contains(Digit::One, Digit::Six));
        assert!(row1.contains(Digit::One, Digit::Seven));
        assert!(row1.contains(Digit::One, Digit::Eight));
        assert!(row1.contains(Digit::One, Digit::Nine));

        let row5 = CellSet::new_row(Digit::Five);
        assert_eq!(row5.get_data(), ROW1_DATA << (9*4));
        assert_eq!(row5.size(), 9);
        assert_eq!(row5.get_data().count_ones() as u8, row5.size());
        assert!(row5.contains(Digit::Five, Digit::One));
        assert!(row5.contains(Digit::Five, Digit::Two));
        assert!(row5.contains(Digit::Five, Digit::Three));
        assert!(row5.contains(Digit::Five, Digit::Four));
        assert!(row5.contains(Digit::Five, Digit::Five));
        assert!(row5.contains(Digit::Five, Digit::Six));
        assert!(row5.contains(Digit::Five, Digit::Seven));
        assert!(row5.contains(Digit::Five, Digit::Eight));
        assert!(row5.contains(Digit::Five, Digit::Nine));

        let row9 = CellSet::new_row(Digit::Nine);
        assert_eq!(row9.get_data(), ROW1_DATA << (9*8));
        assert_eq!(row9.size(), 9);
        assert_eq!(row9.get_data().count_ones() as u8, row9.size());
        assert!(row9.contains(Digit::Nine, Digit::One));
        assert!(row9.contains(Digit::Nine, Digit::Two));
        assert!(row9.contains(Digit::Nine, Digit::Three));
        assert!(row9.contains(Digit::Nine, Digit::Four));
        assert!(row9.contains(Digit::Nine, Digit::Five));
        assert!(row9.contains(Digit::Nine, Digit::Six));
        assert!(row9.contains(Digit::Nine, Digit::Seven));
        assert!(row9.contains(Digit::Nine, Digit::Eight));
        assert!(row9.contains(Digit::Nine, Digit::Nine));


        let col1 = CellSet::new_column(Digit::One);
        assert_eq!(col1.get_data(), COL1_DATA);
        assert_eq!(col1.size(), 9);
        assert_eq!(col1.get_data().count_ones() as u8, col1.size());
        assert!(col1.contains(Digit::One, Digit::One));
        assert!(col1.contains(Digit::Two, Digit::One));
        assert!(col1.contains(Digit::Three, Digit::One));
        assert!(col1.contains(Digit::Four, Digit::One));
        assert!(col1.contains(Digit::Five, Digit::One));
        assert!(col1.contains(Digit::Six, Digit::One));
        assert!(col1.contains(Digit::Seven, Digit::One));
        assert!(col1.contains(Digit::Eight, Digit::One));
        assert!(col1.contains(Digit::Nine, Digit::One));

        let col2 = CellSet::new_column(Digit::Two);
        assert_eq!(col2.get_data(), COL1_DATA << 1);
        assert_eq!(col2.size(), 9);
        assert_eq!(col2.get_data().count_ones() as u8, col2.size());
        assert!(col2.contains(Digit::One, Digit::Two));
        assert!(col2.contains(Digit::Two, Digit::Two));
        assert!(col2.contains(Digit::Three, Digit::Two));
        assert!(col2.contains(Digit::Four, Digit::Two));
        assert!(col2.contains(Digit::Five, Digit::Two));
        assert!(col2.contains(Digit::Six, Digit::Two));
        assert!(col2.contains(Digit::Seven, Digit::Two));
        assert!(col2.contains(Digit::Eight, Digit::Two));
        assert!(col2.contains(Digit::Nine, Digit::Two));

        let col7 = CellSet::new_column(Digit::Seven);
        assert_eq!(col7.get_data(), COL1_DATA << 6);
        assert_eq!(col7.size(), 9);
        assert_eq!(col7.get_data().count_ones() as u8, col7.size());
        assert!(col7.contains(Digit::One, Digit::Seven));
        assert!(col7.contains(Digit::Two, Digit::Seven));
        assert!(col7.contains(Digit::Three, Digit::Seven));
        assert!(col7.contains(Digit::Four, Digit::Seven));
        assert!(col7.contains(Digit::Five, Digit::Seven));
        assert!(col7.contains(Digit::Six, Digit::Seven));
        assert!(col7.contains(Digit::Seven, Digit::Seven));
        assert!(col7.contains(Digit::Eight, Digit::Seven));
        assert!(col7.contains(Digit::Nine, Digit::Seven));

        let box1 = CellSet::new_box(Digit::One);
        assert_eq!(box1.get_data(), BOX1_DATA);
        assert_eq!(box1.size(), 9);
        assert_eq!(box1.get_data().count_ones() as u8, box1.size());
        assert!(box1.contains(Digit::One, Digit::One));
        assert!(box1.contains(Digit::One, Digit::Two));
        assert!(box1.contains(Digit::One, Digit::Three));
        assert!(box1.contains(Digit::Two, Digit::One));
        assert!(box1.contains(Digit::Two, Digit::Two));
        assert!(box1.contains(Digit::Two, Digit::Three));
        assert!(box1.contains(Digit::Three, Digit::One));
        assert!(box1.contains(Digit::Three, Digit::Two));
        assert!(box1.contains(Digit::Three, Digit::Three));

        let box2 = CellSet::new_box(Digit::Two);
        assert_eq!(box2.get_data(), BOX1_DATA << 3);
        assert_eq!(box2.size(), 9);
        assert_eq!(box2.get_data().count_ones() as u8, box2.size());
        assert!(box2.contains(Digit::One, Digit::Four));
        assert!(box2.contains(Digit::One, Digit::Five));
        assert!(box2.contains(Digit::One, Digit::Six));
        assert!(box2.contains(Digit::Two, Digit::Four));
        assert!(box2.contains(Digit::Two, Digit::Five));
        assert!(box2.contains(Digit::Two, Digit::Six));
        assert!(box2.contains(Digit::Three, Digit::Four));
        assert!(box2.contains(Digit::Three, Digit::Five));
        assert!(box2.contains(Digit::Three, Digit::Six));

        let box4 = CellSet::new_box(Digit::Four);
        assert_eq!(box4.get_data(), BOX1_DATA << 27);
        assert_eq!(box4.size(), 9);
        assert_eq!(box4.get_data().count_ones() as u8, box4.size());
        assert!(box4.contains(Digit::Four, Digit::One));
        assert!(box4.contains(Digit::Four, Digit::Two));
        assert!(box4.contains(Digit::Four, Digit::Three));
        assert!(box4.contains(Digit::Five, Digit::One));
        assert!(box4.contains(Digit::Five, Digit::Two));
        assert!(box4.contains(Digit::Five, Digit::Three));
        assert!(box4.contains(Digit::Six, Digit::One));
        assert!(box4.contains(Digit::Six, Digit::Two));
        assert!(box4.contains(Digit::Six, Digit::Three));

        let box9 = CellSet::new_box(Digit::Nine);
        assert_eq!(box9.get_data(), BOX1_DATA << 60);
        assert_eq!(box9.size(), 9);
        assert_eq!(box9.get_data().count_ones() as u8, box9.size());
        assert!(box9.contains(Digit::Seven, Digit::Seven));
        assert!(box9.contains(Digit::Seven, Digit::Eight));
        assert!(box9.contains(Digit::Seven, Digit::Nine));
        assert!(box9.contains(Digit::Eight, Digit::Seven));
        assert!(box9.contains(Digit::Eight, Digit::Eight));
        assert!(box9.contains(Digit::Eight, Digit::Nine));
        assert!(box9.contains(Digit::Nine, Digit::Seven));
        assert!(box9.contains(Digit::Nine, Digit::Eight));
        assert!(box9.contains(Digit::Nine, Digit::Nine));
    }

    #[test]
    fn test_insert() {
        let mut cs = CellSet::new();
        assert_eq!(cs.get_data(), 0);
        assert_eq!(cs.size(), 0);
        cs.insert(Digit::One, Digit::One);
        assert_eq!(cs.get_data(), 1);
        assert_eq!(cs.size(), 1);
        cs.insert(Digit::One, Digit::Seven);
        assert_eq!(cs.get_data(), 65);
        assert_eq!(cs.size(), 2);
        cs.insert(Digit::One, Digit::Seven);
        assert_eq!(cs.get_data(), 65);
        assert_eq!(cs.size(), 2);
        cs.insert(Digit::Seven, Digit::Two);
        assert_eq!(cs.get_data(), 65 + (1<<55));
        assert_eq!(cs.size(), 3);

        cs.insert_all(&CellSet::new_row(Digit::One));
        assert_eq!(cs.get_data(), ROW1_DATA + (1<<55));
        assert_eq!(cs.size(), 10);

        cs.insert_all(&CellSet::new_column(Digit::Two));
        assert_eq!(cs.get_data(), ROW1_DATA | (COL1_DATA<<1));
        assert_eq!(cs.size(), 17);
        assert_eq!(cs.get_data().count_ones() as u8, cs.size());
    }

    #[test]
    fn test_remove() {
        let mut cs = CellSet::new_box(Digit::One);
        assert_eq!(cs.get_data(), BOX1_DATA);
        assert_eq!(cs.size(), 9);
        cs.remove(Digit::One, Digit::Two);
        assert_eq!(cs.get_data(), BOX1_DATA - 2);
        assert_eq!(cs.size(), 8);
        cs.remove(Digit::Two, Digit::Three);
        assert_eq!(cs.get_data(), BOX1_DATA - 2 - (1<<11));
        assert_eq!(cs.size(), 7);
        cs.remove(Digit::Two, Digit::Three);
        assert_eq!(cs.get_data(), BOX1_DATA - 2 - (1<<11));
        assert_eq!(cs.size(), 7);

        cs.remove_all(&CellSet::new_row(Digit::Three));
        assert_eq!(cs.get_data(), 1 + 4 + (1<<9) + (1<<10));
        assert_eq!(cs.size(), 4);

        cs.remove_all(&CellSet::new_column(Digit::One));
        assert_eq!(cs.get_data(), 4 + (1<<10));
        assert_eq!(cs.size(), 2);

        cs.remove_all(&CellSet::new_box(Digit::Five));
        assert_eq!(cs.get_data(), 4 + (1<<10));
        assert_eq!(cs.size(), 2);
    }

    #[test]
    fn test_phistomofel() {
        let mut set1 = CellSet::new();
        set1.insert_all(&CellSet::new_row(Digit::Three));
        set1.insert_all(&CellSet::new_box(Digit::Four));
        set1.insert_all(&CellSet::new_box(Digit::Six));
        set1.insert_all(&CellSet::new_row(Digit::Seven));
        assert_eq!(set1.size(), 9*4);

        let col12 = CellSet::union(&CellSet::new_column(Digit::One), &CellSet::new_column(Digit::Two));
        let col89 = CellSet::union(&CellSet::new_column(Digit::Nine), &CellSet::new_column(Digit::Eight));
        let set2 = CellSet::union(&col12, &col89);
        assert_eq!(set2.size(), 9*4);

        let (ring, boxes) = CellSet::unique_split(&set1, &set2);
        assert_eq!(ring.size(), 16);
        assert_eq!(boxes.size(), 16);

        // central ring
        {
            assert!(ring.contains(Digit::Three, Digit::Three));
            assert!(ring.contains(Digit::Three, Digit::Four));
            assert!(ring.contains(Digit::Three, Digit::Five));
            assert!(ring.contains(Digit::Three, Digit::Six));
            assert!(ring.contains(Digit::Three, Digit::Seven));

            assert!(ring.contains(Digit::Four, Digit::Three));
            assert!(ring.contains(Digit::Four, Digit::Seven));

            assert!(ring.contains(Digit::Five, Digit::Three));
            assert!(ring.contains(Digit::Five, Digit::Seven));

            assert!(ring.contains(Digit::Six, Digit::Three));
            assert!(ring.contains(Digit::Six, Digit::Seven));

            assert!(ring.contains(Digit::Seven, Digit::Three));
            assert!(ring.contains(Digit::Seven, Digit::Four));
            assert!(ring.contains(Digit::Seven, Digit::Five));
            assert!(ring.contains(Digit::Seven, Digit::Six));
            assert!(ring.contains(Digit::Seven, Digit::Seven));
        }

        // outer 2x2s
        {
            assert!(boxes.contains(Digit::One, Digit::One));
            assert!(boxes.contains(Digit::One, Digit::Two));
            assert!(boxes.contains(Digit::Two, Digit::One));
            assert!(boxes.contains(Digit::Two, Digit::Two));

            assert!(boxes.contains(Digit::One, Digit::Eight));
            assert!(boxes.contains(Digit::One, Digit::Nine));
            assert!(boxes.contains(Digit::Two, Digit::Eight));
            assert!(boxes.contains(Digit::Two, Digit::Nine));

            assert!(boxes.contains(Digit::Eight, Digit::One));
            assert!(boxes.contains(Digit::Eight, Digit::Two));
            assert!(boxes.contains(Digit::Nine, Digit::One));
            assert!(boxes.contains(Digit::Nine, Digit::Two));

            assert!(boxes.contains(Digit::Eight, Digit::Eight));
            assert!(boxes.contains(Digit::Eight, Digit::Nine));
            assert!(boxes.contains(Digit::Nine, Digit::Eight));
            assert!(boxes.contains(Digit::Nine, Digit::Nine));
        }

    }
}
