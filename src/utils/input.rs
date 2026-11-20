use std::io::{self, IsTerminal};

pub fn read_stdin() -> io::Result<String> {
    if io::stdin().is_terminal() {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim_end_matches(&['\r', '\n']).to_string())
    } else {
        io::read_to_string(io::stdin())
    }
}