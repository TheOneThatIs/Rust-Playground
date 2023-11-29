use strum_macros::EnumIter;

pub enum Type {
    Middle
    LeftEdge,
    RightEdge,
}

pub enum Facing {
    FaceUp,
    FaceDown,
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Suit {
    Club,
    Diamond,
    Spade,
    Heart,
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

pub fn create_card(suit: Suit, rank: Rank, type: Type, facing: Facing) {
    suit_map = [
        (Suit::Club, '♣'),
        (Suit::Diamond, '♦'),
        (Suit::Spade, '♠'),
        (Suit::Heart, '♥'),
    ].iter().cloned.collect::<HashMap<Suit, char>>();

    let rank_map = [
        (Rank::Ace, "A"),
        (Rank::Two, "2"),
        (Rank::Three, "3"),
        (Rank::Four, "4"),
        (Rank::Five, "5"),
        (Rank::Six, "6"),
        (Rank::Seven, "7"),
        (Rank::Eight, "8"),
        (Rank::Nine, "9"),
        (Rank::Ten, "10"),
        (Rank::Jack, "J"),
        (Rank::Queen, "Q"),
        (Rank::King, "K"),
    ].iter().cloned.collect::<HashMap<Rank, &str>>();
}