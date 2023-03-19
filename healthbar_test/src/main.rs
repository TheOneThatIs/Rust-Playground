
use std::io;

fn main() {
    clear_screen();
    let mut hp = 10;
    let maxHP = 10;

    while hp > 0 {
        clear_screen();
        drawHealth(&hp, &maxHP);
        attack(&mut hp);
    }

    move_cursor_absolute(0, 1);
    drawHealth(&hp, &maxHP);
    move_cursor_absolute(0, 5);
    println!("Goblin died!               ");
    println!("                ");

    let mut msg = String::new();
    io::stdin().read_line(&mut msg).expect("Failed to read the line!");

    clear_screen();
}

fn drawHealth(hp: &i32, maxHP: &i32){
    println!("{}", console::style("Goblin").cyan().italic());

    print!("{}", console::style("HP: ").green());
    
    for num in 0..*hp { // change it to get range
        print!("▓");
    }
    for num in 0..*maxHP-*hp { // change it to get range
        print!("░");
    }
    println!(" {hp}/{maxHP}\n\n");
}

fn attack(hp: &mut i32) {
    println!("Attack for how much?");

    let mut atk = String::new();

    io::stdin().read_line(&mut atk).expect("Failed to read the line!");

    let atk: i32 = match atk.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    *hp = if (*hp - atk) >= 0 { *hp - atk} else {0};

    move_cursor_absolute(0, 2);
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}
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
    Left,
    Right
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