use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ps_organizer",
    version,
    about = "PS 문제 소스코드 자동 정리 유틸리티"
)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub target_dir: PathBuf,

    #[arg(short, long, default_value_t = 20)]
    pub threshold: usize,

    #[arg(short = 'n', long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub verbose: bool,
}
