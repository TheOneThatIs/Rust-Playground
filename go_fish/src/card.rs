#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Club,
    Diamond,
    Spade,
    Heart,
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive (Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}