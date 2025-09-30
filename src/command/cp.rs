use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::env;


pub fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let source = args[0];
    let destination = args[1];

    if is_file(source) && is_file(destination) {
        match fs::copy(source, destination) {
            Ok(_) => println!("Copied '{}' to '{}'", source, destination),
            Err(e) => eprintln!("Error copying file: {}", e),
        }
    } else if is_file(source) && (is_dir(destination) || is_current_dir(destination)) {
        let source_path = Path::new(source);
        let dest_dir_path = Path::new(destination);

        let file_name = match source_path.file_name() {
            Some(name) => name,
            None => {
                eprintln!("Invalid source file path");
                return;
            }
        };


        let dest_path: PathBuf = if dest_dir_path.is_dir() {
            dest_dir_path.join(file_name)
        } else {
            dest_dir_path.to_path_buf()
        };

        fs::copy(source_path, dest_path);
        println!("Copied '{}' to '{}'", source, destination);

    } else if !is_file(source) {
        eprintln!("source file doesn't exist");
        return;
    } else if !is_file(destination) && !is_dir(destination) && !is_current_dir(destination) {
        eprintln!("destination name doesn't exist");
        return;
    }

    /*match fs::copy(source, destination) {
        Ok(_) => println!("Copied '{}' to '{}'", source, destination),
        Err(e) => eprintln!("Error copying file: {}", e),
    }*/
}


fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}


fn is_current_dir(dest: &str) -> bool {
    let dest_path = Path::new(dest);

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return false,
    };


    let binding = current_dir.display().to_string();
    let current_dir_vec = binding.split("/").collect::<Vec<_>>();
    let current_dir_name = current_dir_vec[current_dir_vec.len()-1];


    current_dir_name == dest_path.display().to_string()
}