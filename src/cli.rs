use clap::Parser;
use std::path::PathBuf;

fn parse_placeholder(s: &str) -> Result<char, String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 1 {
        return Err("placeholder must be exactly one character".to_string());
    }
    Ok(chars[0])
}

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

    #[arg(short, long, default_value = "_", value_parser = parse_placeholder)]
    pub placeholder: char,

    #[arg(short = 'n', long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub verbose: bool,
}
