
enum Rank {
	Ace, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King
}

enum Suit {
	Club, Diamond, Heart, Spade, 
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

	println!("{}", create_card(Rank::Jack, Suit::Diamond));

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
    //println!("│      K♧ │");
    //println!("│         │");
    //println!("│    🕆    │");
    //println!("│ │╲╱ ╲╱│ │");
    //println!("│ │⁕ ※ ⁕│ │");
    //println!("│ ╰─────╯ │");
    //println!("│ K♧      │");
    //println!("╰─────────╯");
    //
    //println!("╭─────────╮");
    //println!("│      Q♧ │");
    //println!("│  _.🕆._  │");
    //println!("│(^╲╱^╲╱^)│");
    //println!("│ ╲⁕*⁛*⁕╱ │");
    //println!("│ ╰⏔⏔°⏔⏔╯ │");
    //println!("│         │");
    //println!("│ Q♧      │");
    //println!("╰─────────╯");

	dont_close_program_until_input();
}

fn create_card(rank: Rank, suit: Suit) -> String {
	let base_card = String::from("╭─────────╮\n│         │\n│         │\n│         │\n│         │\n│         │\n│         │\n│         │\n╰─────────╯");
	let edge_card = String::from("╭───\n│   \n│   \n│   \n│   \n│   \n│   \n│   \n╰───");
	let suit_pattern_2 = String::from(" n       \n         \n      s  \n         \n  s      \n         \n       n ");

	String::new()
}

fn dont_close_program_until_input(){
	let mut input = String::new();
	 std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line!");
}