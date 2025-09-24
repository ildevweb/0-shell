mod command;
mod parser;
use std::io::Write;
use std::io;

fn main() {
    if !atty::is(atty::Stream::Stdout) {
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
        let dir_path = std::env::current_dir().unwrap_or_else(|_| "".into());
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());

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
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                for command_str in parser::split_and_parsing_commands(&input) {
                    let mut current_input = command_str.to_string();
                    loop {
                        match parser::parse_args(&current_input) {
                            Ok(args) => {
                                if !args.is_empty() {
                                    let command = args[0];
                                    let command_args: Vec<&str> = args[1..].to_vec();
                                    match command {
                                        "echo" => command::echo::echo(command_args),
                                        "cd" => command::cd::cd(&command_args),
                                        "pwd" => command::pwd::pwd(&command_args),
                                        "cat" => command::cat::cat(&command_args),
                                        "mkdir" => command::mkdir::mkdir(&mut command_args.to_vec()),
                                        "cp" => command::cp::cp(&command_args),
                                        "mv" => command::mv::mv(&mut command_args.to_vec()),
                                        "help" => command::help::help(),
                                        "clear" => command::clear::clear(),
                                        "rm" => command::rm::rm(&mut command_args.to_vec()),
                                        "ls" => command::ls::ls(&command_args)
                                            .expect("Failed to execute ls command"),
                                        "exit" => {
                                            return;
                                        }
                                        _ => eprintln!("Command '{}' not found", command),
                                    }
                                }
                                break;
                            }
                            Err(e) => match e {
                                parser::ParseError::UnclosedSingleQuote => {
                                    print!("quote> ");
                                    io::stdout().flush().unwrap();
                                }
                                parser::ParseError::UnclosedDoubleQuote => {
                                    print!("dquote> ");
                                    io::stdout().flush().unwrap();
                                }
                                parser::ParseError::UnclosedBackslash => {
                                    print!("> ");
                                    io::stdout().flush().unwrap();
                                }
                                parser::ParseError::EmptyInput => {
                                    println!("Input is empty.");
                                    break;
                                }
                                parser::ParseError::Other(msg) => {
                                    println!("Error: {}", msg);
                                    break;
                                }
                            },
                            _ => {
                                println!("rush:");
                                break;
                            }
                        }
                    }
                }
            }
            _ => {
                println!("Error:");
                break;
            }
        }
    }
}