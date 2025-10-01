
pub fn rm(args: &mut Vec<&str>) {
    if args.is_empty() {
        eprintln!("Error: No files or directories specified for removal.");
        return;
    }

    if !args.contains(&"-r") {
        for &arg in args.iter() {
            match std::fs::remove_file(arg) {
                Ok(_) => println!("Removed file: {}", arg),
                Err(e) => eprintln!("Failed to remove '{}': {}", arg, e),
            }
        }
    } else {
        let mut v = args.clone();
        if let Some(pos) = v.iter().position(|x| *x == "-r") {
            v.remove(pos);
        }

        for &arg in v.iter() {
            match std::fs::remove_dir_all(arg) {
                Ok(_) => println!("Removed directory: {}", arg),
                Err(e) => eprintln!("Failed to remove '{}': {}", arg, e),
            }
        }
    }
    
}
