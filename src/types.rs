use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub current_path: PathBuf,
    pub problem_number: Option<u32>,
    pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveOperation {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl MoveOperation {
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Self { from, to }
    }

    pub fn is_needed(&self) -> bool {
        self.from != self.to
    }
}
