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
	suit: u8
}

fn main() {
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
	let mut opponents_hand: Vec<i32>;
	let mut deck: Vec<Card> = Vec::with_capacity(53);

	for i in 0..4 {
		for j in 1..14 {
			deck.push(Card {rank: j as u8, suit: i as u8});
		}
	}
    
    loop {
        render();
        play_turn(&deck);
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
}

fn create_card(rank: i32, suit: i32) -> String {
	let base_card = String::from("╭─────────╮\n│      RS │\n│         │\n│         │\n│         │\n│         │\n│         │\n│ RS      │\n╰─────────╯");
	let edge_card = String::from("╭───\n│   \n│   \n│   \n│   \n│   \n│   \n│   \n╰───");
	let suit_pattern_2 = String::from(" n       \n         \n      s  \n         \n  s      \n         \n       n ");

	println!("{}", base_card);

	String::new()
}

fn play_turn(deck: &Vec<Card>) {
	let mut guess = String::new();
	 std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");
}

fn render() {
	println!("\e[8;25;80t");
	print!("{esc}c", esc = 27 as char); // Clear screen
	let terminal_size = console::Term::size(&console::Term::stdout());
	
    move_cursor_absolute(0, 0);
	println!(" ______     ______        ______   __     ______     __  __");
	println!(r"╱╲  ___╲   ╱╲  __ ╲      ╱╲  ___╲ ╱╲ ╲   ╱\  ___╲   ╱╲ ╲_╲ ╲");
	println!(r"╲ ╲ ╲__ ╲  ╲ ╲ \╱\ ╲     ╲ ╲  __╲ ╲ ╲ ╲  ╲ \___  ╲  ╲ ╲  __ ╲");
	println!(r" ╲ ╲_____╲  ╲ ╲_____╲     ╲ ╲_\_╱  ╲ ╲_╲  ╲╱\_____╲  ╲ ╲_╲╱╲_╲");
	println!(r"  ╲╱_____╱   ╲╱_____╱      ╲╱_╱     ╲╱_╱   ╲╱_____╱   ╲╱_╱╲╱_╱");
	println!("{char:*>width$}", char = "*", width=terminal_size.1 as usize);

	move_cursor(1, Direction::Down);

	println!("╭───╭───╭─────────╮");
    println!("│✾ ₪│✾ ₪│✾ ₪₪₪₪₪ ✾│");
    println!("│  ▚│  ▚│  ▚▚▚▚▚  │");
    println!("│  ▚│  ▚│  ▚▚▚▚▚  │");
    println!("│  ▚│  ▚│  ▚▚▚▚▚  │");
    println!("│  ▚│  ▚│  ▚▚▚▚▚  │");
    println!("│  ▚│  ▚│  ▚▚▚▚▚  │");
    println!("│✾ ₪│✾ ₪│✾ ₪₪₪₪₪ ✾│");
    println!("╰───╰───╰─────────╯");
	

    move_cursor(3, Direction::Down);
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
	println!("╭──────────╮");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│✾╭────────┴╮");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│ │✾ ₪₪₪₪₪ ✾│");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│ │  ▚▚▚▚▚  │");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│ │  ▚▚▚▚▚  │");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│ │  ▚▚▚▚▚  │");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│ │  ▚▚▚▚▚  │");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("│✾│  ▚▚▚▚▚  │");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("╰─│✾ ₪₪₪₪₪ ✾│");
	move_cursor((terminal_size.1 as i32/2)-7, Direction::Right);
    println!("  ╰─────────╯");

	move_cursor(3, Direction::Down);
	println!("{char: >width$}", char = " 1:  2:  3:        ", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "╭───╭───╭─────────╮", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│ A │ 10│ 3       │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│ ♡ │ ♠ │         │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│   │   │      ♠  │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│   │   │    ♠    │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│   │   │  ♠      │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│   │   │         │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "│   │   │       3 │", width=terminal_size.1 as usize);
	println!("{char: >width$}", char = "╰───╰───╰─────────╯", width=terminal_size.1 as usize);
}



// CURSOR STUFF
fn move_cursor(pos: i32, dir: Direction){
    print!("\x1b[{pos}{}", dir as u8 as char);
}

fn move_cursor_absolute(posX: i32, posY: i32){
    print!("\x1b[{posY};{posX}f");
}

fn change_foreground(color: Color) {
    print!("\x1b[{}m", color as i32);
}

fn change_background(color: Color) {
    print!("\x1b[{}m", color as i32 + 10);
}

enum Direction {
    Up = 65,
    Down,
    Right,
    Left
}

enum Color {
    Default = 39,
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray = 90,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}