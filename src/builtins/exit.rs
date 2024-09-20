#[allow(unused_imports)]
use std::io::{self, Write};

pub fn exit(words_array: Vec<&str>) {
    if let Some(num_str) = words_array.get(1) {
        if let Ok(value) = num_str.parse::<i32>() {
            std::process::exit(value);
        }
    }
}
