use crate::builtins::echo::echo;
use crate::builtins::exit::exit;

use crate::utils::constants::ERROR;
use crate::utils::constants::NOT_FOUND;

use std::env;
use std::process::Command;

fn launch_bin(words_array: &Vec<&str>) -> i32 {
    let cmd = words_array.first().unwrap();
    let all_path: String = env::var("PATH").unwrap();
    let path = all_path
        .split(':')
        .find(|path: &&str| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok());

    if path.is_some() {
        let command_output: std::process::Output =
            Command::new(format!("{}/{}", path.unwrap(), cmd))
                .args(&words_array[1..])
                .output()
                .expect("Failed to execute command");

        let output = String::from_utf8_lossy(&command_output.stdout);
        let trimmed_output = output.trim().replace("\n", " ");
        println!("{}", trimmed_output);

        return 0;
    }

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

    return_value = check_builtins(&words_array);

    if return_value == NOT_FOUND {
        return_value = launch_bin(&words_array);
    }
    if return_value == ERROR {
        print!("{}: command not found\n", user_input.trim());
    }
}
