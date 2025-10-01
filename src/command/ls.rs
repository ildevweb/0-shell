use std::io;


pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut paths = Vec::new();
    let mut flags = Flags::new();

    parse(args, &mut flags, &mut paths)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    println!("this is the paths: {:?}", paths);
    println!("this is the flags: {:?}", flags);
    
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