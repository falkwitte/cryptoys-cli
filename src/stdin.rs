use std::io::{self, BufRead};

/// read the stdin to a string
pub fn stdin_read_to_string() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap() + "\n";

        buffer.push_str(line.as_str());
    }
    buffer
}
