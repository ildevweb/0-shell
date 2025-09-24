use std::env;

pub fn pwd(_args: &[&str]) {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}