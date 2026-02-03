//! Utility functions for the AllTheSkills library

use std::path::Path;

use crate::Result;

/// Copy a directory recursively from source to destination
///
/// # Arguments
///
/// * `src` - Source directory path
/// * `dst` - Destination directory path
///
/// # Errors
///
/// Returns an error if directory creation or file copying fails
///
/// # Example
///
/// ```rust,no_run
/// use alltheskills::utils::copy_dir_recursive;
/// use std::path::PathBuf;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// copy_dir_recursive(&PathBuf::from("./source"), &PathBuf::from("./dest"))?;
/// # Ok(())
/// # }
/// ```
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(entry.path().as_ref(), &dest_path)?;
        } else {
            std::fs::copy(entry.path(), &dest_path)?;
        }
    }

    Ok(())
}

/// Copy skill directory contents from source to target
///
/// This is a convenience wrapper around `copy_dir_recursive` specifically
/// for installing skills.
pub fn copy_skill_dir(src: &Path, dst: &Path) -> Result<()> {
    copy_dir_recursive(src, dst)
}

/// Check if a path is a valid skill directory
///
/// Returns true if the directory contains at least one recognized
/// skill manifest file.
pub fn is_skill_dir(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    let manifest_files = [
        "claude.json",
        "cline.json",
        "cursor.json",
        "roo.json",
        ".roomodes",
        "manifest.json",
        "skill.json",
        "codex.json",
        "kilo.yaml",
        "kilo.yml",
        "wrangler.toml",
        ".cursorrules",
        "SKILL.md",
    ];

    manifest_files
        .iter()
        .any(|file| path.join(file).exists())
        || path.join("README.md").exists()
}

/// Sanitize a string for use as a file name
///
/// Removes or replaces characters that are not safe for file systems.
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            c => c,
        })
        .collect::<String>()
        .trim_start_matches('.')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("hello/world"), "hello-world");
        assert_eq!(sanitize_filename("file:name"), "file-name");
        assert_eq!(sanitize_filename(".hidden"), "hidden");
        assert_eq!(sanitize_filename("normal"), "normal");
    }
}
