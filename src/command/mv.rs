pub fn mv(args: &mut Vec<&str>) {
    if args.len() < 2 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let source = args.remove(0);
    let destination = args.remove(0);

    match std::fs::rename(source, destination) {
        Ok(_) => println!("Moved '{}' to '{}'", source, destination),
        Err(e) => eprintln!("Error moving file: {}", e),
    }
}