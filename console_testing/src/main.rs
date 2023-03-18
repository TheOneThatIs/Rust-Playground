
fn main() {
    clear_screen();

    draw_board();
    
    // println!(" ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ ");
    // println!(" ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ");
    // println!("");
    // println!("");
    // println!("");
    // println!("");
    // println!(" ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ ");
    // println!(" ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ ");

    //move_cursor_absolute(0, 0);

}

fn draw_board() {
    change_foreground(Color::Blue);
    change_background(Color::Black); print!("♜ ");
    change_background(Color::White); print!("♞ ");
    change_background(Color::Black); print!("♝ ");
    change_background(Color::White); print!("♛ ");
    change_background(Color::Black); print!("♚ ");
    change_background(Color::White); print!("♝ ");
    change_background(Color::Black); print!("♞ ");
    change_background(Color::White); print!("♜ \n");

    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ \n");

    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  \n");

    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  \n");

    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  \n");

    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  ");
    change_background(Color::White); print!("  ");
    change_background(Color::Black); print!("  \n");

    change_foreground(Color::Green);
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ ");
    change_background(Color::Black); print!("♟︎ ");
    change_background(Color::White); print!("♟︎ \n");

    change_background(Color::White); print!("♜ ");
    change_background(Color::Black); print!("♞ ");
    change_background(Color::White); print!("♝ ");
    change_background(Color::Black); print!("♛ ");
    change_background(Color::White); print!("♚ ");
    change_background(Color::Black); print!("♝ ");
    change_background(Color::White); print!("♞ ");
    change_background(Color::Black); print!("♜ \n");

    
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
    move_cursor_absolute(0, 0);
}
fn move_cursor(pos: i32, dir: Direction){
    print!("\x1b[{pos}{}", dir as u8 as char);
}

fn move_cursor_absolute(posX: i32, posY: i32){
    print!("\x1b[{posX};{posY}f");
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