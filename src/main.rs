fn main() {
    // Clear screen
    print!("{}[2J", 27 as char);
    // Set console size to 80x25 colour (text)
    print!("{}[=3h", 27 as char);
    // Move cursor to top left location
    print!("{}[0;0H", 27 as char);
    println!("Welcome to Conway's Game of Life!");
}
