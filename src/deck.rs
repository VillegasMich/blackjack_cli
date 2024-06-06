use crate::card::Card;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        let suits = ["Spades ♠", "Hearts ♥", "Diamonds ♦", "Clubs ♣"];
        let values = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        for suit in suits.iter() {
            for value in values.iter() {
                cards.push(Card {
                    suit: suit.to_string(),
                    value: value.to_string(),
                });
            }
        }
        Deck { cards }
    }
}
