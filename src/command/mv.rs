use std::fs;
use std::io;
use std::path::{Path, PathBuf};


pub fn mv(args: &mut Vec<&str>) {
    if args.len() < 2 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let source = args.remove(0);
    let destination = args.remove(0);

    if is_file(source) && !is_dir(destination) {
        match std::fs::rename(source, destination) {
            Ok(_) => println!("Moved '{}' to '{}'", source, destination),
            Err(e) => eprintln!("Error moving file: {}", e),
        }
    } else if (is_file(source) || is_dir(source)) && is_dir(destination) {
        match move_to_dir(source, destination) {
            Ok(new_path) => println!("Moved to: {}", new_path.display()),
            Err(e) => eprintln!("Error moving file: {}", e),
        }
    }
}


fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}



fn move_to_dir<S: AsRef<Path>, D: AsRef<Path>>(source: S, target_dir: D) -> io::Result<PathBuf> {
    let source = source.as_ref();
    let target_dir = target_dir.as_ref();

    if !target_dir.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Target is not a directory or doesn't exist"));
    }

    let file_name = source.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Source has no valid file name"))?;

    let destination = target_dir.join(file_name);

    fs::rename(source, &destination)?;

    Ok(destination)
}