use std::io::Write;

pub fn clear() {
    print!("{}[2J", 27 as char); // Clear the screen
    print!("{}[H", 27 as char); // Move cursor to the top-left corner
    std::io::stdout().flush().unwrap(); // Ensure the output is flushed
}