//! Commit dialog view.
//!
//! Provides a dialog for creating and amending commits.

use crate::components::status_icons::FileStatus;
use crate::theme::{self, BadgeTone, Surface};
use crate::widgets::{self, button, diff_viewer, scrollable, OptionalPush};
use git_core::commit::CommitInfo;
use git_core::diff::{Diff, FileDiff};
use git_core::index::Change;
use iced::widget::{text, text_editor, Button, Checkbox, Column, Container, Row, Space, Text};
use iced::{Alignment, Element, Length};

/// Message types for commit dialog.
#[derive(Debug, Clone)]
pub enum CommitDialogMessage {
    MessageEdited(text_editor::Action),
    FileToggled(String, bool),
    PreviewFile(String),
    CommitPressed,
    AmendPressed,
    CancelPressed,
}

/// State for the commit dialog.
#[derive(Debug)]
pub struct CommitDialogState {
    pub message: String,
    pub message_editor: text_editor::Content,
    pub is_amend: bool,
    pub commit_to_amend: Option<CommitInfo>,
    pub diff: Diff,
    pub staged_files: Vec<Change>,
    pub selected_files: Vec<String>,
    pub previewed_file: Option<String>,
    pub is_committing: bool,
    pub error: Option<String>,
    pub success_message: Option<String>,
}

impl CommitDialogState {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            message_editor: text_editor::Content::new(),
            is_amend: false,
            commit_to_amend: None,
            diff: Diff {
                files: Vec::new(),
                total_additions: 0,
                total_deletions: 0,
            },
            staged_files: Vec::new(),
            selected_files: Vec::new(),
            previewed_file: None,
            is_committing: false,
            error: None,
            success_message: None,
        }
    }

    pub fn for_new_commit(staged_files: Vec<Change>, diff: &Diff) -> Self {
        let selected_files = staged_files
            .iter()
            .map(|change| change.path.clone())
            .collect::<Vec<_>>();

        Self {
            message: String::new(),
            message_editor: text_editor::Content::new(),
            is_amend: false,
            commit_to_amend: None,
            diff: diff.clone(),
            staged_files,
            selected_files: selected_files.clone(),
            previewed_file: initial_preview_path(&selected_files, diff),
            is_committing: false,
            error: None,
            success_message: None,
        }
    }

    pub fn for_amend(staged_files: Vec<Change>, commit: CommitInfo, diff: &Diff) -> Self {
        let selected_files = staged_files
            .iter()
            .map(|change| change.path.clone())
            .collect::<Vec<_>>();

        Self {
            message: commit.message.clone(),
            message_editor: text_editor::Content::with_text(&commit.message),
            is_amend: true,
            commit_to_amend: Some(commit),
            diff: diff.clone(),
            staged_files,
            selected_files: selected_files.clone(),
            previewed_file: initial_preview_path(&selected_files, diff),
            is_committing: false,
            error: None,
            success_message: None,
        }
    }

    /// Check if the commit message is valid (non-empty after trimming).
    pub fn is_message_valid(&self) -> bool {
        !self.message.trim().is_empty()
    }

    /// Check if there are files to commit.
    pub fn has_files_to_commit(&self) -> bool {
        self.is_amend || !self.selected_files.is_empty()
    }

    /// Toggle file selection.
    pub fn toggle_file(&mut self, path: String) {
        self.success_message = None;

        if let Some(pos) = self.selected_files.iter().position(|p| p == &path) {
            self.selected_files.remove(pos);
        } else {
            self.selected_files.push(path);
        }

        self.ensure_preview_target();
    }

    pub fn preview_file(&mut self, path: String) {
        self.previewed_file = Some(path);
        self.success_message = None;
    }

    pub fn apply_message_edit(&mut self, action: text_editor::Action) {
        self.message_editor.perform(action);
        self.message = normalize_editor_text(self.message_editor.text());
        self.error = None;
        self.success_message = None;
    }

    /// Set error message.
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.is_committing = false;
        self.success_message = None;
    }

    /// Clear error message.
    pub fn clear_error(&mut self) {
        self.error = None;
    }

    /// Start committing.
    pub fn start_commit(&mut self) {
        self.is_committing = true;
        self.clear_error();
        self.success_message = None;
    }

    /// Finish committing successfully.
    pub fn commit_success(&mut self) {
        self.is_committing = false;
        self.error = None;
        self.success_message = Some(if self.is_amend {
            "已更新最近一次提交。".to_string()
        } else {
            "已创建提交。".to_string()
        });
        self.message.clear();
        self.message_editor = text_editor::Content::new();
        self.selected_files.clear();
        self.previewed_file = None;
    }

    pub fn selected_diff_summary(&self) -> (usize, u32, u32) {
        let mut file_count = 0usize;
        let mut additions = 0u32;
        let mut deletions = 0u32;

        for file in &self.diff.files {
            let Some(path) = diff_file_path(file) else {
                continue;
            };

            if self.selected_files.iter().any(|selected| selected == path) {
                file_count += 1;
                additions += file.additions;
                deletions += file.deletions;
            }
        }

        (file_count, additions, deletions)
    }

    pub fn file_diff(&self, path: &str) -> Option<&FileDiff> {
        self.diff
            .files
            .iter()
            .find(|file| diff_file_path(file) == Some(path))
    }

    fn ensure_preview_target(&mut self) {
        let is_valid = self.previewed_file.as_ref().is_some_and(|path| {
            self.staged_files.iter().any(|change| &change.path == path)
                || self
                    .diff
                    .files
                    .iter()
                    .any(|file| diff_file_path(file) == Some(path.as_str()))
        });

        if !is_valid {
            self.previewed_file = initial_preview_path(&self.selected_files, &self.diff);
        }
    }
}

