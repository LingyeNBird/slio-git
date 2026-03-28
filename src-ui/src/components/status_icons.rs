//! Status icon helpers for file changes and shell badges.

use crate::theme::{darcula, BadgeTone};
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Unversioned,
    Conflict,
    Ignored,
}

impl FileStatus {
    pub fn color(self) -> Color {
        match self {
            FileStatus::Added => darcula::STATUS_ADDED,
            FileStatus::Modified => darcula::STATUS_MODIFIED,
            FileStatus::Deleted => darcula::STATUS_DELETED,
            FileStatus::Renamed => darcula::STATUS_RENAMED,
            FileStatus::Unversioned => darcula::STATUS_UNVERSIONED,
            FileStatus::Conflict => darcula::DANGER,
            FileStatus::Ignored => darcula::TEXT_DISABLED,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            FileStatus::Added => "A",
            FileStatus::Modified => "M",
            FileStatus::Deleted => "D",
            FileStatus::Renamed => "R",
            FileStatus::Unversioned => "?",
            FileStatus::Conflict => "!",
            FileStatus::Ignored => "I",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            FileStatus::Added => "新增文件",
            FileStatus::Modified => "修改文件",
            FileStatus::Deleted => "删除文件",
            FileStatus::Renamed => "重命名",
            FileStatus::Unversioned => "未跟踪",
            FileStatus::Conflict => "存在冲突",
            FileStatus::Ignored => "已忽略",
        }
    }

    pub fn badge_tone(self) -> BadgeTone {
        match self {
            FileStatus::Added => BadgeTone::Success,
            FileStatus::Modified | FileStatus::Renamed => BadgeTone::Accent,
            FileStatus::Deleted | FileStatus::Conflict => BadgeTone::Danger,
            FileStatus::Unversioned | FileStatus::Ignored => BadgeTone::Neutral,
        }
    }
}

impl From<&git_core::index::ChangeStatus> for FileStatus {
    fn from(status: &git_core::index::ChangeStatus) -> Self {
        match status {
            git_core::index::ChangeStatus::Added => FileStatus::Added,
            git_core::index::ChangeStatus::Modified => FileStatus::Modified,
            git_core::index::ChangeStatus::Deleted => FileStatus::Deleted,
            git_core::index::ChangeStatus::Renamed => FileStatus::Renamed,
            git_core::index::ChangeStatus::Untracked => FileStatus::Unversioned,
            git_core::index::ChangeStatus::Conflict => FileStatus::Conflict,
            git_core::index::ChangeStatus::Ignored => FileStatus::Ignored,
        }
    }
}
