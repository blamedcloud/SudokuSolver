

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    pub fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Digit> {
        [Digit::One, Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine].iter().copied()
    }

    pub fn get_box_num(row: Digit, column: Digit) -> Self {
        match (row.into(), column.into()) {
            (Entropy::Low, Entropy::Low) => Digit::One,
            (Entropy::Low, Entropy::Middle) => Digit::Two,
            (Entropy::Low, Entropy::High) => Digit::Three,
            (Entropy::Middle, Entropy::Low) => Digit::Four,
            (Entropy::Middle, Entropy::Middle) => Digit::Five,
            (Entropy::Middle, Entropy::High) => Digit::Six,
            (Entropy::High, Entropy::Low) => Digit::Seven,
            (Entropy::High, Entropy::Middle) => Digit::Eight,
            (Entropy::High, Entropy::High) => Digit::Nine,
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Entropy {
    Low,
    Middle,
    High,
}

impl From<Digit> for Entropy {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::One | Digit::Two | Digit::Three => Self::Low,
            Digit::Four | Digit::Five | Digit::Six => Self::Middle,
            Digit::Seven | Digit::Eight | Digit::Nine => Self::High,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mod3 {
    Zero,
    One,
    Two,
}

impl From<Digit> for Mod3 {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::One | Digit::Four | Digit::Seven => Self::One,
            Digit::Two | Digit::Five | Digit::Eight => Self::Two,
            Digit::Three | Digit::Six | Digit::Nine => Self::Zero,
        }
    }
}
