pub fn rm(args: &mut Vec<&str>) {
    if args.is_empty() {
        eprintln!("Error: No files or directories specified for removal.");
        return;
    }

    for &arg in args.iter() {
        match std::fs::remove_file(arg) {
            Ok(_) => println!("Removed file: {}", arg),
            Err(e) => eprintln!("Failed to remove '{}': {}", arg, e),
        }
    }
}