//! Git settings view — matches IDEA's Version Control > Git settings panel.

use crate::theme::{self, BadgeTone, Surface};
use crate::widgets::{self, button, scrollable, text_input};
use iced::widget::{Checkbox, Column, Container, Row, Space, Text};
use iced::{Alignment, Element, Length};

/// Settings messages
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    // Update
    SetUpdateMethod(UpdateMethod),
    ToggleAutoUpdateOnPushReject,
    // Push
    SetProtectedBranches(String),
    TogglePreviewPushOnCommit,
    // Commit
    ToggleSignOffCommit,
    ToggleWarnCrlf,
    ToggleWarnDetachedHead,
    ToggleWarnLargeFiles,
    SetLargeFileLimitMb(String),
    ToggleStagingArea,
    // Fetch
    SetFetchTagsMode(FetchTagsMode),
    // Actions
    Close,
    SaveAndClose,
}

/// Update method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateMethod {
    BranchDefault,
    Merge,
    Rebase,
}

impl UpdateMethod {
    pub fn label(&self) -> &'static str {
        match self {
            Self::BranchDefault => "分支默认",
            Self::Merge => "合并",
            Self::Rebase => "变基",
        }
    }
}

/// Fetch tags mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchTagsMode {
    Default,
    AllTags,
    NoTags,
}

impl FetchTagsMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Default => "默认",
            Self::AllTags => "获取所有标签",
            Self::NoTags => "不获取标签",
        }
    }
}

/// Git settings state
#[derive(Debug, Clone)]
pub struct GitSettings {
    pub update_method: UpdateMethod,
    pub auto_update_on_push_reject: bool,
    pub protected_branches: String,
    pub preview_push_on_commit: bool,
    pub sign_off_commit: bool,
    pub warn_crlf: bool,
    pub warn_detached_head: bool,
    pub warn_large_files: bool,
    pub large_file_limit_mb: String,
    pub staging_area_enabled: bool,
    pub fetch_tags_mode: FetchTagsMode,
}

impl Default for GitSettings {
    fn default() -> Self {
        Self {
            update_method: UpdateMethod::Merge,
            auto_update_on_push_reject: false,
            protected_branches: "main, master".to_string(),
            preview_push_on_commit: true,
            sign_off_commit: false,
            warn_crlf: true,
            warn_detached_head: true,
            warn_large_files: true,
            large_file_limit_mb: "50".to_string(),
            staging_area_enabled: false,
            fetch_tags_mode: FetchTagsMode::Default,
        }
    }
}

impl GitSettings {
    pub fn apply_message(&mut self, message: &SettingsMessage) {
        match message {
            SettingsMessage::SetUpdateMethod(method) => self.update_method = *method,
            SettingsMessage::ToggleAutoUpdateOnPushReject => {
                self.auto_update_on_push_reject = !self.auto_update_on_push_reject;
            }
            SettingsMessage::SetProtectedBranches(val) => self.protected_branches = val.clone(),
            SettingsMessage::TogglePreviewPushOnCommit => {
                self.preview_push_on_commit = !self.preview_push_on_commit;
            }
            SettingsMessage::ToggleSignOffCommit => {
                self.sign_off_commit = !self.sign_off_commit;
            }
            SettingsMessage::ToggleWarnCrlf => self.warn_crlf = !self.warn_crlf,
            SettingsMessage::ToggleWarnDetachedHead => {
                self.warn_detached_head = !self.warn_detached_head;
            }
            SettingsMessage::ToggleWarnLargeFiles => {
                self.warn_large_files = !self.warn_large_files;
            }
            SettingsMessage::SetLargeFileLimitMb(val) => self.large_file_limit_mb = val.clone(),
            SettingsMessage::ToggleStagingArea => {
                self.staging_area_enabled = !self.staging_area_enabled;
            }
            SettingsMessage::SetFetchTagsMode(mode) => self.fetch_tags_mode = *mode,
            SettingsMessage::Close | SettingsMessage::SaveAndClose => {}
        }
    }
}

