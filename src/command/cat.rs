pub fn cat(args: &[&str]) {
    if !args.contains(&">") {
        for &file in args {
            match std::fs::read_to_string(file) {
                Ok(contents) => println!("{}", contents),
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
    } else {
        let mut parts = args.split(|&arg| arg == ">");
        let files: Vec<&str> = parts.next().unwrap_or(&[]).to_vec();
        let output_file = parts.next().and_then(|p| p.first()).map(|s| *s);

        if let Some(output) = output_file {
            let mut combined_contents = String::new();
            for &file in &files {
                match std::fs::read_to_string(file) {
                    Ok(contents) => combined_contents.push_str(&contents),
                    Err(e) => eprintln!("Error reading file '{}': {}", file, e),
                }
            }
            match std::fs::write(output, combined_contents) {
                Ok(_) => println!("Contents written to '{}'", output),
                Err(e) => eprintln!("Error writing to file '{}': {}", output, e),
            }
        } else {
            eprintln!("No output file specified after '>'");
        }
    }
    
}

