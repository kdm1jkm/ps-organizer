//! 디렉토리 스캔 및 문제 번호 추출.

use crate::types::FileEntry;
use std::path::Path;
use walkdir::WalkDir;

const VALID_EXTENSIONS: [&str; 3] = ["cpp", "c", "py"];

/// 파일명에서 문제 번호를 추출합니다.
///
/// `숫자.확장자` 형식의 파일명만 인식합니다 (예: `1010.cpp` → `Some(1010)`).
/// 파일명에 숫자 외의 문자가 포함되거나 지원하지 않는 확장자면 `None`을 반환합니다.
pub fn extract_problem_number(filename: &str) -> Option<u32> {
    for ext in VALID_EXTENSIONS {
        let suffix = format!(".{ext}");
        if let Some(stem) = filename.strip_suffix(&suffix)
            && !stem.is_empty()
            && stem.chars().all(|c| c.is_ascii_digit())
        {
            return stem.parse::<u32>().ok();
        }
    }
    None
}

/// 디렉토리를 재귀적으로 스캔하여 소스 파일 목록을 반환합니다.
///
/// `.cpp`, `.c`, `.py` 확장자를 가진 파일만 수집합니다.
pub fn scan_directory(root: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
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