/// Render the settings panel
pub fn view(settings: &GitSettings) -> Element<'_, SettingsMessage> {
    let header = Container::new(
        Row::new()
            .align_y(Alignment::Center)
            .push(Text::new("Git 设置").size(14).color(theme::darcula::TEXT_PRIMARY))
            .push(Space::new().width(Length::Fill))
            .push(button::compact_ghost("关闭", Some(SettingsMessage::Close))),
    )
    .padding([6, 14])
    .width(Length::Fill)
    .style(theme::frame_style(Surface::Toolbar));

    // ── 提交 ──
    let commit_section = settings_section(
        "提交",
        vec![
            checkbox_row(settings.sign_off_commit, "签署提交 (--sign-off)", SettingsMessage::ToggleSignOffCommit),
            checkbox_row(settings.staging_area_enabled, "启用暂存区", SettingsMessage::ToggleStagingArea),
            checkbox_row(settings.warn_crlf, "换行符 CRLF 警告", SettingsMessage::ToggleWarnCrlf),
            checkbox_row(settings.warn_detached_head, "游离 HEAD 警告", SettingsMessage::ToggleWarnDetachedHead),
            checkbox_row(settings.warn_large_files, "大文件警告", SettingsMessage::ToggleWarnLargeFiles),
        ],
    );

    let large_file_row = Container::new(
        Row::new()
            .spacing(8)
            .align_y(Alignment::Center)
            .push(Space::new().width(Length::Fixed(20.0)))
            .push(Text::new("大文件阈值 (MB):").size(12).color(theme::darcula::TEXT_SECONDARY))
            .push(
                Container::new(text_input::styled(
                    "50",
                    &settings.large_file_limit_mb,
                    SettingsMessage::SetLargeFileLimitMb,
                ))
                .width(Length::Fixed(60.0)),
            ),
    )
    .padding([2, 14]);

    // ── 推送 ──
    let push_section = settings_section(
        "推送",
        vec![
            checkbox_row(settings.auto_update_on_push_reject, "推送被拒时自动更新", SettingsMessage::ToggleAutoUpdateOnPushReject),
            checkbox_row(settings.preview_push_on_commit, "提交并推送时预览推送", SettingsMessage::TogglePreviewPushOnCommit),
        ],
    );

    let protected_row = Container::new(
        Row::new()
            .spacing(8)
            .align_y(Alignment::Center)
            .push(Text::new("受保护分支:").size(12).color(theme::darcula::TEXT_SECONDARY))
            .push(
                Container::new(text_input::styled(
                    "main, master",
                    &settings.protected_branches,
                    SettingsMessage::SetProtectedBranches,
                ))
                .width(Length::Fill),
            ),
    )
    .padding([4, 14]);

    // ── 更新 ──
    let update_section = Container::new(
        Column::new()
            .spacing(4)
            .push(
                Text::new("更新")
                    .size(10)
                    .color(theme::darcula::TEXT_DISABLED),
            )
            .push(
                Text::new("更新方式:")
                    .size(12)
                    .color(theme::darcula::TEXT_SECONDARY),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(radio_button(
                        "分支默认",
                        settings.update_method == UpdateMethod::BranchDefault,
                        SettingsMessage::SetUpdateMethod(UpdateMethod::BranchDefault),
                    ))
                    .push(radio_button(
                        "合并",
                        settings.update_method == UpdateMethod::Merge,
                        SettingsMessage::SetUpdateMethod(UpdateMethod::Merge),
                    ))
                    .push(radio_button(
                        "变基",
                        settings.update_method == UpdateMethod::Rebase,
                        SettingsMessage::SetUpdateMethod(UpdateMethod::Rebase),
                    )),
            ),
    )
    .padding([8, 14]);

    // ── 获取 ──
    let fetch_section = Container::new(
        Column::new()
            .spacing(4)
            .push(
                Text::new("获取")
                    .size(10)
                    .color(theme::darcula::TEXT_DISABLED),
            )
            .push(
                Row::new()
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .push(Text::new("标签模式:").size(12).color(theme::darcula::TEXT_SECONDARY))
                    .push(radio_button(
                        "默认",
                        settings.fetch_tags_mode == FetchTagsMode::Default,
                        SettingsMessage::SetFetchTagsMode(FetchTagsMode::Default),
                    ))
                    .push(radio_button(
                        "所有标签",
                        settings.fetch_tags_mode == FetchTagsMode::AllTags,
                        SettingsMessage::SetFetchTagsMode(FetchTagsMode::AllTags),
                    ))
                    .push(radio_button(
                        "不获取",
                        settings.fetch_tags_mode == FetchTagsMode::NoTags,
                        SettingsMessage::SetFetchTagsMode(FetchTagsMode::NoTags),
                    )),
            ),
    )
    .padding([8, 14]);

    // ── Footer ──
    let footer = Container::new(
        Row::new()
            .spacing(8)
            .align_y(Alignment::Center)
            .push(Space::new().width(Length::Fill))
            .push(button::ghost("取消", Some(SettingsMessage::Close)))
            .push(button::primary("保存", Some(SettingsMessage::SaveAndClose))),
    )
    .padding([8, 14])
    .width(Length::Fill)
    .style(theme::frame_style(Surface::Toolbar));

    // ── Assembly ──
    let content = Column::new()
        .spacing(0)
        .width(Length::Fill)
        .push(header)
        .push(iced::widget::rule::horizontal(1))
        .push(commit_section)
        .push(large_file_row)
        .push(iced::widget::rule::horizontal(1))
        .push(push_section)
        .push(protected_row)
        .push(iced::widget::rule::horizontal(1))
        .push(update_section)
        .push(iced::widget::rule::horizontal(1))
        .push(fetch_section)
        .push(Space::new().height(Length::Fill))
        .push(iced::widget::rule::horizontal(1))
        .push(footer);

    Container::new(scrollable::styled(content).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::panel_style(Surface::Panel))
        .into()
}

