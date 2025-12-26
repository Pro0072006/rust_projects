use std::io::{self, Write};
pub fn clear() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().expect("WTF_clear");
}
