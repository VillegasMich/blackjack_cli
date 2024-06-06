use core::fmt;

pub struct Card {
    pub suit: String,
    pub value: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Suit: {}, Value: {}", self.suit, self.value)
    }
}