impl Clone for CommitDialogState {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            message_editor: text_editor::Content::with_text(&self.message),
            is_amend: self.is_amend,
            commit_to_amend: self.commit_to_amend.clone(),
            diff: self.diff.clone(),
            staged_files: self.staged_files.clone(),
            selected_files: self.selected_files.clone(),
            previewed_file: self.previewed_file.clone(),
            is_committing: self.is_committing,
            error: self.error.clone(),
            success_message: self.success_message.clone(),
        }
    }
}

impl Default for CommitDialogState {
    fn default() -> Self {
        Self::new()
    }
}

/// Build the commit dialog view.
pub fn view(state: &CommitDialogState) -> Element<'_, CommitDialogMessage> {
    let title = if state.is_amend {
        "修改上次提交"
    } else {
        "创建提交"
    };
    let detail = if state.is_amend {
        "沿用上一条提交的上下文，在提交前重新确认变更与说明。"
    } else {
        "先确认待提交文件，再填写摘要和补充说明。"
    };

    let files_list: Element<'_, CommitDialogMessage> = if state.staged_files.is_empty() {
        Column::new()
            .push(
                Text::new("当前没有可提交的暂存文件。")
                    .size(12)
                    .color(theme::darcula::TEXT_SECONDARY),
            )
            .into()
    } else {
        state
            .staged_files
            .iter()
            .fold(Column::new().spacing(theme::spacing::XS), |column, file| {
                column.push(build_file_row(state, file))
            })
            .into()
    };

    let files_panel = Container::new(
        Column::new()
            .spacing(theme::spacing::MD)
            .push(
                Row::new()
                    .spacing(theme::spacing::XS)
                    .align_y(Alignment::Center)
                    .push(
                        Text::new("待提交文件")
                            .size(13)
                            .color(theme::darcula::TEXT_SECONDARY),
                    )
                    .push(widgets::info_chip::<CommitDialogMessage>(
                        format!("暂存 {}", state.staged_files.len()),
                        BadgeTone::Success,
                    ))
                    .push(widgets::info_chip::<CommitDialogMessage>(
                        format!("已选 {}", state.selected_files.len()),
                        BadgeTone::Accent,
                    )),
            )
            .push(scrollable::styled(files_list).height(Length::Fixed(240.0))),
    )
    .padding([16, 16])
    .width(Length::FillPortion(2))
    .style(theme::panel_style(Surface::Panel));

    let (selected_file_count, selected_additions, selected_deletions) =
        state.selected_diff_summary();

    let preview_header = Container::new(
        scrollable::styled_horizontal(
            Row::new()
                .spacing(theme::spacing::XS)
                .align_y(Alignment::Center)
                .push(widgets::info_chip::<CommitDialogMessage>(
                    format!("提交文件 {}", selected_file_count),
                    BadgeTone::Success,
                ))
                .push(widgets::info_chip::<CommitDialogMessage>(
                    format!("+{}", selected_additions),
                    BadgeTone::Success,
                ))
                .push(widgets::info_chip::<CommitDialogMessage>(
                    format!("-{}", selected_deletions),
                    BadgeTone::Danger,
                ))
                .push_maybe(state.previewed_file.as_ref().map(|path| {
                    widgets::info_chip::<CommitDialogMessage>(
                        format!("当前预览 {}", path),
                        BadgeTone::Accent,
                    )
                }))
                .push(
                    Text::new("文件变更摘要，具体改动在下方 diff 预览中查看。")
                        .size(11)
                        .color(theme::darcula::TEXT_SECONDARY),
                ),
        )
        .width(Length::Fill),
    )
    .padding([10, 12])
    .style(theme::panel_style(Surface::Raised));

    let preview_body: Element<'_, CommitDialogMessage> =
        if let Some(path) = state.previewed_file.as_deref() {
            if let Some(file_diff) = state.file_diff(path) {
                Container::new(diff_viewer::file_preview(file_diff))
                    .height(Length::Fixed(300.0))
                    .padding([8, 8])
                    .style(theme::panel_style(Surface::Raised))
                    .into()
            } else {
                widgets::panel_empty_state(
                    "预览",
                    "没有找到当前文件的 diff",
                    "请重新选择左侧文件，或刷新后再试。",
                    None,
                )
            }
        } else {
            widgets::panel_empty_state(
                "预览",
                "当前没有可显示的文件改动",
                "先保留至少一个待提交文件，或在左侧点击要预览的文件。",
                None,
            )
        };

    let diff_panel = Container::new(
        Column::new()
            .spacing(theme::spacing::MD)
            .push(widgets::section_header(
                "预览",
                "文件改动",
                "左侧勾选控制是否提交，点击文件名切换具体 diff 预览。",
            ))
            .push(preview_header)
            .push(preview_body),
    )
    .padding([16, 16])
    .width(Length::FillPortion(3))
    .style(theme::panel_style(Surface::Panel));

    let message_panel = Container::new(
        Column::new()
            .spacing(theme::spacing::MD)
            .push(widgets::section_header(
                "消息",
                "提交说明",
                "支持多行编辑，首行作为提交标题。",
            ))
            .push(
                text_editor(&state.message_editor)
                    .placeholder("输入提交消息（第一行为标题）...")
                    .padding([10, 12])
                    .size(f32::from(theme::typography::BODY_SIZE))
                    .height(Length::Fixed(120.0))
                    .style(theme::text_editor_style())
                    .on_action(CommitDialogMessage::MessageEdited),
            )
            .push(
                Text::new(if state.message.trim().is_empty() {
                    "请输入至少一行提交摘要。"
                } else {
                    "提交说明已就绪，可以直接执行提交。"
                })
                .size(12)
                .color(if state.message.trim().is_empty() {
                    theme::darcula::DANGER
                } else {
                    theme::darcula::TEXT_SECONDARY
                }),
            ),
    )
    .padding([16, 16])
    .style(theme::panel_style(Surface::Panel));

    let status_panel = if state.is_committing {
        Some(build_status_panel::<CommitDialogMessage>(
            "处理中",
            "正在写入提交，请稍候。",
            BadgeTone::Neutral,
        ))
    } else if let Some(error) = state.error.as_ref() {
        Some(build_status_panel::<CommitDialogMessage>(
            "失败",
            error,
            BadgeTone::Danger,
        ))
    } else if let Some(message) = state.success_message.as_ref() {
        Some(build_status_panel::<CommitDialogMessage>(
            "完成",
            message,
            BadgeTone::Success,
        ))
    } else if state.staged_files.is_empty() {
        Some(build_status_panel::<CommitDialogMessage>(
            "空状态",
            "当前没有暂存文件，先在工作区整理出一组待提交变更。",
            BadgeTone::Neutral,
        ))
    } else if !state.is_message_valid() {
        Some(build_status_panel::<CommitDialogMessage>(
            "待补充",
            "文件已就绪，再填写一行提交摘要即可执行提交。",
            BadgeTone::Warning,
        ))
    } else {
        Some(build_status_panel::<CommitDialogMessage>(
            "准备就绪",
            format!(
                "当前可提交 {} 个文件，提交说明也已填写完整。",
                state.selected_files.len()
            ),
            BadgeTone::Success,
        ))
    };

    let commit_label = if state.is_committing {
        "提交中..."
    } else if state.is_amend {
        "应用修改"
    } else {
        "创建提交"
    };
    let commit_enabled =
        state.is_message_valid() && state.has_files_to_commit() && !state.is_committing;

    let actions = scrollable::styled_horizontal(
        Row::new()
            .spacing(theme::spacing::XS)
            .push(Space::new().width(Length::Fill))
            .push_maybe((!state.is_amend && !state.is_committing).then(|| {
                button::secondary("修改上次提交", Some(CommitDialogMessage::AmendPressed))
            }))
            .push(button::ghost(
                "取消",
                Some(CommitDialogMessage::CancelPressed),
            ))
            .push(button::primary(
                commit_label,
                commit_enabled.then_some(CommitDialogMessage::CommitPressed),
            )),
    )
    .width(Length::Fill);

    let tool_tabs = Container::new(
        Row::new()
            .spacing(theme::spacing::XS)
            .push(button::tab("提交", true, None::<CommitDialogMessage>))
            .push(button::tab("搁置", false, None::<CommitDialogMessage>))
            .push(button::tab("储藏", false, None::<CommitDialogMessage>)),
    )
    .padding([6, 6])
    .style(theme::frame_style(Surface::Toolbar));

    let overview_cards = Row::new()
        .spacing(theme::spacing::MD)
        .push(widgets::stat_card(
            "待提交文件",
            state.staged_files.len().to_string(),
            "当前暂存区已准备的文件数",
        ))
        .push(widgets::stat_card(
            "已选文件",
            state.selected_files.len().to_string(),
            "真正会进入本次提交的文件数",
        ))
        .push(widgets::stat_card(
            "变更行",
            format!("+{} / -{}", selected_additions, selected_deletions),
            "用于快速确认本次提交体量",
        ));

    Container::new(
        scrollable::styled(
            Column::new()
                .spacing(theme::spacing::MD)
                .push(tool_tabs)
                .push(widgets::section_header("提交", title, detail))
                .push(overview_cards)
                .push(
                    scrollable::styled_horizontal(
                        Row::new()
                            .spacing(theme::spacing::XS)
                            .push(widgets::info_chip::<CommitDialogMessage>(
                                format!("暂存文件 {}", state.staged_files.len()),
                                BadgeTone::Success,
                            ))
                            .push(widgets::info_chip::<CommitDialogMessage>(
                                format!("已选文件 {}", state.selected_files.len()),
                                BadgeTone::Accent,
                            ))
                            .push(widgets::info_chip::<CommitDialogMessage>(
                                if state.is_amend {
                                    "Amend 模式"
                                } else {
                                    "新提交"
                                },
                                BadgeTone::Neutral,
                            )),
                    )
                    .width(Length::Fill),
                )
                .push_maybe(status_panel)
                .push(
                    Row::new()
                        .spacing(theme::spacing::MD)
                        .push(files_panel)
                        .push(diff_panel),
                )
                .push(message_panel)
                .push(actions),
        )
        .height(Length::Fill),
    )
    .padding([16, 18])
    .width(Length::Fill)
    .height(Length::Fill)
    .style(theme::panel_style(Surface::Panel))
    .into()
}

