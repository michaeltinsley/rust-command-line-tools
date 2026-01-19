# File Lister

A simple command-line tool written in Rust for listing files and directories with various formatting options.

## Features

- List files and directories in a specified path
- Recursive directory traversal
- Show/hide hidden files
- JSON output format
- Visual icons for files and directories (📁 for directories, 📄 for files)

## Project Structure

```
file-lister/
├── src/
│   ├── main.rs         # Main application logic and entry point
│   ├── arg_parser.rs   # Command-line argument parsing using clap
│   └── file_handler.rs # File operations and directory listing logic
├── Cargo.toml          # Rust project configuration and dependencies
└── README.md           # Project documentation
```

## Installation

```bash
cargo build --release
```

## Usage

```bash
# List files in current directory
cargo run

# List files in a specific directory
cargo run -- /path/to/directory

# List files recursively
cargo run -- -r

# Show hidden files
cargo run -- -a

# Output in JSON format
cargo run -- -j

# Combine multiple options
cargo run -- /path/to/directory -r -a -j
```

## Command-Line Options

- `[PATH]` - Path to the directory to list (default: current directory)
- `-j, --json` - Output in JSON format
- `-r, --recursive` - List directories recursively
- `-a, --all` - List hidden files (files starting with '.')
- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Examples

### Basic listing
```bash
$ cargo run
📁 src
📄 Cargo.toml
📄 README.md
```

### Recursive listing
```bash
$ cargo run -- -r
📁 src
  📄 main.rs
  📄 arg_parser.rs
  📄 file_handler.rs
📄 Cargo.toml
📄 README.md
```

### JSON output
```bash
$ cargo run -- -j
{"is_dir":true,"name":"src","path":"./src"}
{"is_dir":false,"name":"Cargo.toml","path":"./Cargo.toml"}
{"is_dir":false,"name":"README.md","path":"./README.md"}
```

## Running Tests

```bash
cargo test
```

## Dependencies

- [clap](https://docs.rs/clap/) - Command-line argument parsing
- [serde_json](https://docs.rs/serde_json/) - JSON serialization

## Module Breakdown

### `main.rs`
Entry point of the application. Coordinates between argument parsing and file handling modules.

### `arg_parser.rs`
Handles command-line argument parsing using the `clap` crate with derive macros. Defines the `Args` struct with all CLI options and their default values.

### `file_handler.rs`
Contains all file system operations:
- `list_dir()` - Main entry point for directory listing
- `list_recursive()` - Recursively traverses directories
- `print_entry()` - Formats and prints individual file/directory entries

## License

MIT