pub fn ls(args: &[&str]) -> Result<(), String> {
    let paths = std::fs::read_dir(".").map_err(|e| e.to_string())?;
    
    for path in paths {
        let entry = path.map_err(|e| e.to_string())?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    
    Ok(())
}