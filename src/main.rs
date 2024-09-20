#[allow(unused_imports)]
use std::io::{self, Write};

fn check_command(user_input: String) {
    let first_word: Vec<&str> = user_input.split(' ').collect();

    if first_word.get(0) == Some(&"exit") {
        if let Some(num_str) = first_word.get(1) {
            if let Ok(value) = num_str.parse::<i32>() {
                std::process::exit(value);
            }
        }
    }

    print!("{}: command not found\n", user_input.trim());
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
