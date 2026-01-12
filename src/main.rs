mod cli;
mod executor;
mod grouper;
mod planner;
mod scanner;
mod types;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();

    let root = args.target_dir.canonicalize().unwrap_or(args.target_dir);

    if args.verbose {
        println!("대상 디렉토리: {}", root.display());
        println!("임계값: {}", args.threshold);
        println!("Placeholder: '{}'", args.placeholder);
        println!("Dry-run: {}\n", args.dry_run);
    }

    let entries = scanner::scan_directory(&root);

    if args.verbose {
        println!("스캔된 파일: {} 개\n", entries.len());
    }

    if entries.is_empty() {
        println!("정리할 파일이 없습니다.");
        return Ok(());
    }

    let moves = planner::plan_moves(&entries, args.threshold, args.placeholder);

    if args.dry_run {
        executor::print_plan(&moves, args.verbose);
    } else {
        executor::execute_moves(&root, &moves, args.verbose)?;
        executor::cleanup_empty_dirs(&root, args.verbose)?;
    }

    Ok(())
}
