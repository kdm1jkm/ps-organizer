use std::fs::{self, File};
use tempfile::TempDir;

fn create_test_file(dir: &std::path::Path, name: &str) {
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    File::create(&path).unwrap();
}

#[test]
fn integration_organize_flat_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    for i in 1001..=1010 {
        create_test_file(root, &format!("{}.cpp", i));
    }

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    assert_eq!(entries.len(), 10);

    let moves = ps_organizer::planner::plan_moves(&entries, 20, '_');
    assert!(moves.is_empty());
}

#[test]
fn integration_organize_needs_grouping() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    for i in 1001..=1050 {
        create_test_file(root, &format!("{}.cpp", i));
    }

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    assert_eq!(entries.len(), 50);

    let moves = ps_organizer::planner::plan_moves(&entries, 20, '_');
    assert!(!moves.is_empty());

    ps_organizer::executor::execute_moves(root, &moves, false).unwrap();

    assert!(!root.join("1001.cpp").exists());

    let subfolders: Vec<_> = fs::read_dir(root)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    assert!(!subfolders.is_empty());
}

#[test]
fn integration_etc_folder_for_non_matching() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    create_test_file(root, "solution.cpp");
    create_test_file(root, "main.c");
    create_test_file(root, "1001.cpp");

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    assert_eq!(entries.len(), 3);

    let moves = ps_organizer::planner::plan_moves(&entries, 20, '_');

    let etc_moves: Vec<_> = moves.iter().filter(|m| m.to.starts_with("etc")).collect();
    assert_eq!(etc_moves.len(), 2);
}

#[test]
fn integration_cleanup_empty_dirs() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    fs::create_dir_all(root.join("old/nested")).unwrap();
    create_test_file(root, "old/nested/1001.cpp");

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    assert_eq!(entries.len(), 1);

    let moves = ps_organizer::planner::plan_moves(&entries, 20, '_');
    ps_organizer::executor::execute_moves(root, &moves, false).unwrap();
    ps_organizer::executor::cleanup_empty_dirs(root, false).unwrap();

    assert!(!root.join("old").exists());
}

#[test]
fn integration_conflict_resolution() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    create_test_file(root, "1001.cpp");
    create_test_file(root, "backup/1001.cpp");

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    assert_eq!(entries.len(), 2);

    let moves = ps_organizer::planner::plan_moves(&entries, 20, '_');
    ps_organizer::executor::execute_moves(root, &moves, false).unwrap();

    let cpp_files: Vec<_> = fs::read_dir(root)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "cpp")
                .unwrap_or(false)
        })
        .collect();
    assert_eq!(cpp_files.len(), 2);
}

#[test]
fn integration_already_organized() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    for i in 1001..=1010 {
        create_test_file(root, &format!("{}.cpp", i));
    }

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    let moves = ps_organizer::planner::plan_moves(&entries, 20, '_');

    assert!(moves.is_empty());
}

#[test]
fn integration_placeholder_x() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    for i in 1001..=1050 {
        create_test_file(root, &format!("{}.cpp", i));
    }

    let entries = ps_organizer::scanner::scan_directory(&root.to_path_buf());
    let moves = ps_organizer::planner::plan_moves(&entries, 20, 'x');

    assert!(!moves.is_empty());
    let sample = moves
        .iter()
        .find(|m| m.from.to_string_lossy().contains("1001"))
        .unwrap();
    assert!(sample.to.to_string_lossy().contains("x"));
}
