use crate::grouper::compute_structure;
use crate::types::{FileEntry, MoveOperation};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn resolve_conflict(filename: &str, conflict_num: u32) -> String {
    if let Some(dot_pos) = filename.rfind('.') {
        let (name, ext) = filename.split_at(dot_pos);
        format!("{}_conflict{}{}", name, conflict_num, ext)
    } else {
        format!("{}_conflict{}", filename, conflict_num)
    }
}

pub fn plan_moves(
    entries: &[FileEntry],
    threshold: usize,
    placeholder: char,
) -> Vec<MoveOperation> {
    let numbers: Vec<u32> = entries.iter().filter_map(|e| e.problem_number).collect();

    let structure = compute_structure(&numbers, threshold, placeholder);

    let mut moves = Vec::new();
    let mut target_files: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    let mut conflict_counts: HashMap<String, u32> = HashMap::new();

    for entry in entries {
        let target_folder = match entry.problem_number {
            Some(num) => structure.get(&num).cloned().unwrap_or_default(),
            None => "etc".to_string(),
        };

        let target_folder_path = PathBuf::from(&target_folder);
        let existing_files = target_files.entry(target_folder_path.clone()).or_default();

        let final_filename = if existing_files.contains(&entry.filename) {
            let key = format!("{}/{}", target_folder, entry.filename);
            let count = conflict_counts.entry(key).or_insert(0);
            *count += 1;
            resolve_conflict(&entry.filename, *count)
        } else {
            entry.filename.clone()
        };

        existing_files.insert(final_filename.clone());

        let target_path = if target_folder.is_empty() {
            PathBuf::from(&final_filename)
        } else {
            PathBuf::from(&target_folder).join(&final_filename)
        };

        moves.push(MoveOperation::new(entry.current_path.clone(), target_path));
    }

    moves.into_iter().filter(|m| m.is_needed()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_conflict_adds_suffix() {
        assert_eq!(resolve_conflict("1010.cpp", 1), "1010_conflict1.cpp");
        assert_eq!(resolve_conflict("1010.cpp", 2), "1010_conflict2.cpp");
    }

    #[test]
    fn resolve_conflict_no_extension() {
        assert_eq!(resolve_conflict("readme", 1), "readme_conflict1");
    }

    #[test]
    fn plan_moves_no_move_when_already_correct() {
        let entries = vec![FileEntry {
            current_path: PathBuf::from("1010.cpp"),
            problem_number: Some(1010),
            filename: "1010.cpp".to_string(),
        }];

        let moves = plan_moves(&entries, 20, '_');
        assert!(moves.is_empty());
    }

    #[test]
    fn plan_moves_etc_for_non_matching() {
        let entries = vec![FileEntry {
            current_path: PathBuf::from("solution.cpp"),
            problem_number: None,
            filename: "solution.cpp".to_string(),
        }];

        let moves = plan_moves(&entries, 20, '_');
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to, PathBuf::from("etc/solution.cpp"));
    }

    #[test]
    fn plan_moves_handles_conflict() {
        let entries = vec![
            FileEntry {
                current_path: PathBuf::from("1010.cpp"),
                problem_number: Some(1010),
                filename: "1010.cpp".to_string(),
            },
            FileEntry {
                current_path: PathBuf::from("old/1010.cpp"),
                problem_number: Some(1010),
                filename: "1010.cpp".to_string(),
            },
        ];

        let moves = plan_moves(&entries, 20, '_');

        let destinations: Vec<_> = moves.iter().map(|m| m.to.clone()).collect();
        assert!(
            destinations.contains(&PathBuf::from("1010.cpp"))
                || destinations.contains(&PathBuf::from("1010_conflict1.cpp"))
        );
    }

    #[test]
    fn plan_moves_splits_when_over_threshold() {
        let entries: Vec<FileEntry> = (1001..=1050)
            .map(|n| FileEntry {
                current_path: PathBuf::from(format!("{}.cpp", n)),
                problem_number: Some(n),
                filename: format!("{}.cpp", n),
            })
            .collect();

        let moves = plan_moves(&entries, 20, '_');

        assert!(!moves.is_empty());
        let sample_move = moves.iter().find(|m| m.from == PathBuf::from("1001.cpp"));
        assert!(sample_move.is_some());
        assert!(sample_move.unwrap().to.to_string_lossy().contains("100_"));
    }
}
