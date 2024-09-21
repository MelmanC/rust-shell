#[allow(unused_imports)]
use std::io::{self, Write};

use std::env;
use std::path::Path;

fn error_handling(words_array: &Vec<&str>, args: Option<&&str>) -> i32 {
    if words_array.len() > 2 {
        let error_message: &str = "cd: too many arguments\n";
        io::stderr().write_all(error_message.as_bytes()).unwrap();
        return -1;
    } else if args != None && Path::new(args.unwrap()).exists() == false {
        let error_message: &str = "cd: no such file or directory\n";
        io::stderr().write_all(error_message.as_bytes()).unwrap();
        return -1;
    } else if args != None && Path::new(args.unwrap()).is_dir() == false {
        let error_message: &str = "cd: Not a directory\n";
        io::stderr().write_all(error_message.as_bytes()).unwrap();
        return -1;
    }
    return 0;
}

pub fn change_directory(words_array: &Vec<&str>) {
    let args: Option<&&str> = words_array.get(1);
    let home: String = env::var("HOME").unwrap();

    if error_handling(words_array, args) == -1 {
        return;
    }

    if args == None {
        let root: &Path = Path::new(&home);
        assert!(env::set_current_dir(&root).is_ok());
    } else {
        let path: &Path = Path::new(args.unwrap());
        assert!(env::set_current_dir(&path).is_ok());
    }
}
