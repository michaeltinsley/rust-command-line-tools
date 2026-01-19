use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments for the file lister application
#[derive(Parser, Debug)]
#[command(name = "file-lister")]
#[command(author, version, about = "A simple file and directory lister", long_about = None)]
pub struct Args {
    /// Path to the directory to list
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output in JSON format
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// List directories recursively
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,

    /// List hidden files (files starting with '.')
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
}

impl Args {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Args::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_defaults() {
        // Test that default values are properly set
        let args = Args {
            path: PathBuf::from("."),
            json: false,
            recursive: false,
            all: false,
        };

        assert_eq!(args.path, PathBuf::from("."));
        assert!(!args.json);
        assert!(!args.recursive);
        assert!(!args.all);
    }
}
