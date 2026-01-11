use crate::types::MoveOperation;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn print_plan(moves: &[MoveOperation], verbose: bool) {
    if moves.is_empty() {
        println!("변경 사항 없음. 모든 파일이 이미 올바른 위치에 있습니다.");
        return;
    }

    println!("=== 이동 계획 ({} 개 파일) ===\n", moves.len());

    for op in moves {
        println!("  {} -> {}", op.from.display(), op.to.display());
    }

    if verbose {
        let folders: HashSet<_> = moves
            .iter()
            .filter_map(|m| m.to.parent())
            .filter(|p| !p.as_os_str().is_empty())
            .collect();

        if !folders.is_empty() {
            println!("\n생성될 폴더:");
            for folder in folders {
                println!("  {}/", folder.display());
            }
        }
    }
}

pub fn execute_moves(root: &Path, moves: &[MoveOperation], verbose: bool) -> Result<()> {
    if moves.is_empty() {
        println!("변경 사항 없음.");
        return Ok(());
    }

    println!("=== 파일 이동 중 ({} 개) ===\n", moves.len());

    for op in moves {
        let from_abs = root.join(&op.from);
        let to_abs = root.join(&op.to);

        if let Some(parent) = to_abs.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("폴더 생성 실패: {}", parent.display()))?;
                if verbose {
                    println!("  [폴더 생성] {}", parent.display());
                }
            }
        }

        fs::rename(&from_abs, &to_abs).with_context(|| {
            format!(
                "파일 이동 실패: {} -> {}",
                from_abs.display(),
                to_abs.display()
            )
        })?;

        if verbose {
            println!("  [이동] {} -> {}", op.from.display(), op.to.display());
        }
    }

    println!("\n완료: {} 개 파일 이동됨", moves.len());
    Ok(())
}

pub fn cleanup_empty_dirs(root: &Path, verbose: bool) -> Result<()> {
    cleanup_empty_dirs_recursive(root, root, verbose)
}

fn cleanup_empty_dirs_recursive(root: &Path, current: &Path, verbose: bool) -> Result<()> {
    if !current.is_dir() {
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(current)
        .with_context(|| format!("디렉토리 읽기 실패: {}", current.display()))?
        .filter_map(|e| e.ok())
        .collect();

    for entry in &entries {
        let path = entry.path();
        if path.is_dir() {
            cleanup_empty_dirs_recursive(root, &path, verbose)?;
        }
    }

    if current != root {
        let is_empty = fs::read_dir(current)
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false);

        if is_empty {
            fs::remove_dir(current)
                .with_context(|| format!("빈 폴더 삭제 실패: {}", current.display()))?;
            if verbose {
                println!("  [삭제] 빈 폴더: {}", current.display());
            }
        }
    }

    Ok(())
}
