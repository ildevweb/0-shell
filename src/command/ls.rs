use std::fs;
use std::io;
use std::path::Path;

pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut paths = Vec::new();
    let mut flags = Flags::new();

    parse(args, &mut flags, &mut paths)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for path_str in &paths {
        let path = Path::new(&path_str);

        if paths.len() > 1 {
            println!("{}:", path.display());
        }

        let entries = fs::read_dir(path)?;

        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in entries {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if !flags.a && name.starts_with('.') {
                continue;
            }

            if file_type.is_dir() {
                dirs.push(format!("\x1b[34m{}\x1b[0m", name));
            } else {
                files.push(name.to_string());
            }
        }

        files.sort(); 
        for name in files {
            print!("{}  ", name);
        }
        dirs.sort(); 
        for dir in dirs {
            print!("{}  ", dir);
        }
        println!();

        if paths.len() > 1 {
            println!();
        }
    }

    Ok(())
}




#[derive(Debug)]
struct Flags {
    a: bool,
    F: bool,
    l: bool,
}

impl Flags {
    fn new() -> Self {
        Self { a: false, F: false, l: false }
    }
}

fn parse(args: &[&str], flg: &mut Flags, paths: &mut Vec<String>) -> io::Result<()> {
    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            let arg_flags = arg.trim_start_matches('-');
            for f in arg_flags.chars() {
                match f {
                    'F' => flg.F = true,
                    'l' => flg.l = true,
                    'a' => flg.a = true,
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput, 
                            format!("ls: invalid option -- '{}'", f)
                        ));
                    }
                }
            }
        } else {
            paths.push(arg.to_string());
        }
    }
    Ok(())
}