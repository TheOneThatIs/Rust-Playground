use crate::card::*;
use strum::IntoEnumIterator;

pub fn play() {
	let mut player_has_won: bool = false;
	let deck = create_deck();
	
	while !player_has_won {
		player_1_turn();
		player_2_turn();
	}
}

fn player_1_turn() {
	
}

fn player_2_turn() {
	
}

fn create_deck() -> Vec<Card> {
	let mut deck = Vec::<Card>::new();
	deck.reserve(52);
	
	for suit in Suit::iter() {
		for rank in Rank::iter() {
			let card = Card::new(suit, rank);
			deck.push(card);
		}
	}
	
	// for suit in &[Suit::Club, Suit::Diamond, Suit::Spade, Suit::Heart] {
	// 	for rank in &[
	// 		Rank::Ace,
	// 		Rank::Two,
	// 		Rank::Three,
	// 		Rank::Four,
	// 		Rank::Five,
	// 		Rank::Six,
	// 		Rank::Seven,
	// 		Rank::Eight,
	// 		Rank::Nine,
	// 		Rank::Ten,
	// 		Rank::Jack,
	// 		Rank::Queen,
	// 		Rank::King,
	// 	] {
	// 		let card = Card::new(*suit, *rank);
	// 		deck.push(card);
	// 	}
	// }
	
	deck
}