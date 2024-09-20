#[allow(unused_imports)]
use std::io::{self, Write};

pub fn echo(words_array: &Vec<&str>) {
    let mut output: String = String::new();

    for words in words_array.iter().skip(1) {
        output.push_str(words);
        output.push(' ');
    }
    output.pop();

    print!("{}\n", output);
}
