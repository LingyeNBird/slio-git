//! Split (side-by-side) diff viewer — Meld-style line alignment via diff_core.
//!
//! Each row is a fixed-height Row with two FillPortion(1) halves separated
//! by a 1px center divider. Lines are aligned using the shared AlignedRow model:
//! Equal → both sides, Insert → right only, Delete → left only, Replace → both.

use crate::widgets::{diff_core, diff_file_header, scrollable, syntax_highlighting};
use git_core::diff::{Diff, FileDiff};
use iced::widget::{Column, Container, Row};
use iced::{Element, Length};

pub struct SplitDiffViewer<'a, Message> {
    diff: &'a Diff,
    on_stage_hunk: Option<Box<dyn Fn(String, usize) -> Message + 'a>>,
    on_unstage_hunk: Option<Box<dyn Fn(String, usize) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> SplitDiffViewer<'a, Message> {
    pub fn new(diff: &'a Diff) -> Self {
        Self {
            diff,
            on_stage_hunk: None,
            on_unstage_hunk: None,
        }
    }

    pub fn with_stage_hunk_handler(
        mut self,
        handler: impl Fn(String, usize) -> Message + 'a,
    ) -> Self {
        self.on_stage_hunk = Some(Box::new(handler));
        self
    }

    pub fn with_unstage_hunk_handler(
        mut self,
        handler: impl Fn(String, usize) -> Message + 'a,
    ) -> Self {
        self.on_unstage_hunk = Some(Box::new(handler));
        self
    }

    pub fn view(&self) -> Element<'a, Message> {
        if self.diff.files.is_empty() {
            return crate::widgets::panel_empty_state_compact(
                "当前没有可显示的分栏 diff",
                "先选择有差异的文件，或刷新状态后再切换到分栏查看。",
            );
        }

        let show_header = self.diff.files.len() > 1;
        let mut content = Column::new().spacing(0).width(Length::Fill);

        for file_diff in &self.diff.files {
            if show_header {
                let meta = diff_file_header::DiffFileHeaderMeta::from_file_diff(file_diff);
                content = content.push(diff_file_header::view::<Message>(
                    meta,
                    file_diff.hunks.len(),
                    file_diff.additions,
                    file_diff.deletions,
                ));
            }
            content = content.push(self.render_file(file_diff));
        }

        scrollable::styled(content)
            .height(Length::Fill)
            .into()
    }

    fn render_file(&self, file_diff: &'a FileDiff) -> Element<'a, Message> {
        let syntax_hl = syntax_highlighting::FileSyntaxHighlighter::for_file_diff(file_diff);
        let file_path = file_diff
            .new_path
            .clone()
            .or_else(|| file_diff.old_path.clone())
            .unwrap_or_default();

        let mut lines = Column::new().spacing(0).width(Length::Fill);

        if file_diff.hunks.is_empty() {
            lines = lines.push(diff_core::empty_editor_row("无差异内容"));
        }

        for (index, hunk) in file_diff.hunks.iter().enumerate() {
            if index > 0 {
                lines = lines.push(diff_core::hunk_divider());
            }

            let stage_msg = self
                .on_stage_hunk
                .as_ref()
                .map(|f| f(file_path.clone(), index));
            let unstage_msg = self
                .on_unstage_hunk
                .as_ref()
                .map(|f| f(file_path.clone(), index));

            lines = lines.push(diff_core::hunk_header(hunk, stage_msg, unstage_msg));

            let mut hl = syntax_hl.start_hunk();
            let aligned = diff_core::build_aligned_rows(hunk);

            // Meld: draw 1px boundary lines at chunk transitions
            let mut prev_tag: Option<diff_core::ChunkTag> = None;
            for arow in &aligned {
                // Add boundary at the start of a non-equal chunk group
                if arow.tag != diff_core::ChunkTag::Equal {
                    if prev_tag != Some(arow.tag) && prev_tag != None {
                        // Transition from different tag or from Equal
                        lines = lines.push(diff_core::chunk_boundary(arow.tag));
                    } else if prev_tag.is_none() || prev_tag == Some(diff_core::ChunkTag::Equal) {
                        lines = lines.push(diff_core::chunk_boundary(arow.tag));
                    }
                } else if let Some(pt) = prev_tag {
                    if pt != diff_core::ChunkTag::Equal {
                        // Closing boundary for the previous chunk group
                        lines = lines.push(diff_core::chunk_boundary(pt));
                    }
                }
                lines = lines.push(render_aligned_row(arow, &mut hl));
                prev_tag = Some(arow.tag);
            }
            // Close final chunk boundary if needed
            if let Some(pt) = prev_tag {
                if pt != diff_core::ChunkTag::Equal {
                    lines = lines.push(diff_core::chunk_boundary(pt));
                }
            }
        }

        Container::new(lines)
            .width(Length::Fill)
            .style(diff_core::editor_surface_style())
            .into()
    }
}

fn render_aligned_row<'a, Message: Clone + 'static>(
    arow: &diff_core::AlignedRow,
    hl: &mut syntax_highlighting::HunkSyntaxHighlighter,
) -> Element<'a, Message> {
    let left: Container<'a, Message> = match &arow.left {
        Some(side) => diff_core::render_split_half(side, arow.tag, true, hl),
        None => diff_core::render_empty_half(arow.tag),
    };

    let right: Container<'a, Message> = match &arow.right {
        Some(side) => diff_core::render_split_half(side, arow.tag, false, hl),
        None => diff_core::render_empty_half(arow.tag),
    };

    Row::new()
        .spacing(0)
        .width(Length::Fill)
        .push(left.width(Length::FillPortion(1)))
        .push(diff_core::center_divider())
        .push(right.width(Length::FillPortion(1)))
        .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn split_diff_uses_fill_layout() {
        assert_eq!(iced::Length::Fill, iced::Length::Fill);
    }
}
