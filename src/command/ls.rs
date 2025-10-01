use std::io;
use std::path::Path;
use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::FileTypeExt;



pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut paths = Vec::new();
    let mut flags = Flags::new();

    parse(args, &mut flags, &mut paths)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    show_elems(paths, flags)?;

    Ok(())
}


#[derive(Debug)]
#[allow(non_snake_case)]
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


fn show_elems(paths: Vec<String>, flags: Flags) -> io::Result<()> {
    for path_str in &paths {
        let path = Path::new(&path_str);

        if paths.len() > 1 {
            println!("{}:", path.display());
        }

        let entries = fs::read_dir(path)?;

        let mut names = Vec::new();

        if flags.a {
            names.push("\x1b[34m.\x1b[0m".to_string());
            names.push("\x1b[34m..\x1b[0m".to_string());
        }
        

        for entry in entries {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if !flags.a && name.starts_with('.') {
                continue;
            }

            let classified_name = if flags.F {
                classify_with_suffix(&entry.path(), &name)
            } else {
                name.to_string()
            };

            if file_type.is_dir() {
                names.push(format!("\x1b[34m{}\x1b[0m", classified_name));
            } else {
                names.push(classified_name);
            }
        }


        names.sort_by(|a, b| {
            let ansi_re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();

            let a_clean = ansi_re.replace_all(a, "").to_string();
            let b_clean = ansi_re.replace_all(b, "").to_string();

            let a_stripped = a_clean
                .chars()
                .skip_while(|c| !c.is_alphanumeric())
                .collect::<String>();
            let b_stripped = b_clean
                .chars()
                .skip_while(|c| !c.is_alphanumeric())
                .collect::<String>();

            let cmp = a_stripped.to_lowercase().cmp(&b_stripped.to_lowercase());

            if cmp == std::cmp::Ordering::Equal {
                a_clean.cmp(&b_clean)
            } else {
                cmp
            }
        });


        for name in names {
            print!("{}  ", name);
        }
        println!();

        if paths.len() > 1 {
            println!();
        }
    }

    Ok(())
}



pub fn classify_with_suffix(path: &Path, file_name: &str) -> String {
    let metadata_result = fs::symlink_metadata(path);
    if metadata_result.is_err() {
        return file_name.to_string();
    }

    let metadata = metadata_result.unwrap();
    let file_type = metadata.file_type();

    if file_type.is_dir() {
        return format!("{}/", file_name);
    }

    if file_type.is_symlink() {
        return format!("{}@", file_name);
    }

    #[cfg(unix)]
    if file_type.is_fifo() {
        return format!("{}|", file_name);
    }

    #[cfg(unix)]
    if file_type.is_socket() {
        return format!("{}=", file_name);
    }

    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();
        if mode & 0o111 != 0 {
            return format!("{}*", file_name);
        }
    }

    file_name.to_string()
}