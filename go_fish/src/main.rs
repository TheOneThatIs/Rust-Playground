
enum Rank {
	Ace=1, Two=2, Three=3, Four=4, Five=5, Six=6, Seven=7, Eight=8, Nine=9, Ten=10, Jack=11, Queen=12, King=13
}

enum Suit {
	Club=0, Diamond=1, Heart=2, Spade=3, 
}

enum Location {
	PlayerHand=0, OpponentHand=1, Deck=2, PairPile=3
}

struct Card {
	rank: u8,
	suit: u8,
	location: Location
}

fn main() {
    //println!(" ______     ______        ______   __     ______     __  __");
    //println!(r"/\  ___\   /\  __ \      /\  ___\ /\ \   /\  ___\   /\ \_\ \");
    //println!(r"\ \ \__ \  \ \ \/\ \     \ \  __\ \ \ \  \ \___  \  \ \  __ \");
    //println!(r" \ \_____\  \ \_____\     \ \_\    \ \_\  \/\_____\  \ \_\ \_\");
    //println!(r"  \/_____/   \/_____/      \/_/     \/_/   \/_____/   \/_/\/_");
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
    //// DECK
    //println!("╭──────────╮");
    //println!("│ ╭────────┴╮");
    //println!("│ │✾ ₪₪₪₪₪ ✾│");
    //println!("│ │  ▚▚▚▚▚  │");
    //println!("│ │  ▚▚▚▚▚  │");
    //println!("│ │  ▚▚▚▚▚  │");
    //println!("│ │  ▚▚▚▚▚  │");
    //println!("│ │  ▚▚▚▚▚  │");
    //println!("╰─│✾ ₪₪₪₪₪ ✾│");
    //println!("  ╰─────────╯");
    //
    //// HAND
    //println!(" 1:  2:  3:");
    //println!("╭───╭───╭─────────╮");
    //println!("│ A │ 10│ 3       │");
    //println!("│ ♡ │ ♠ │         │");
    //println!("│   │   │      ♠  │");
    //println!("│   │   │    ♠    │");
    //println!("│   │   │  ♠      │");
    //println!("│   │   │         │");
    //println!("│   │   │       3 │");
    //println!("╰───╰───╰─────────╯");

    //println!("╭─────────╮");//⌢⌄ 
    //println!("│ A♧      │");
    //println!("│    █    │");
    //println!("│   ███   │");
    //println!("│  ██ ██  │");
    //println!("│ ███████ │");
    //println!("│ ██   ██ │");
    //println!("│      A♧ │");
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
    //
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
	let mut hand: Vec<i32>;
	let mut deck = Vec::with_capacity(53);

	for i in 0..4 {
		for j in 1..14 {
			deck.push(Card {rank: j as u8, suit: i as u8, location: Location::Deck});
		}
	}

	for i in 0..52 {
		println!("Rank: {}, Suit:{}", deck[i].rank, deck[i].suit);
	}

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
	println!("╭─────────╮");
    println!("│      Q♧ │");
    println!("│    🕆    │");
    println!("│ │╲╱ ╲╱│ │");
    println!("│ │⁕ ※ ⁕│ │");
    println!("│ ╰⏔⏔⏔⏔⏔╯ │");
    println!("│         │");
    println!("│ Q♧      │");
    println!("╰─────────╯");
    //
    println!("╭─────────╮");
    println!("│      K♧ │");
    println!("│  _.🕆._  │");
    println!("│(^╲╱^╲╱^)│");
    println!("│ ╲⁕*⁛*⁕╱ │");
    println!("│ ╰⏔⏔°⏔⏔╯ │");
    println!("│         │");
    println!("│ K♧      │");
    println!("╰─────────╯");

	dont_close_program_until_input();
}

fn create_card(rank: i32, suit: i32) -> String {
	let base_card = String::from("╭─────────╮\n│      RS │\n│         │\n│         │\n│         │\n│         │\n│         │\n│ RS      │\n╰─────────╯");
	let edge_card = String::from("╭───\n│   \n│   \n│   \n│   \n│   \n│   \n│   \n╰───");
	let suit_pattern_2 = String::from(" n       \n         \n      s  \n         \n  s      \n         \n       n ");

	println!("{}", base_card);

	String::new()
}

fn dont_close_program_until_input(){
	let mut input = String::new();
	 std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line!");
}