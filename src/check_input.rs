use crate::builtins::echo::echo;
use crate::builtins::exit::exit;

use crate::utils::constants::ERROR;
use crate::utils::constants::NOT_FOUND;

fn launch_bin(words_array: &Vec<&str>) -> i32 {
    return ERROR;
}

fn check_builtins(words_array: &Vec<&str>) -> i32 {
    if words_array.get(0) == Some(&"exit") {
        exit(words_array);
        return 0;
    } else if words_array.get(0) == Some(&"echo") {
        echo(words_array);
        return 0;
    }
    return NOT_FOUND;
}

pub fn check_command(user_input: String) {
    let mut return_value: i32;

    let words_array: Vec<&str> = user_input.split(' ').collect();

    if words_array.is_empty() {
        println!("Please enter a command.");
        return;
    }

    return_value = check_builtins(&words_array);

    if return_value == NOT_FOUND {
        return_value = launch_bin(&words_array);
    }
    if return_value == ERROR {
        print!("{}: command not found\n", user_input.trim());
    }
}
