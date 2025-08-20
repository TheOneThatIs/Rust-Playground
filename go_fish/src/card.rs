use strum_macros::EnumIter;
use std::collections::HashMap;
use std::ptr::copy_nonoverlapping;

pub enum Type {
    Middle,
    LeftEdge,
    RightEdge,
}

pub enum Facing {
    FaceUp,
    FaceDown,
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Suit {
    Club,
    Diamond,
    Spade,
    Heart,
}

#[derive (Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
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

#[derive (Debug, Copy, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank
        }
    }
    pub fn render(self) {
        println!("{}", create_card(self.suit, self.rank, Type::Middle, Facing::FaceUp));
    }

}

pub fn create_card(suit: Suit, rank: Rank, card_type: Type, facing: Facing) -> String {
    let suit_map: HashMap<Suit, &str> = [
        (Suit::Club, "♧"),
        (Suit::Diamond, "♦"),
        (Suit::Spade, "♤"),
        (Suit::Heart, "♥"),
    ].iter().cloned().collect();

    let art_map: HashMap<Rank, &str> = [
        (Rank::Ace, "╭─────────╮\n│ As      │\n│    █    │\n│   ███   │\n│  ██ ██  │\n│ ███████ │\n│ ██   ██ │\n│      As │\n╰─────────╯"),
        (Rank::Two, "╭─────────╮\n│ 2       │\n│         │\n│      s  │\n│         │\n│  s      │\n│         │\n│       2 │\n╰─────────╯"),
        (Rank::Three, "╭─────────╮\n│ 3       │\n│         │\n│      s  │\n│    s    │\n│  s      │\n│         │\n│       3 │\n╰─────────╯"),
        (Rank::Four, "╭─────────╮\n│ 4       │\n│         │\n│  s   s  │\n│         │\n│  s   s  │\n│         │\n│       4 │\n╰─────────╯"),
        (Rank::Five, "╭─────────╮\n│ 5       │\n│         │\n│  s   s  │\n│    s    │\n│  s   s  │\n│         │\n│       5 │\n╰─────────╯"),
        (Rank::Six, "╭─────────╮\n│ 6       │\n│         │\n│  s   s  │\n│  s   s  │\n│  s   s  │\n│         │\n│       6 │\n╰─────────╯"),
        (Rank::Seven, "╭─────────╮\n│ 7       │\n│         │\n│  s   s  │\n│ s  s  s │\n│  s   s  │\n│         │\n│       7 │\n╰─────────╯"),
        (Rank::Eight, "╭─────────╮\n│ 8       │\n│    s    │\n│  s   s  │\n│ s     s │\n│  s   s  │\n│    s    │\n│       8 │\n╰─────────╯"),
        (Rank::Nine, "╭─────────╮\n│ 9       │\n│    s    │\n│  s   s  │\n│ s  s  s │\n│  s   s  │\n│    s    │\n│       9 │\n╰─────────╯"),
        (Rank::Ten, "╭─────────╮\n│ 10      │\n│    s    │\n│  s   s  │\n│ s s s s │\n│  s   s  │\n│    s    │\n│      10 │\n╰─────────╯"),
        (Rank::Jack, "╭─────────╮\n│      Js │\n│      ██ │\n│      ██ │\n│      ██ │\n│ ██   ██ │\n│  █████  │\n│ Js      │\n╰─────────╯"),
        (Rank::Queen, "╭─────────╮\n│      Qs │\n│    🕆   │\n│ │╲╱ ╲╱│ │\n│ │⁕ ※ ⁕│ │\n│ ╰⏔⏔⏔⏔⏔╯ │\n│         │\n│ Qs      │\n╰─────────╯"),
        (Rank::King, "╭─────────╮\n│      Ks │\n│  _.🕆._ │\n│(^╲╱^╲╱^)│\n│ ╲⁕*⁛*⁕╱ │\n│ ╰⏔⏔°⏔⏔╯ │\n│         │\n│ Ks      │\n╰─────────╯"),
    ].iter().cloned().collect();

    let mut card_string = art_map.get(&rank).unwrap().to_string();
    card_string = card_string.replace("s", suit_map.get(&suit).unwrap());
    card_string
}

