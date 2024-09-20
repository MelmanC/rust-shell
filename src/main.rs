mod builtins;
mod check_input;
mod utils;

use check_input::check_command;

#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut user_input: String = String::new();

        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.trim().to_string();

        check_command(user_input);
    }
}
