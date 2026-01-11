use crate::types::FileEntry;
use std::path::PathBuf;
use walkdir::WalkDir;

const VALID_EXTENSIONS: [&str; 3] = ["cpp", "c", "py"];

pub fn extract_problem_number(filename: &str) -> Option<u32> {
    for ext in VALID_EXTENSIONS {
        let suffix = format!(".{}", ext);
        if let Some(stem) = filename.strip_suffix(&suffix) {
            if !stem.is_empty() && stem.chars().all(|c| c.is_ascii_digit()) {
                return stem.parse::<u32>().ok();
            }
        }
    }
    None
}

pub fn scan_directory(root: &PathBuf) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        if !VALID_EXTENSIONS.contains(&ext) {
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        if filename.is_empty() {
            continue;
        }

        let relative_path = path.strip_prefix(root).unwrap_or(path).to_path_buf();

        let problem_number = extract_problem_number(&filename);

        entries.push(FileEntry {
            current_path: relative_path,
            problem_number,
            filename,
        });
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_simple_cpp() {
        assert_eq!(extract_problem_number("1010.cpp"), Some(1010));
    }

    #[test]
    fn extract_simple_c() {
        assert_eq!(extract_problem_number("999.c"), Some(999));
    }

    #[test]
    fn extract_simple_py() {
        assert_eq!(extract_problem_number("12345.py"), Some(12345));
    }

    #[test]
    fn reject_with_prefix() {
        assert_eq!(extract_problem_number("problem_1234.py"), None);
    }

    #[test]
    fn reject_with_suffix() {
        assert_eq!(extract_problem_number("1234_solution.cpp"), None);
    }

    #[test]
    fn reject_no_number() {
        assert_eq!(extract_problem_number("main.cpp"), None);
    }

    #[test]
    fn reject_wrong_extension() {
        assert_eq!(extract_problem_number("1010.java"), None);
    }

    #[test]
    fn reject_mixed_chars() {
        assert_eq!(extract_problem_number("1a2b.cpp"), None);
    }

    #[test]
    fn handle_leading_zeros() {
        assert_eq!(extract_problem_number("0001.cpp"), Some(1));
    }

    #[test]
    fn handle_zero() {
        assert_eq!(extract_problem_number("0.cpp"), Some(0));
    }

    #[test]
    fn reject_capital_extension() {
        assert_eq!(extract_problem_number("1010.CPP"), None);
    }
}