// ── Helpers ──

fn settings_section<'a>(
    title: &'a str,
    items: Vec<Element<'a, SettingsMessage>>,
) -> Container<'a, SettingsMessage> {
    let mut col = Column::new()
        .spacing(4)
        .push(
            Text::new(title)
                .size(10)
                .color(theme::darcula::TEXT_DISABLED),
        );
    for item in items {
        col = col.push(item);
    }
    Container::new(col).padding([8, 14])
}

fn checkbox_row<'a>(
    checked: bool,
    label: &'a str,
    on_toggle: SettingsMessage,
) -> Element<'a, SettingsMessage> {
    Checkbox::new(checked)
        .label(label)
        .size(14)
        .spacing(6)
        .style(theme::checkbox_style())
        .on_toggle(move |_| on_toggle.clone())
        .into()
}

fn radio_button<'a>(
    label: &'a str,
    selected: bool,
    on_press: SettingsMessage,
) -> Element<'a, SettingsMessage> {
    let icon = if selected { "◉" } else { "○" };
    let color = if selected {
        theme::darcula::ACCENT
    } else {
        theme::darcula::TEXT_SECONDARY
    };

    iced::widget::Button::new(
        Row::new()
            .spacing(4)
            .align_y(Alignment::Center)
            .push(Text::new(icon).size(12).color(color))
            .push(Text::new(label).size(12).color(theme::darcula::TEXT_PRIMARY)),
    )
    .style(theme::button_style(theme::ButtonTone::Ghost))
    .padding([2, 4])
    .on_press(on_press)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_match_idea() {
        let s = GitSettings::default();
        assert_eq!(s.update_method, UpdateMethod::Merge);
        assert!(!s.auto_update_on_push_reject);
        assert!(s.protected_branches.contains("main"));
        assert!(s.protected_branches.contains("master"));
        assert!(!s.sign_off_commit);
        assert!(s.warn_crlf);
        assert!(s.warn_detached_head);
        assert!(s.warn_large_files);
        assert_eq!(s.large_file_limit_mb, "50");
        assert!(!s.staging_area_enabled);
        assert_eq!(s.fetch_tags_mode, FetchTagsMode::Default);
    }

    #[test]
    fn apply_toggle_messages() {
        let mut s = GitSettings::default();
        s.apply_message(&SettingsMessage::ToggleSignOffCommit);
        assert!(s.sign_off_commit);
        s.apply_message(&SettingsMessage::ToggleSignOffCommit);
        assert!(!s.sign_off_commit);
    }

    #[test]
    fn apply_update_method() {
        let mut s = GitSettings::default();
        s.apply_message(&SettingsMessage::SetUpdateMethod(UpdateMethod::Rebase));
        assert_eq!(s.update_method, UpdateMethod::Rebase);
    }

    #[test]
    fn apply_protected_branches() {
        let mut s = GitSettings::default();
        s.apply_message(&SettingsMessage::SetProtectedBranches("main, dev, release/*".to_string()));
        assert!(s.protected_branches.contains("release"));
    }
}
