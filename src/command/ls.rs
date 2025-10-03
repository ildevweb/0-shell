use std::io;
use std::path::Path;
use std::fs::{self, Metadata};
use std::os::unix::fs::{PermissionsExt, FileTypeExt, MetadataExt};
use chrono::{DateTime, Local};
use users::{get_user_by_uid, get_group_by_gid};
use std::collections::HashMap;




pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut paths = Vec::new();
    let mut flags = Flags::new();

    parse(args, &mut flags, &mut paths)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    if flags.l {
        show_long_listing(paths, flags)?;
    } else {
        show_elems(paths, flags)?;
    }

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

            
            names.push(classified_name);
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
        return format!("\x1b[34m{}\x1b[0m/", file_name);
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





fn show_long_listing(paths: Vec<String>, flags: Flags) -> io::Result<()> {
    for path_str in &paths {
        let path = Path::new(&path_str);
        let new_path = Path::new(".");
        let mut blocks = 0;
        let mut result: Vec<HashMap<&str, String>> = Vec::new();
        let mut map = HashMap::new();

        if paths.len() > 1 {
            println!("{}:", path.display());
        }


        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(Result::ok)
            .collect();



        entries.sort_by(|a, b| {
            let a_name = a.file_name().to_string_lossy().to_string();
            let b_name = b.file_name().to_string_lossy().to_string();

            let strip_punct = |s: &str| {
                s.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
                    .to_lowercase()
            };

            strip_punct(&a_name).cmp(&strip_punct(&b_name))
        });


        if flags.a {
            let dirs = vec![".", ".."];
            

            for entry in dirs {
                let path = Path::new(entry);
                let metadata = fs::metadata(path)?;
                let file_type = metadata.file_type();
                let name_str = format!("\x1b[34m{}\x1b[0m", entry);

                blocks += metadata.blocks();

                // Permissions string
                let perms = build_permissions_string(&metadata, &file_type);

                // Number of hard links
                let hard_links = metadata.nlink();

                // Owner and group
                let uid = metadata.uid();
                let gid = metadata.gid();
                let user = get_user_by_uid(uid)
                    .and_then(|u| Some(u.name().to_string_lossy().to_string()))
                    .unwrap_or(uid.to_string());
                let group = get_group_by_gid(gid)
                    .and_then(|g| Some(g.name().to_string_lossy().to_string()))
                    .unwrap_or(gid.to_string());

                // File size
                let size = metadata.len();


                // Modification time
                let mtime = metadata.mtime();
                let system_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(mtime as u64 + 60*60);
                let datetime: DateTime<Local> = DateTime::<Local>::from(system_time);
                //let datetime: DateTime<Local> = DateTime::from(std::time::UNIX_EPOCH + std::time::Duration::from_secs(mtime as u64));
                let formatted_time = datetime.format("%b %e %H:%M").to_string(); // e.g., "Oct  1 14:00"


                // File name (with suffix if -F)
                let mut name_out = name_str.to_string();
                if flags.F {
                    let full_path = Path::new(entry);
                    name_out = classify_with_suffix(&full_path, &name_out);
                }

                // Print result
                /*println!("{:<10} {:<3} {:<8} {:<8} {:>6} {} {}", 
                    perms, hard_links, user, group, size, formatted_time, name_out
                );*/

                map.insert("permissions", perms);
                map.insert("hard_links", hard_links.to_string());
                map.insert("owner", user);
                map.insert("group", group);
                map.insert("size", size.to_string());
                map.insert("time", formatted_time);
                map.insert("name", name_out);

                result.push(map);
                map = HashMap::new();

            }
        }
        



        for entry in entries {
            //let entry = entry?;
            let file_type = entry.file_type()?;
            let metadata = entry.metadata()?;
            let file_name = entry.file_name();
            let mut name_str = file_name.to_string_lossy();

            if !flags.a && name_str.starts_with('.') {
                continue;
            }

            if file_type.is_dir() {
                name_str = format!("\x1b[34m{}\x1b[0m", file_name.to_string_lossy()).into();
            }

            blocks += metadata.blocks();

            // Permissions string
            let perms = build_permissions_string(&metadata, &file_type);

            // Number of hard links
            let hard_links = metadata.nlink();

            // Owner and group
            let uid = metadata.uid();
            let gid = metadata.gid();
            let user = get_user_by_uid(uid)
                .and_then(|u| Some(u.name().to_string_lossy().to_string()))
                .unwrap_or(uid.to_string());
            let group = get_group_by_gid(gid)
                .and_then(|g| Some(g.name().to_string_lossy().to_string()))
                .unwrap_or(gid.to_string());

            // File size
            let size = metadata.len();


            // Modification time
            let mtime = metadata.mtime();
            let system_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(mtime as u64 + 60*60);
            let datetime: DateTime<Local> = DateTime::<Local>::from(system_time);
            //let datetime: DateTime<Local> = DateTime::from(std::time::UNIX_EPOCH + std::time::Duration::from_secs(mtime as u64));
            let formatted_time = datetime.format("%b %e %H:%M").to_string(); // e.g., "Oct  1 14:00"


            // File name (with suffix if -F)
            let mut name_out = name_str.to_string();
            if flags.F {
                let full_path = entry.path();
                name_out = classify_with_suffix(&full_path, &name_out);
            }


            // Print result
            /*println!("{:<10} {:<3} {:<8} {:<8} {:>6} {} {}", 
                perms, hard_links, user, group, size, formatted_time, name_out
            );*/


            map.insert("permissions", perms);
            map.insert("hard_links", hard_links.to_string());
            map.insert("owner", user);
            map.insert("group", group);
            map.insert("size", size.to_string());
            map.insert("time", formatted_time);
            map.insert("name", name_out);

            result.push(map);
            map = HashMap::new();
        }

        //print total blocks
        println!("total {}", blocks/2);

        //print
        for mp in result {
            println!("{:<10} {:<3} {:<8} {:<8} {:>6} {} {}", 
                mp.get("permissions").unwrap(), mp.get("hard_links").unwrap(), mp.get("owner").unwrap(), mp.get("group").unwrap(), mp.get("size").unwrap(), mp.get("time").unwrap(), mp.get("name").unwrap()
            );
        }

        if paths.len() > 1 {
            println!();
        }
    }

    Ok(())
}


fn build_permissions_string(metadata: &fs::Metadata, file_type: &fs::FileType) -> String {
    let mut perms = String::new();

    perms.push(if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else {
        '-'
    });

    let mode = metadata.mode();
    for i in (0..3).rev() {
        let shift = i * 3;
        let perm = (mode >> shift) & 0o7;
        perms.push(if perm & 0o4 != 0 { 'r' } else { '-' });
        perms.push(if perm & 0o2 != 0 { 'w' } else { '-' });
        perms.push(if perm & 0o1 != 0 { 'x' } else { '-' });
    }

    perms
}