//println!("╭─────────╮");
//println!("│ 3       │");
//println!("│         │");
//println!("│      ♠  │");
//println!("│    ♠    │");
//println!("│  ♠      │");
//println!("│         │");
//println!("│       3 │");
//println!("╰─────────╯");
//
//
//println!("╭─────────╮");//⌢⌄
//println!("│ A♧      │");
//println!("│    █    │");
//println!("│   ███   │");
//println!("│  ██ ██  │");
//println!("│ ███████ │");
//println!("│ ██   ██ │");
//println!("│      A♧ │");
//println!("╰─────────╯");
//println!("╭─────────╮");
//println!("│      J♧ │");
//println!("│      ██ │");
//println!("│      ██ │");
//println!("│      ██ │");
//println!("│ ██   ██ │");
//println!("│  █████  │");
//println!("│ J♧      │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│      Q♧ │");
//println!("│    🕆    │");
//println!("│ │╲╱ ╲╱│ │");
//println!("│ │⁕ ※ ⁕│ │");
//println!("│ ╰⏔⏔⏔⏔⏔╯ │");
//println!("│         │");
//println!("│ Q♧      │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│      K♧ │");
//println!("│  _.🕆._  │");
//println!("│(^╲╱^╲╱^)│");
//println!("│ ╲⁕*⁛*⁕╱ │");
//println!("│ ╰⏔⏔°⏔⏔╯ │");
//println!("│         │");
//println!("│ K♧      │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 2       │");
//println!("│         │");
//println!("│      ♠  │");
//println!("│         │");
//println!("│  ♠      │");
//println!("│         │");
//println!("│       2 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 3       │");
//println!("│         │");
//println!("│      ♠  │");
//println!("│    ♠    │");
//println!("│  ♠      │");
//println!("│         │");
//println!("│       3 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 4       │");
//println!("│         │");
//println!("│  ♠   ♠  │");
//println!("│         │");
//println!("│  ♠   ♠  │");
//println!("│         │");
//println!("│       4 │");
//println!("╰─────────╯");

//println!("╭─────────╮");
//println!("│ 5       │");
//println!("│         │");
//println!("│  ♠   ♠  │");
//println!("│    ♠    │");
//println!("│  ♠   ♠  │");
//println!("│         │");
//println!("│       5 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 6       │");
//println!("│         │");
//println!("│  ♠   ♠  │");
//println!("│  ♠   ♠  │");
//println!("│  ♠   ♠  │");
//println!("│         │");
//println!("│       6 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 7       │");
//println!("│         │");
//println!("│  ♠   ♠  │");
//println!("│ ♠  ♠  ♠ │");
//println!("│  ♠   ♠  │");
//println!("│         │");
//println!("│       7 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 8       │");
//println!("│    ♠    │");
//println!("│  ♠   ♠  │");
//println!("│ ♠     ♠ │");
//println!("│  ♠   ♠  │");
//println!("│    ♠    │");
//println!("│       8 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 9       │");
//println!("│    ♠    │");
//println!("│  ♠   ♠  │");
//println!("│ ♠  ♠  ♠ │");
//println!("│  ♠   ♠  │");
//println!("│    ♠    │");
//println!("│       9 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ 10      │");
//println!("│    ♠    │");
//println!("│  ♠   ♠  │");
//println!("│ ♠ ♠ ♠ ♠ │");
//println!("│  ♠   ♠  │");
//println!("│    ♠    │");
//println!("│      10 │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│ ♧       │");
//println!("│  _  __  │");
//println!("│ │ │╱ ╱  │");
//println!("│ │ ' ╱   │");
//println!("│ │ . ╲   │");
//println!(r"│ │_│╲_╲  │");
//println!("│       ♧ │");
//println!("╰─────────╯");
//println!("╭─────────╮");
//println!("│ ♧ ___   │");
//println!("│  ╱ _ ╲  │");
//println!(r"│ │ | | │ │");
//println!("│ │ |_| │ │");
//println!("│  ╲__╲_╲ │");
//println!(r"│         │");
//println!("│       ♧ │");
//println!("╰─────────╯");
//println!("╭─────────╮");
//println!("│ ♧       │");
//println!("│  ┌─────┐│");
//println!("│  └─┐ ┌─┘│");
//println!("│┌─┐ │ │  │");
//println!("││ └─┘ │  │");
//println!("│ ╲___╱   │");
//println!("│       ♧ │");
//println!("╰─────────╯");
//println!("╭─────────╮");
//println!("│      Q♧ │");
//println!("│  ██████ │");
//println!("│ ██    ██│");
//println!("│ ██    ██│");
//println!("│ ██ ▄▄ ██│");
//println!("│  ██████ │");
//println!("│ Q♧  ▀▀  │");
//println!("╰─────────╯");
//
//println!("╭─────────╮");
//println!("│      K♧ │");
//println!("│ ██   ██ │");
//println!("│ ██  ██  │");
//println!("│ █████   │");
//println!("│ ██  ██  │");
//println!("│ ██   ██ │");
//println!("│ K♧      │");
//println!("╰─────────╯");