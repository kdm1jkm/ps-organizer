//! 명령줄 인터페이스 정의.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ps_organizer",
    version,
    about = "PS 문제 소스코드 자동 정리 유틸리티"
)]
pub struct Cli {
    /// 정리할 대상 디렉토리.
    #[arg(default_value = ".")]
    pub target_dir: PathBuf,

    /// 그룹핑 임계값 (이 값 이하면 폴더를 만들지 않음).
    #[arg(short, long, default_value_t = 20)]
    pub threshold: usize,

    /// 실제 이동 없이 계획만 출력.
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// 상세 출력 모드.
    #[arg(short, long)]
    pub verbose: bool,
}
