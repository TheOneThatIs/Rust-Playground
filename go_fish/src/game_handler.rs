use crate::card::*;
use strum::IntoEnumIterator;

pub fn play() {
	let mut player_has_won: bool = false;
	let deck = create_deck();
	
	for i in 0..52 {
		println!("{:?}", deck[i]);
	}
	
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
	deck
}