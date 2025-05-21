use crate::card::*;
use strum::IntoEnumIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn play() {
	let mut player_has_won: bool = false;
	let mut deck = create_deck(true);
	let mut player_1_hand: Vec<Card> = Vec::new();
	let mut player_2_hand: Vec<Card> = Vec::new();
	
	for &card in &deck{
		card.render();
	}
	
	for i in 0..7 {
		player_1_hand.push(draw(&mut deck));
		player_2_hand.push(draw(&mut deck));
	}
	
	// while !player_has_won {
	// 	player_1_turn();
	// 	player_2_turn();
	// }
}

fn player_1_turn() {
	
}

fn player_2_turn() {
	
}

fn create_deck(is_shuffled: bool) -> Vec<Card> {
	let mut deck = Vec::<Card>::with_capacity(52);
	
	for suit in Suit::iter() {
		for rank in Rank::iter() {
			let card = Card::new(suit, rank);
			deck.push(card);
		}
	}
	
	if is_shuffled {
		shuffle(&mut deck);
	}
	
	create_card(Suit::Club, Rank::Ace, Type::Middle, Facing::FaceUp);
	deck
}

fn shuffle(card_set: &mut Vec<Card>){
	let mut generator = thread_rng();
	card_set.shuffle(&mut generator);
}

fn draw(deck: &mut Vec<Card>) -> Card {
	let card = deck.pop().unwrap();
	card
}