fn build_status_panel<'a, Message: 'a>(
    label: impl Into<String>,
    detail: impl Into<String>,
    tone: BadgeTone,
) -> Element<'a, Message> {
    widgets::status_banner(label, detail, tone)
}

fn build_file_row<'a>(
    state: &'a CommitDialogState,
    file: &'a Change,
) -> Element<'a, CommitDialogMessage> {
    let path = file.path.clone();
    let is_selected = state.selected_files.contains(&path);
    let is_previewed = state.previewed_file.as_deref() == Some(path.as_str());
    let status = FileStatus::from(&file.status);
    let additions = state
        .file_diff(&path)
        .map(|diff| diff.additions)
        .unwrap_or_default();
    let deletions = state
        .file_diff(&path)
        .map(|diff| diff.deletions)
        .unwrap_or_default();

    let preview_button = Button::new(
        Container::new(
            Row::new()
                .spacing(theme::spacing::XS)
                .align_y(Alignment::Center)
                .push(Text::new(status.symbol()).size(11).color(status.color()))
                .push(
                    Text::new(path.clone())
                        .size(12)
                        .width(Length::Fill)
                        .wrapping(text::Wrapping::WordOrGlyph),
                )
                .push_maybe((additions > 0).then(|| {
                    widgets::info_chip::<CommitDialogMessage>(
                        format!("+{}", additions),
                        BadgeTone::Success,
                    )
                }))
                .push_maybe((deletions > 0).then(|| {
                    widgets::info_chip::<CommitDialogMessage>(
                        format!("-{}", deletions),
                        BadgeTone::Danger,
                    )
                }))
                .push_maybe(is_previewed.then(|| {
                    widgets::info_chip::<CommitDialogMessage>("预览中", BadgeTone::Accent)
                })),
        )
        .padding([5, 8])
        .width(Length::Fill)
        .style(theme::panel_style(if is_previewed {
            Surface::Selection
        } else {
            Surface::Editor
        })),
    )
    .width(Length::Fill)
    .style(theme::button_style(theme::ButtonTone::Ghost))
    .on_press(CommitDialogMessage::PreviewFile(path.clone()));

    Row::new()
        .spacing(theme::spacing::XS)
        .align_y(Alignment::Center)
        .push(
            Checkbox::new(is_selected)
                .size(13)
                .style(theme::checkbox_style())
                .on_toggle(move |checked| CommitDialogMessage::FileToggled(path.clone(), checked)),
        )
        .push(preview_button)
        .into()
}

fn initial_preview_path(selected_files: &[String], diff: &Diff) -> Option<String> {
    selected_files.first().cloned().or_else(|| {
        diff.files
            .iter()
            .find_map(diff_file_path)
            .map(str::to_string)
    })
}

fn diff_file_path(file: &FileDiff) -> Option<&str> {
    file.new_path.as_deref().or(file.old_path.as_deref())
}

fn normalize_editor_text(text: String) -> String {
    text.trim_end_matches('\n').to_string()
}
