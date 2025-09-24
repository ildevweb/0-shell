pub fn cat(args: &[&str]) {
    for &file in args {
        match std::fs::read_to_string(file) {
            Ok(contents) => print!("{}", contents),
            Err(e) => eprintln!("Error reading file '{}': {}", file, e),
        }
    }
}

