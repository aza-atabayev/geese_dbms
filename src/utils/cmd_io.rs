use std::io::{self};

pub fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // read a line from stdin
    let input = input.trim(); // remove trailing newline
    input.to_string()
}