#[allow(unused_imports)]
use std::io::{self, Write};

use std::env;
use std::path::Path;
use std::path::PathBuf;

pub fn change_directory(words_array: &Vec<&str>) {
    let args: Option<&&str> = words_array.get(1);
    let user: String = env::var("USER").unwrap();

    if args == None {
        let root: PathBuf = Path::new("/home").join(&user);
        assert!(env::set_current_dir(&root).is_ok());
    } else {
        let path: &Path = Path::new(args.unwrap());
        assert!(env::set_current_dir(&path).is_ok());
    }
}
