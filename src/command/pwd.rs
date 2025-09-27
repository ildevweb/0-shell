use std::env;
use std::fs;

pub fn pwd(args: &[&str]) {
    let mut logical = true;

    for arg in args {
        match *arg {
            "-L" => logical = true,
            "-P" => logical = false,
            _ => {}
        }
    }

    if logical {
        match env::var("PWD") {
            Ok(pwd) => println!("{}", pwd),
            Err(_) => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("Error getting current directory: {}", e),
            },
        }
    } else {
        match env::current_dir() {
            Ok(path) => match fs::canonicalize(&path) {
                Ok(real_path) => println!("{}", real_path.display()),
                Err(e) => eprintln!("Error resolving physical path: {}", e),
            },
            Err(e) => eprintln!("Error getting current directory: {}", e),
        }
    }
}