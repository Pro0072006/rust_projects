use std::io;
pub fn read_user() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("???????????");
    buffer.trim().to_string()
}
