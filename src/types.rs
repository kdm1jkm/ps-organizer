//! 핵심 데이터 타입 정의.
//!
//! 이 모듈은 파일 정리 작업에 필요한 기본 데이터 구조를 정의합니다.

use std::path::PathBuf;

/// 스캔된 소스 파일 정보.
///
/// 디렉토리 스캔 시 발견된 각 파일의 현재 경로, 추출된 문제 번호,
/// 파일명을 저장합니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// 루트 디렉토리 기준 상대 경로.
    pub current_path: PathBuf,
    /// 파일명에서 추출한 문제 번호 (없으면 `None`).
    pub problem_number: Option<u32>,
    /// 파일명 (확장자 포함).
    pub filename: String,
}

/// 파일 이동 작업 정보.
///
/// 원본 경로에서 목적지 경로로의 이동을 나타냅니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveOperation {
    /// 이동 전 상대 경로.
    pub from: PathBuf,
    /// 이동 후 상대 경로.
    pub to: PathBuf,
}

impl MoveOperation {
    /// 새 이동 작업을 생성합니다.
    pub const fn new(from: PathBuf, to: PathBuf) -> Self {
        Self { from, to }
    }

    /// 실제 이동이 필요한지 확인합니다.
    ///
    /// 원본과 목적지가 같으면 `false`를 반환합니다.
    pub fn is_needed(&self) -> bool {
        self.from != self.to
    }
}
