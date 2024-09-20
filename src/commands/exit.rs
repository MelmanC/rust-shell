#[allow(unused_imports)]
use std::io::{self, Write};

pub fn exit(first_word: Vec<&str>) {
    if let Some(num_str) = first_word.get(1) {
        if let Ok(value) = num_str.parse::<i32>() {
            std::process::exit(value);
        }
    }
}
