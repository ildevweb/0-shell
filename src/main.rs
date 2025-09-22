mod command;

/*
fn main() {
    if !atty::is(Stream::Stdout) {
        eprintln!("error: broken pipe\nstdout is NOT connected to a terminal.");
        return;
    }
    loop {
        if std::env::current_dir().is_err() {
            let backup_path = std::env::var("HOME").unwrap_or_else(|_| "".to_string());
            if let Err(e) = std::env::set_current_dir(&backup_path) {
                eprintln!("Failed to recover working directory: {}", e);
                std::env::set_current_dir("/").ok();
            }
        }
        let dir_path = env::current_dir().unwrap_or("".into());
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/".to_string());

        let prompt = if dir_path.to_str() == Some(&home_dir) {
            "~".to_string()
        } else {
            dir_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("/")
                .to_string()
        };

        print!("{}$ ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                for command_str in split_and_parsing_commands(&input) {
                    let mut current_input = command_str.to_string();
                    loop {
                        match parse_args(&current_input) {
                            Ok(args) => {
                                if !args.is_empty() {
                                    let command = &args[0];
                                    let command_args = &args[1..];
                                    // println!("{:?}", command_args);
                                    match command.as_str() {
                                        "echo" => command::echo::echo(command_args),
                                        "cd" => command::cd::cd(&command_args),
                                        "pwd" => command::pwd::pwd(&command_args),
                                        "cat" => command::cat::cat(&command_args),
                                        "mkdir" =>
                                            command::mkdir::mkdir(&mut command_args.to_vec()),
                                        "cp" => command::cp::cp(&command_args),
                                        "mv" => command::mv::mv(&command_args.to_vec()),
                                        "help" => command::help::help(),
                                        "clear" => command::clear::clear(),
                                        "rm" => command::rm::rm(&mut command_args.to_vec()),
                                        "ls" =>
                                            command::ls
                                                ::ls(&command_args)
                                                .expect("Failed to execute ls command"),
                                        "exit" => {
                                            return;
                                        }
                                        _ => eprintln!("Command '{}' not found", command),
                                    }
                                }
                                break; // Command complete, exit multi-line loop.
                            }
                            Err(e) if e.contains("unclosed") => {
                                // handle unclosed quotes, wait the close quoete
                                if e.contains("single quote") {
                                    print!("quote>");
                                } else if e.contains("double quote") {
                                    print!("dquote> ");
                                } else if e.contains("dbackslash") {
                                    print!(">");
                                    io::stdout().flush().unwrap();
                                    let mut next_line = String::new();
                                    if io::stdin().read_line(&mut next_line).unwrap_or(0) == 0 {
                                        println!();
                                        continue;
                                    }
                                    print!("{}", next_line);
                                    break;
                                }
                                io::stdout().flush().unwrap();
                                let mut next_line = String::new();
                                if io::stdin().read_line(&mut next_line).unwrap_or(0) == 0 {
                                    println!();
                                    continue;
                                }
                                current_input.push_str(&next_line);
                            }
                            Err(e) => {
                                // Another kind of parsing error.
                                eprintln!("rush: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}*/


fn main() {
    let command_args = vec!["Hello", "World!"];
    command::echo::echo(command_args);
}