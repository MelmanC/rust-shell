mod builtins;

use builtins::echo::echo;
use builtins::exit::exit;

#[allow(unused_imports)]
use std::io::{self, Write};

fn check_command(user_input: String) {
    let words_array: Vec<&str> = user_input.split(' ').collect();

    if words_array.get(0) == Some(&"exit") {
        exit(words_array);
    } else if words_array.get(0) == Some(&"echo") {
        echo(words_array);
    } else {
        print!("{}: command not found\n", user_input.trim());
    }
}

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
