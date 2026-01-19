use std::fs;
use std::io;
use std::path::Path;

/// List directory contents with various formatting options
pub fn list_dir(path: &Path, json: bool, recursive: bool, hidden: bool) {
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", path.display());
        return;
    }

    // Start recursion with depth 0
    if let Err(e) = list_recursive(path, json, recursive, hidden, 0) {
        eprintln!("Error reading directory: {}", e);
    }
}

/// Recursively list directory contents
fn list_recursive(
    path: &Path,
    json: bool,
    recursive: bool,
    hidden: bool,
    depth: usize,
) -> io::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = child_path.is_dir();

        // Filter hidden files if not requested
        if !hidden && name.starts_with('.') {
            continue;
        }

        // Print formatted output
        print_entry(&name, &child_path, is_dir, json, depth);

        // Recurse if directory and recursive flag is set
        if recursive && is_dir {
            list_recursive(&child_path, json, recursive, hidden, depth + 1)?;
        }
    }

    Ok(())
}

/// Print a single directory entry in the requested format
fn print_entry(name: &str, path: &Path, is_dir: bool, json: bool, depth: usize) {
    if json {
        // Use the `json!` macro from `serde_json` for safe serialization
        let output = serde_json::json!({
            "name": name,
            "path": path.display().to_string(),
            "is_dir": is_dir
        });
        println!("{}", output);
    } else {
        let indent = "  ".repeat(depth);
        let icon = if is_dir { "📁" } else { "📄" };
        println!("{}{} {}", indent, icon, name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_list_dir_current() {
        let path = Path::new(".");
        list_dir(path, false, false, false);
    }

    #[test]
    fn test_list_dir_recursive() {
        let path = Path::new(".");
        list_dir(path, false, true, false);
    }

    #[test]
    fn test_list_dir_hidden() {
        let path = Path::new(".");
        list_dir(path, false, false, true);
    }

    #[test]
    fn test_list_dir_json() {
        let path = Path::new(".");
        list_dir(path, true, false, false);
    }

    #[test]
    fn test_invalid_path() {
        let path = Path::new("/this/path/should/not/exist/hopefully");
        list_dir(path, false, false, false);
        // Should print error but not panic
    }

    #[test]
    fn test_print_entry_json() {
        let path = PathBuf::from("test.txt");
        print_entry("test.txt", &path, false, true, 0);
    }

    #[test]
    fn test_print_entry_normal() {
        let path = PathBuf::from("test_dir");
        print_entry("test_dir", &path, true, false, 0);
    }

    #[test]
    fn test_print_entry_with_depth() {
        let path = PathBuf::from("nested/file.txt");
        print_entry("file.txt", &path, false, false, 2);
    }
}
