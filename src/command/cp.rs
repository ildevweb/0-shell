// src/command/cp.rs
use std::fs;
use std::io;

pub fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let source = args[0];
    let destination = args[1];

    match fs::copy(source, destination) {
        Ok(_) => println!("Copied '{}' to '{}'", source, destination),
        Err(e) => eprintln!("Error copying file: {}", e),
    }
}