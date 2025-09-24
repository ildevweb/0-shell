# Rust Shell

This project is a simple command-line shell implemented in Rust. It provides basic shell functionalities such as executing commands, changing directories, and managing files.

## Features

- **Command Execution**: Supports various commands like `echo`, `cd`, `pwd`, `cat`, `mkdir`, `cp`, `mv`, `help`, `clear`, `rm`, and `ls`.
- **Interactive Shell**: Users can interact with the shell in a loop, entering commands and receiving output in real-time.
- **Error Handling**: The shell gracefully handles errors, such as invalid commands or issues with file operations.

## Installation

To build and run the project, ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

1. Clone the repository:
   ```
   git clone <repository-url>
   cd rust-shell
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the shell:
   ```
   cargo run
   ```

## Usage

Once the shell is running, you can enter commands as follows:

- `echo Hello World!` - Prints "Hello World!" to the terminal.
- `cd <directory>` - Changes the current working directory to `<directory>`.
- `pwd` - Displays the current working directory.
- `cat <file>` - Displays the contents of `<file>`.
- `mkdir <directory>` - Creates a new directory named `<directory>`.
- `cp <source> <destination>` - Copies a file from `<source>` to `<destination>`.
- `mv <source> <destination>` - Moves or renames a file from `<source>` to `<destination>`.
- `help` - Displays a list of available commands and their usage.
- `clear` - Clears the terminal screen.
- `rm <file>` - Removes the specified `<file>`.
- `ls` - Lists files and directories in the current directory.
- `exit` - Exits the shell.

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.