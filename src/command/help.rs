pub fn help() {
    println!("Available commands:");
    println!("  echo <args>    - Prints the provided arguments to the standard output.");
    println!("  cd <dir>      - Changes the current working directory.");
    println!("  pwd           - Prints the current working directory.");
    println!("  cat <file>    - Reads and prints the contents of the specified file.");
    println!("  mkdir <dir>   - Creates a directory with the specified name.");
    println!("  cp <src> <dst>- Copies a file from source to destination.");
    println!("  mv <src> <dst>- Moves or renames a file.");
    println!("  help          - Displays this help message.");
    println!("  clear         - Clears the terminal screen.");
    println!("  rm <file>     - Removes the specified file or directory.");
    println!("  ls <args>     - Lists files and directories in the current directory.");
}