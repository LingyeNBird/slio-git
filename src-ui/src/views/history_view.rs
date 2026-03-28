//! History view.
//!
//! Provides a view for browsing commit history.

use crate::theme::{self, BadgeTone, Surface};
use crate::widgets::{self, button, scrollable, text_input, OptionalPush};
use chrono::DateTime;
use git_core::{
    commit::get_commit,
    history::{get_history, search_history, HistoryEntry},
    Repository,
};
use iced::mouse;
use iced::widget::canvas::{self, Canvas};
use iced::widget::{mouse_area, opaque, stack, text, Button, Column, Container, Row, Space, Text};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Theme};
use std::collections::HashSet;

/// Message types for history view.
#[derive(Debug, Clone)]
pub enum HistoryMessage {
    Refresh,
    SelectCommit(String),
    ViewDiff(String),
    SetSearchQuery(String),
    Search,
    ClearSearch,
    TrackContextMenuCursor(Point),
    OpenCommitContextMenu(String),
    CloseCommitContextMenu,
    CopyCommitHash(String),
    ExportCommitPatch(String),
    CompareWithCurrent(String),
    CompareWithWorktree(String),
    PrepareCreateBranch(String),
    PrepareTagFromCommit(String),
    PrepareCherryPickCommit(String),
    PrepareRevertCommit(String),
    PrepareResetCurrentBranchToCommit(String),
    PreparePushCurrentBranchToCommit(String),
    EditCommitMessage(String),
    FixupCommitToPrevious(String),
    SquashCommitToPrevious(String),
    DropCommitFromHistory(String),
    OpenInteractiveRebaseFromCommit(String),
}

/// State for the history view.
#[derive(Debug, Clone)]
pub struct HistoryState {
    pub entries: Vec<HistoryEntry>,
    pub filtered_entries: Vec<HistoryEntry>,
    pub selected_commit: Option<String>,
    pub selected_commit_info: Option<git_core::commit::CommitInfo>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub search_query: String,
    pub is_searching: bool,
    pub context_menu_commit: Option<String>,
    pub context_menu_cursor: Point,
    pub context_menu_anchor: Option<Point>,
    pub current_branch_name: Option<String>,
    pub current_upstream_ref: Option<String>,
    pub current_branch_state_hint: Option<String>,
}

impl HistoryState {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            selected_commit: None,
            selected_commit_info: None,
            is_loading: false,
            error: None,
            search_query: String::new(),
            is_searching: false,
            context_menu_commit: None,
            context_menu_cursor: Point::new(0.0, 0.0),
            context_menu_anchor: None,
            current_branch_name: None,
            current_upstream_ref: None,
            current_branch_state_hint: None,
        }
    }

    fn refresh_repo_context(&mut self, repo: &Repository) {
        self.current_branch_name = repo.current_branch().ok().flatten();
        self.current_upstream_ref = repo.current_upstream_ref();
        self.current_branch_state_hint = repo.state_hint();
    }

    pub fn load_history(&mut self, repo: &Repository) {
        self.is_loading = true;
        self.error = None;
        self.refresh_repo_context(repo);

        match get_history(repo, Some(100)) {
            Ok(entries) => {
                self.entries = entries.clone();
                self.filtered_entries = entries;
                self.is_loading = false;
                self.context_menu_commit = None;
                self.context_menu_anchor = None;
            }
            Err(error) => {
                self.error = Some(format!("加载历史失败: {error}"));
                self.is_loading = false;
            }
        }
    }

    pub fn select_commit(&mut self, repo: &Repository, commit_id: String) {
        self.selected_commit = Some(commit_id.clone());
        self.error = None;
        self.refresh_repo_context(repo);

        match get_commit(repo, &commit_id) {
            Ok(info) => {
                self.selected_commit_info = Some(info);
            }
            Err(error) => {
                self.error = Some(format!("加载提交详情失败: {error}"));
            }
        }
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn track_context_menu_cursor(&mut self, position: Point) {
        self.context_menu_cursor = position;
    }

    pub fn perform_search(&mut self, repo: &Repository) {
        self.context_menu_commit = None;
        self.context_menu_anchor = None;
        self.refresh_repo_context(repo);
        if self.search_query.trim().is_empty() {
            self.filtered_entries = self.entries.clone();
            self.error = None;
            return;
        }

        self.is_searching = true;
        self.error = None;
        match search_history(repo, &self.search_query, Some(100)) {
            Ok(entries) => {
                self.filtered_entries = entries;
                self.is_searching = false;
            }
            Err(error) => {
                self.error = Some(format!("搜索失败: {error}"));
                self.is_searching = false;
            }
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query = String::new();
        self.filtered_entries = self.entries.clone();
        self.error = None;
        self.is_searching = false;
        self.context_menu_commit = None;
        self.context_menu_anchor = None;
    }
}

impl Default for HistoryState {
    fn default() -> Self {
        Self::new()
    }
}

fn format_timestamp(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

const HISTORY_ROW_HEIGHT: f32 = 26.0;
const HISTORY_CONTEXT_MENU_WIDTH: f32 = 392.0;
const HISTORY_CONTEXT_MENU_ESTIMATED_HEIGHT: f32 = 420.0;
const HISTORY_CONTEXT_MENU_EDGE_PADDING: f32 = 8.0;
const HISTORY_GRAPH_LANE_WIDTH: f32 = 16.0;
const HISTORY_GRAPH_PADDING: f32 = 8.0;
const HISTORY_GRAPH_MIN_WIDTH: f32 = 56.0;
const HISTORY_GRAPH_LINE_WIDTH: f32 = 1.6;
const HISTORY_GRAPH_NODE_RADIUS: f32 = 4.0;

#[derive(Debug, Clone)]
struct LaneState {
    commit_id: String,
    color_index: usize,
}

#[derive(Debug, Clone)]
struct GraphLane {
    lane: usize,
    color_index: usize,
}

#[derive(Debug, Clone)]
struct GraphTransition {
    from_lane: usize,
    to_lane: usize,
    color_index: usize,
}

#[derive(Debug, Clone)]
struct HistoryGraphRow {
    top_lanes: Vec<GraphLane>,
    continuing: Vec<GraphTransition>,
    parent_transitions: Vec<GraphTransition>,
    node_lane: usize,
    node_color_index: usize,
    total_lanes: usize,
}

#[derive(Debug, Clone)]
struct HistoryGraphLayout {
    rows: Vec<HistoryGraphRow>,
    lane_count: usize,
}

#[derive(Debug, Clone)]
struct HistoryGraphCanvas {
    row: HistoryGraphRow,
    is_selected: bool,
}

impl<Message> canvas::Program<Message> for HistoryGraphCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center_y = bounds.height / 2.0;
        let bottom_y = bounds.height;

        for lane in &self.row.top_lanes {
            stroke_segment(
                &mut frame,
                lane_center_x(lane.lane),
                0.0,
                lane_center_x(lane.lane),
                center_y,
                history_graph_color(lane.color_index),
            );
        }

        for transition in &self.row.continuing {
            stroke_segment(
                &mut frame,
                lane_center_x(transition.from_lane),
                center_y,
                lane_center_x(transition.to_lane),
                bottom_y,
                history_graph_color(transition.color_index),
            );
        }

        for transition in &self.row.parent_transitions {
            stroke_segment(
                &mut frame,
                lane_center_x(transition.from_lane),
                center_y,
                lane_center_x(transition.to_lane),
                bottom_y,
                history_graph_color(transition.color_index),
            );
        }

        if self.is_selected {
            let halo = canvas::Path::circle(
                Point::new(lane_center_x(self.row.node_lane), center_y),
                HISTORY_GRAPH_NODE_RADIUS + 2.5,
            );
            frame.fill(&halo, theme::darcula::SELECTION_BG.scale_alpha(0.55));
        }

        let node = canvas::Path::circle(
            Point::new(lane_center_x(self.row.node_lane), center_y),
            HISTORY_GRAPH_NODE_RADIUS,
        );
        let node_color = history_graph_color(self.row.node_color_index);
        frame.fill(&node, node_color);
        frame.stroke(
            &node,
            canvas::Stroke::default()
                .with_color(theme::darcula::BG_EDITOR)
                .with_width(1.2),
        );

        vec![frame.into_geometry()]
    }
}

fn build_history_graph(entries: &[HistoryEntry]) -> HistoryGraphLayout {
    let mut active_lanes: Vec<LaneState> = Vec::new();
    let mut rows = Vec::with_capacity(entries.len());
    let mut next_color_index = 0usize;
    let mut max_lane_count = 1usize;

    for entry in entries {
        let incoming = active_lanes.clone();

        let (working_lanes, node_lane, node_color_index) =
            if let Some(position) = incoming.iter().position(|lane| lane.commit_id == entry.id) {
                (incoming.clone(), position, incoming[position].color_index)
            } else {
                let mut lanes = incoming.clone();
                let color_index = next_color_index;
                next_color_index += 1;
                lanes.push(LaneState {
                    commit_id: entry.id.clone(),
                    color_index,
                });
                let node_lane = lanes.len() - 1;
                (lanes, node_lane, color_index)
            };

        let mut after = working_lanes.clone();

        if entry.parent_ids.is_empty() {
            after.remove(node_lane);
        } else {
            let first_parent = &entry.parent_ids[0];
            let existing_first_parent = after
                .iter()
                .enumerate()
                .find(|(index, lane)| *index != node_lane && lane.commit_id == *first_parent)
                .map(|(index, _)| index);

            if existing_first_parent.is_some() {
                after.remove(node_lane);
            } else if let Some(current_lane) = after.get_mut(node_lane) {
                current_lane.commit_id = first_parent.clone();
            }

            let mut insertion_index = (node_lane + 1).min(after.len());
            for parent in entry.parent_ids.iter().skip(1) {
                if after.iter().any(|lane| lane.commit_id == *parent) {
                    continue;
                }

                let color_index = next_color_index;
                next_color_index += 1;
                after.insert(
                    insertion_index,
                    LaneState {
                        commit_id: parent.clone(),
                        color_index,
                    },
                );
                insertion_index += 1;
            }
        }

        let top_lanes = incoming
            .iter()
            .enumerate()
            .map(|(lane, state)| GraphLane {
                lane,
                color_index: state.color_index,
            })
            .collect::<Vec<_>>();

        let continuing = incoming
            .iter()
            .enumerate()
            .filter_map(|(from_lane, lane)| {
                if lane.commit_id == entry.id {
                    return None;
                }

                after
                    .iter()
                    .position(|next_lane| next_lane.commit_id == lane.commit_id)
                    .map(|to_lane| GraphTransition {
                        from_lane,
                        to_lane,
                        color_index: lane.color_index,
                    })
            })
            .collect::<Vec<_>>();

        let mut seen_parent_targets = HashSet::new();
        let parent_transitions = entry
            .parent_ids
            .iter()
            .filter_map(|parent| {
                let target_lane = after.iter().position(|lane| lane.commit_id == *parent)?;

                if !seen_parent_targets.insert(target_lane) {
                    return None;
                }

                Some(GraphTransition {
                    from_lane: node_lane,
                    to_lane: target_lane,
                    color_index: after[target_lane].color_index,
                })
            })
            .collect::<Vec<_>>();

        max_lane_count = max_lane_count
            .max(incoming.len())
            .max(after.len())
            .max(node_lane + 1);

        rows.push(HistoryGraphRow {
            top_lanes,
            continuing,
            parent_transitions,
            node_lane,
            node_color_index,
            total_lanes: 1,
        });

        active_lanes = after;
    }

    for row in &mut rows {
        row.total_lanes = max_lane_count.max(1);
    }

    HistoryGraphLayout {
        rows,
        lane_count: max_lane_count.max(1),
    }
}

fn build_commit_row<'a>(
    entry: &'a HistoryEntry,
    graph_row: &HistoryGraphRow,
    graph_width: f32,
    is_selected: bool,
    is_menu_open: bool,
) -> Element<'a, HistoryMessage> {
    let subject = commit_subject(&entry.message);
    let metadata = format!("{} · {}", short_commit_id(&entry.id), entry.author_name);

    let graph = Canvas::new(HistoryGraphCanvas {
        row: graph_row.clone(),
        is_selected,
    })
    .width(Length::Fixed(graph_width))
    .height(Length::Fixed(HISTORY_ROW_HEIGHT));

    let row = Container::new(
        Row::new()
            .align_y(Alignment::Center)
            .push(graph)
            .push(
                Text::new(subject)
                    .size(13)
                    .width(Length::FillPortion(5))
                    .wrapping(text::Wrapping::WordOrGlyph),
            )
            .push(
                Text::new(metadata)
                    .size(11)
                    .width(Length::FillPortion(3))
                    .wrapping(text::Wrapping::WordOrGlyph)
                    .color(theme::darcula::TEXT_SECONDARY),
            )
            .push(
                Text::new(format_timestamp(entry.timestamp))
                    .size(11)
                    .width(Length::FillPortion(2))
                    .wrapping(text::Wrapping::None)
                    .color(theme::darcula::TEXT_SECONDARY),
            ),
    )
    .padding([8, 10])
    .style(theme::panel_style(if is_menu_open {
        Surface::Selection
    } else if is_selected {
        Surface::Selection
    } else {
        Surface::Editor
    }));

    mouse_area(
        Container::new(
            Button::new(row)
                .width(Length::Fill)
                .style(theme::button_style(theme::ButtonTone::Ghost))
                .on_press(HistoryMessage::SelectCommit(entry.id.clone())),
        )
        .width(Length::Fill),
    )
    .on_right_press(HistoryMessage::OpenCommitContextMenu(entry.id.clone()))
    .interaction(mouse::Interaction::Pointer)
    .into()
}

fn build_history_list<'a>(state: &'a HistoryState) -> Element<'a, HistoryMessage> {
    let entries = &state.filtered_entries;
    let graph = build_history_graph(entries);
    let graph_width = (graph.lane_count as f32 * HISTORY_GRAPH_LANE_WIDTH
        + HISTORY_GRAPH_PADDING * 2.0)
        .max(HISTORY_GRAPH_MIN_WIDTH);

    let list = if entries.is_empty() {
        Column::new().push(
            Text::new("当前没有可显示的提交历史。")
                .size(12)
                .color(theme::darcula::TEXT_SECONDARY),
        )
    } else {
        entries.iter().zip(graph.rows.iter()).fold(
            Column::new().spacing(1),
            |column, (entry, graph_row)| {
                let is_selected = state
                    .selected_commit
                    .as_deref()
                    .map(|value| value == entry.id)
                    .unwrap_or(false);
                let is_menu_open = state
                    .context_menu_commit
                    .as_deref()
                    .map(|value| value == entry.id)
                    .unwrap_or(false);
                column.push(build_commit_row(
                    entry,
                    graph_row,
                    graph_width,
                    is_selected,
                    is_menu_open,
                ))
            },
        )
    };

    mouse_area(
        Container::new(
            Column::new()
                .spacing(theme::spacing::SM)
                .push(
                    Row::new()
                        .spacing(theme::spacing::XS)
                        .align_y(Alignment::Center)
                        .push(
                            Text::new("提交列表")
                                .size(13)
                                .color(theme::darcula::TEXT_SECONDARY),
                        )
                        .push(widgets::info_chip::<HistoryMessage>(
                            entries.len().to_string(),
                            BadgeTone::Neutral,
                        )),
                )
                .push(
                    Container::new(
                        Row::new()
                            .align_y(Alignment::Center)
                            .push(
                                Text::new("图谱")
                                    .size(10)
                                    .width(Length::Fixed(graph_width))
                                    .color(theme::darcula::TEXT_DISABLED),
                            )
                            .push(
                                Text::new("提交")
                                    .size(10)
                                    .width(Length::FillPortion(5))
                                    .color(theme::darcula::TEXT_DISABLED),
                            )
                            .push(
                                Text::new("作者 / 哈希")
                                    .size(10)
                                    .width(Length::FillPortion(3))
                                    .color(theme::darcula::TEXT_DISABLED),
                            )
                            .push(
                                Text::new("时间")
                                    .size(10)
                                    .width(Length::FillPortion(2))
                                    .color(theme::darcula::TEXT_DISABLED),
                            ),
                    )
                    .padding([8, 10])
                    .style(theme::panel_style(Surface::Raised)),
                )
                .push(scrollable::styled(list).height(Length::Fixed(320.0))),
        )
        .padding([16, 16])
        .style(theme::panel_style(Surface::Panel)),
    )
    .on_move(HistoryMessage::TrackContextMenuCursor)
    .into()
}

fn build_commit_context_menu_overlay<'a>(state: &'a HistoryState) -> Element<'a, HistoryMessage> {
    let Some(commit_id) = state.context_menu_commit.as_deref() else {
        return Space::new().width(Length::Shrink).into();
    };
    let anchor = state
        .context_menu_anchor
        .unwrap_or(state.context_menu_cursor);
    let Some(entry) = state
        .filtered_entries
        .iter()
        .find(|entry| entry.id == commit_id)
        .or_else(|| state.entries.iter().find(|entry| entry.id == commit_id))
    else {
        return Space::new().width(Length::Shrink).into();
    };

    let selected_info = state
        .selected_commit
        .as_deref()
        .filter(|selected| *selected == entry.id)
        .and_then(|_| state.selected_commit_info.as_ref());
    let has_current_branch = state.current_branch_name.is_some();
    let has_upstream = state.current_upstream_ref.is_some();
    let commit_detail_ready = selected_info.is_some();
    let is_merge_commit = selected_info.is_some_and(|info| info.parent_ids.len() > 1);
    let is_root_commit = selected_info.is_some_and(|info| info.parent_ids.is_empty());

    let compare_with_current_detail = if let Some(branch_name) = state.current_branch_name.as_ref()
    {
        format!("直接比较这条提交和当前分支 {branch_name} 的差异")
    } else {
        "当前为 detached HEAD，不能直接与当前分支比较".to_string()
    };
    let compare_with_worktree_detail = if let Some(branch_name) = state.current_branch_name.as_ref()
    {
        format!("把这条提交和当前工作树直接做比较，基于 {branch_name} 继续判断")
    } else {
        "当前为 detached HEAD，仍可查看工作树差异，但不能继续围绕当前分支操作".to_string()
    };
    let cherry_pick_detail = if !commit_detail_ready {
        "提交详情还没加载完成，请稍后再试".to_string()
    } else if is_merge_commit {
        "merge 提交暂不支持直接 Cherry-pick".to_string()
    } else if let Some(branch_name) = state.current_branch_name.as_ref() {
        format!("把这条提交复制应用到当前分支 {branch_name}")
    } else {
        "当前为 detached HEAD，不能直接摘取到当前分支".to_string()
    };
    let revert_detail = if !commit_detail_ready {
        "提交详情还没加载完成，请稍后再试".to_string()
    } else if is_merge_commit {
        "merge 提交暂不支持直接回退".to_string()
    } else if let Some(branch_name) = state.current_branch_name.as_ref() {
        format!("在当前分支 {branch_name} 上生成一条新的反向提交")
    } else {
        "当前为 detached HEAD，不能直接回退提交".to_string()
    };
    let reset_detail = if let Some(branch_name) = state.current_branch_name.as_ref() {
        format!("把当前分支 {branch_name} 重置到这条提交；若不是祖先提交，确认时会阻止")
    } else {
        "当前为 detached HEAD，无法重置当前分支".to_string()
    };
    let push_to_here_detail = if let Some(upstream_ref) = state.current_upstream_ref.as_ref() {
        format!("只访问当前分支的上游 {upstream_ref}，把远端推进到这条提交")
    } else if has_current_branch {
        "当前分支还没有上游，暂时不能推送到这里".to_string()
    } else {
        "当前为 detached HEAD，无法执行“推送到这里”".to_string()
    };
    let edit_message_detail = if !commit_detail_ready {
        "提交详情还没加载完成，请稍后再试".to_string()
    } else if is_merge_commit {
        "merge 提交暂不支持直接改说明".to_string()
    } else if let Some(branch_name) = state.current_branch_name.as_ref() {
        format!("围绕当前分支 {branch_name} 启动改说明流程，并在停住后进入 amend 面板")
    } else {
        "当前为 detached HEAD，不能直接改写当前分支历史".to_string()
    };
    let fixup_detail = if !commit_detail_ready {
        "提交详情还没加载完成，请稍后再试".to_string()
    } else if is_merge_commit {
        "merge 提交暂不支持直接 Fixup".to_string()
    } else if is_root_commit {
        "根提交前面没有可合并的目标提交".to_string()
    } else {
        "把这条提交压进前一条提交，并尽量自动完成后续整理".to_string()
    };
    let squash_detail = if !commit_detail_ready {
        "提交详情还没加载完成，请稍后再试".to_string()
    } else if is_merge_commit {
        "merge 提交暂不支持直接压缩".to_string()
    } else if is_root_commit {
        "根提交前面没有可合并的目标提交".to_string()
    } else {
        "把这条提交压缩到前一条提交，并保留自动生成的合并说明".to_string()
    };
    let drop_detail = if !commit_detail_ready {
        "提交详情还没加载完成，请稍后再试".to_string()
    } else if is_merge_commit {
        "merge 提交暂不支持直接删除".to_string()
    } else {
        "从当前分支历史里移除这条提交；若后续提交依赖它，可能进入冲突处理".to_string()
    };
    let interactive_rebase_detail = if has_current_branch {
        "打开 Rebase 面板并保留这条提交的整理上下文，后续 todo 编辑会继续接到这里".to_string()
    } else {
        "当前为 detached HEAD，不能直接围绕当前分支开始交互式变基".to_string()
    };

    let actions = Column::new()
        .spacing(theme::spacing::XS)
        .push(history_context_group(
            "常用",
            "复制、导出和快速查看。",
            vec![
                history_context_action_row(
                    "复制哈希",
                    "把完整提交哈希复制到系统剪贴板".to_string(),
                    Some(HistoryMessage::CopyCommitHash(entry.id.clone())),
                    HistoryMenuTone::Neutral,
                ),
                history_context_action_row(
                    "导出 Patch",
                    "用 git format-patch 导出这条提交的补丁文件".to_string(),
                    Some(HistoryMessage::ExportCommitPatch(entry.id.clone())),
                    HistoryMenuTone::Neutral,
                ),
                history_context_action_row(
                    "查看详情",
                    "加载完整提交信息并保持当前列表上下文".to_string(),
                    Some(HistoryMessage::SelectCommit(entry.id.clone())),
                    HistoryMenuTone::Neutral,
                ),
            ],
        ))
        .push(history_context_group(
            "比较与派生",
            "保留历史不动，围绕这条提交继续判断。",
            vec![
                history_context_action_row(
                    "与当前分支比较",
                    compare_with_current_detail,
                    has_current_branch
                        .then_some(HistoryMessage::CompareWithCurrent(entry.id.clone())),
                    HistoryMenuTone::Accent,
                ),
                history_context_action_row(
                    "查看与工作树差异",
                    compare_with_worktree_detail,
                    (!state.is_loading)
                        .then_some(HistoryMessage::CompareWithWorktree(entry.id.clone())),
                    HistoryMenuTone::Accent,
                ),
                history_context_action_row(
                    "从该提交建分支",
                    "保留当前分支不动，基于这条提交创建新分支".to_string(),
                    (!state.is_loading)
                        .then_some(HistoryMessage::PrepareCreateBranch(entry.id.clone())),
                    HistoryMenuTone::Neutral,
                ),
                history_context_action_row(
                    "给该提交打标签",
                    "在这条提交上创建一个新的标签".to_string(),
                    (!state.is_loading)
                        .then_some(HistoryMessage::PrepareTagFromCommit(entry.id.clone())),
                    HistoryMenuTone::Neutral,
                ),
            ],
        ))
        .push(history_context_group(
            "应用到当前分支",
            "会在当前分支生成新的提交。",
            vec![
                history_context_action_row(
                    "Cherry-pick",
                    cherry_pick_detail,
                    (!state.is_loading
                        && commit_detail_ready
                        && !is_merge_commit
                        && has_current_branch)
                        .then_some(HistoryMessage::PrepareCherryPickCommit(entry.id.clone())),
                    HistoryMenuTone::Accent,
                ),
                history_context_action_row(
                    "Revert",
                    revert_detail,
                    (!state.is_loading
                        && commit_detail_ready
                        && !is_merge_commit
                        && has_current_branch)
                        .then_some(HistoryMessage::PrepareRevertCommit(entry.id.clone())),
                    HistoryMenuTone::Accent,
                ),
            ],
        ))
        .push(history_context_group(
            "危险动作",
            "会移动当前分支指针或直接发布到当前上游。",
            vec![
                history_context_action_row(
                    "重置当前分支到这里",
                    reset_detail,
                    (!state.is_loading && has_current_branch).then_some(
                        HistoryMessage::PrepareResetCurrentBranchToCommit(entry.id.clone()),
                    ),
                    HistoryMenuTone::Danger,
                ),
                history_context_action_row(
                    "推送当前分支到这里",
                    push_to_here_detail,
                    (!state.is_loading && has_current_branch && has_upstream).then_some(
                        HistoryMessage::PreparePushCurrentBranchToCommit(entry.id.clone()),
                    ),
                    HistoryMenuTone::Danger,
                ),
            ],
        ))
        .push(history_context_group(
            "历史整理",
            "围绕本地提交改说明、压缩或删除历史。",
            vec![
                history_context_action_row(
                    "编辑提交消息...",
                    edit_message_detail,
                    (!state.is_loading
                        && commit_detail_ready
                        && !is_merge_commit
                        && has_current_branch)
                        .then_some(HistoryMessage::EditCommitMessage(entry.id.clone())),
                    HistoryMenuTone::Neutral,
                ),
                history_context_action_row(
                    "Fixup...",
                    fixup_detail,
                    (!state.is_loading
                        && commit_detail_ready
                        && !is_merge_commit
                        && !is_root_commit
                        && has_current_branch)
                        .then_some(HistoryMessage::FixupCommitToPrevious(entry.id.clone())),
                    HistoryMenuTone::Accent,
                ),
                history_context_action_row(
                    "压缩到...",
                    squash_detail,
                    (!state.is_loading
                        && commit_detail_ready
                        && !is_merge_commit
                        && !is_root_commit
                        && has_current_branch)
                        .then_some(HistoryMessage::SquashCommitToPrevious(entry.id.clone())),
                    HistoryMenuTone::Accent,
                ),
                history_context_action_row(
                    "删除提交",
                    drop_detail,
                    (!state.is_loading
                        && commit_detail_ready
                        && !is_merge_commit
                        && has_current_branch)
                        .then_some(HistoryMessage::DropCommitFromHistory(entry.id.clone())),
                    HistoryMenuTone::Danger,
                ),
                history_context_action_row(
                    "从这里进行交互式变基...",
                    interactive_rebase_detail,
                    (!state.is_loading && has_current_branch).then_some(
                        HistoryMessage::OpenInteractiveRebaseFromCommit(entry.id.clone()),
                    ),
                    HistoryMenuTone::Neutral,
                ),
            ],
        ));

    let menu = Container::new(
        Column::new()
            .spacing(theme::spacing::MD)
            .push(
                Row::new()
                    .spacing(theme::spacing::XS)
                    .align_y(Alignment::Center)
                    .push(
                        Text::new(format!("提交动作 · {}", short_commit_id(&entry.id)))
                            .size(12)
                            .width(Length::Fill)
                            .wrapping(text::Wrapping::WordOrGlyph),
                    )
                    .push(button::compact_ghost(
                        "关闭",
                        Some(HistoryMessage::CloseCommitContextMenu),
                    )),
            )
            .push(
                Text::new(commit_subject(&entry.message))
                    .size(11)
                    .width(Length::Fill)
                    .wrapping(text::Wrapping::WordOrGlyph)
                    .color(theme::darcula::TEXT_SECONDARY),
            )
            .push(
                Row::new()
                    .spacing(theme::spacing::XS)
                    .push_maybe(state.current_branch_name.as_ref().map(|branch_name| {
                        widgets::info_chip::<HistoryMessage>(
                            format!("当前 {branch_name}"),
                            BadgeTone::Success,
                        )
                    }))
                    .push_maybe(state.current_upstream_ref.as_ref().map(|upstream_ref| {
                        widgets::info_chip::<HistoryMessage>(
                            format!("上游 {upstream_ref}"),
                            BadgeTone::Neutral,
                        )
                    }))
                    .push_maybe(state.current_branch_name.is_none().then(|| {
                        widgets::info_chip::<HistoryMessage>("detached HEAD", BadgeTone::Warning)
                    })),
            )
            .push_maybe(state.current_branch_state_hint.as_ref().map(|hint| {
                Text::new(hint)
                    .size(10)
                    .width(Length::Fill)
                    .wrapping(text::Wrapping::WordOrGlyph)
                    .color(theme::darcula::TEXT_SECONDARY)
            }))
            .push(Container::new(
                scrollable::styled(actions).height(Length::Fixed(320.0)),
            )),
    )
    .padding([16, 18])
    .width(Length::Fixed(HISTORY_CONTEXT_MENU_WIDTH))
    .style(history_context_menu_style);

    build_history_context_menu_layer(anchor, menu.into())
}

#[derive(Debug, Clone, Copy)]
enum HistoryMenuTone {
    Neutral,
    Accent,
    Danger,
}

fn history_context_group<'a>(
    title: &'static str,
    detail: &'static str,
    rows: Vec<Element<'a, HistoryMessage>>,
) -> Element<'a, HistoryMessage> {
    let rows = rows
        .into_iter()
        .fold(Column::new().spacing(0), |column, row| column.push(row));

    let (background, border_color) = match title {
        "危险动作" => (
            blend_color(theme::darcula::BG_PANEL, theme::darcula::DANGER, 0.10),
            theme::darcula::DANGER.scale_alpha(0.24),
        ),
        "应用到当前分支" | "比较与派生" | "修改当前分支历史" => (
            blend_color(theme::darcula::BG_PANEL, theme::darcula::ACCENT_WEAK, 0.66),
            theme::darcula::ACCENT.scale_alpha(0.20),
        ),
        _ => (
            blend_color(theme::darcula::BG_PANEL, theme::darcula::BG_RAISED, 0.76),
            theme::darcula::BORDER.scale_alpha(0.74),
        ),
    };

    Container::new(
        Column::new()
            .spacing(theme::spacing::SM)
            .push(
                Text::new(title)
                    .size(11)
                    .color(theme::darcula::TEXT_DISABLED),
            )
            .push(
                Text::new(detail)
                    .size(10)
                    .width(Length::Fill)
                    .wrapping(text::Wrapping::WordOrGlyph)
                    .color(theme::darcula::TEXT_SECONDARY),
            )
            .push(rows),
    )
    .padding([9, 11])
    .style(move |_theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(background)),
        border: iced::Border {
            width: 1.0,
            color: border_color,
            radius: theme::radius::LG.into(),
        },
        ..Default::default()
    })
    .into()
}

fn history_context_action_row<'a>(
    title: &'static str,
    detail: String,
    message: Option<HistoryMessage>,
    tone: HistoryMenuTone,
) -> Element<'a, HistoryMessage> {
    let enabled = message.is_some();
    let title_color = if enabled {
        match tone {
            HistoryMenuTone::Neutral => theme::darcula::TEXT_PRIMARY,
            HistoryMenuTone::Accent => theme::darcula::TEXT_PRIMARY,
            HistoryMenuTone::Danger => theme::darcula::DANGER.scale_alpha(0.95),
        }
    } else {
        theme::darcula::TEXT_DISABLED
    };
    let detail_color = if enabled {
        match tone {
            HistoryMenuTone::Danger => theme::darcula::DANGER.scale_alpha(0.7),
            _ => theme::darcula::TEXT_SECONDARY,
        }
    } else {
        theme::darcula::TEXT_DISABLED
    };

    Button::new(
        Container::new(
            Row::new()
                .spacing(theme::spacing::SM)
                .align_y(Alignment::Center)
                .push(
                    Column::new()
                        .spacing(1)
                        .width(Length::Fill)
                        .push(Text::new(title).size(12).color(title_color))
                        .push(
                            Text::new(detail)
                                .size(10)
                                .width(Length::Fill)
                                .wrapping(text::Wrapping::WordOrGlyph)
                                .color(detail_color),
                        ),
                )
                .push(
                    Text::new(if enabled { ">" } else { "·" })
                        .size(11)
                        .color(detail_color),
                ),
        )
        .padding([13, 10])
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .style(history_context_action_button_style(tone, enabled))
    .on_press_maybe(message)
    .into()
}

fn build_history_context_menu_layer<'a>(
    anchor: Point,
    menu: Element<'a, HistoryMessage>,
) -> Element<'a, HistoryMessage> {
    let origin = history_context_menu_origin(anchor);

    opaque(
        mouse_area(
            Container::new(
                Column::new()
                    .push(Space::new().height(Length::Fixed(origin.y)))
                    .push(
                        Row::new()
                            .width(Length::Fill)
                            .push(Space::new().width(Length::Fixed(origin.x)))
                            .push(menu)
                            .push(Space::new().width(Length::Fill)),
                    )
                    .push(Space::new().height(Length::Fill)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(history_context_menu_scrim_style),
        )
        .on_press(HistoryMessage::CloseCommitContextMenu),
    )
    .into()
}

fn history_context_menu_origin(anchor: Point) -> Point {
    let x = if anchor.x > HISTORY_CONTEXT_MENU_WIDTH * 0.68 {
        (anchor.x - HISTORY_CONTEXT_MENU_WIDTH + 28.0).max(HISTORY_CONTEXT_MENU_EDGE_PADDING)
    } else {
        (anchor.x + 6.0).max(HISTORY_CONTEXT_MENU_EDGE_PADDING)
    };
    let y = if anchor.y > HISTORY_CONTEXT_MENU_ESTIMATED_HEIGHT * 0.52 {
        (anchor.y - HISTORY_CONTEXT_MENU_ESTIMATED_HEIGHT + 18.0)
            .max(HISTORY_CONTEXT_MENU_EDGE_PADDING)
    } else {
        (anchor.y + 6.0).max(HISTORY_CONTEXT_MENU_EDGE_PADDING)
    };

    Point::new(x, y)
}

fn history_context_menu_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(blend_color(
            theme::darcula::BG_PANEL,
            theme::darcula::BG_RAISED,
            0.92,
        ))),
        border: iced::Border {
            width: 1.0,
            color: theme::darcula::ACCENT.scale_alpha(0.24),
            radius: theme::radius::LG.into(),
        },
        shadow: iced::Shadow {
            color: Color {
                a: 0.20,
                ..theme::darcula::BG_MAIN
            },
            offset: iced::Vector::new(0.0, 16.0),
            blur_radius: 32.0,
        },
        ..Default::default()
    }
}

fn history_context_action_button_style(
    tone: HistoryMenuTone,
    enabled: bool,
) -> impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style {
    move |_theme, status| {
        let interaction_color = match tone {
            HistoryMenuTone::Neutral => theme::darcula::BG_TAB_HOVER,
            HistoryMenuTone::Accent => theme::darcula::ACCENT,
            HistoryMenuTone::Danger => theme::darcula::DANGER,
        };

        let (background, border_color) = if enabled {
            match status {
                iced::widget::button::Status::Active => (
                    blend_color(theme::darcula::BG_PANEL, interaction_color, 0.06),
                    interaction_color.scale_alpha(0.14),
                ),
                iced::widget::button::Status::Hovered => (
                    blend_color(theme::darcula::BG_PANEL, interaction_color, 0.18),
                    interaction_color.scale_alpha(0.28),
                ),
                iced::widget::button::Status::Pressed => (
                    blend_color(theme::darcula::BG_PANEL, interaction_color, 0.26),
                    interaction_color.scale_alpha(0.38),
                ),
                iced::widget::button::Status::Disabled => (
                    blend_color(theme::darcula::BG_PANEL, theme::darcula::BG_MAIN, 0.20),
                    theme::darcula::SEPARATOR.scale_alpha(0.28),
                ),
            }
        } else {
            (
                blend_color(theme::darcula::BG_PANEL, theme::darcula::BG_MAIN, 0.18),
                theme::darcula::SEPARATOR.scale_alpha(0.22),
            )
        };

        iced::widget::button::Style {
            background: Some(iced::Background::Color(background)),
            border: iced::Border {
                width: 1.0,
                color: border_color,
                radius: theme::radius::LG.into(),
            },
            text_color: if enabled {
                theme::darcula::TEXT_PRIMARY
            } else {
                theme::darcula::TEXT_DISABLED
            },
            ..Default::default()
        }
    }
}

fn history_context_menu_scrim_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(Color {
            a: 0.08,
            ..theme::darcula::BG_EDITOR
        })),
        border: iced::Border::default(),
        ..Default::default()
    }
}

fn blend_color(base: Color, overlay: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    Color {
        r: (base.r * (1.0 - amount)) + (overlay.r * amount),
        g: (base.g * (1.0 - amount)) + (overlay.g * amount),
        b: (base.b * (1.0 - amount)) + (overlay.b * amount),
        a: (base.a * (1.0 - amount)) + (overlay.a * amount),
    }
}

fn commit_subject(message: &str) -> &str {
    message
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or(message)
}

fn short_commit_id(id: &str) -> &str {
    &id[..id.len().min(8)]
}

fn lane_center_x(lane: usize) -> f32 {
    HISTORY_GRAPH_PADDING + lane as f32 * HISTORY_GRAPH_LANE_WIDTH + HISTORY_GRAPH_LANE_WIDTH / 2.0
}

fn stroke_segment(
    frame: &mut canvas::Frame<Renderer>,
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
    color: Color,
) {
    let path = canvas::Path::line(Point::new(from_x, from_y), Point::new(to_x, to_y));
    frame.stroke(
        &path,
        canvas::Stroke::default()
            .with_color(color)
            .with_width(HISTORY_GRAPH_LINE_WIDTH)
            .with_line_cap(canvas::LineCap::Round)
            .with_line_join(canvas::LineJoin::Round),
    );
}

/// Branch-lane palette tuned for the dark mint-accent theme.
/// Colors are vivid enough to distinguish lanes but harmonize with the mint/teal brand.
fn history_graph_color(index: usize) -> Color {
    match index % 8 {
        0 => Color::from_rgb(0.224, 0.816, 0.769), // accent mint
        1 => Color::from_rgb(0.345, 0.651, 1.000), // brand blue
        2 => Color::from_rgb(0.247, 0.718, 0.314), // success green
        3 => Color::from_rgb(0.627, 0.431, 0.863), // violet
        4 => Color::from_rgb(0.235, 0.757, 0.851), // cyan
        5 => Color::from_rgb(0.824, 0.600, 0.133), // warning gold
        6 => Color::from_rgb(0.863, 0.431, 0.478), // rose
        _ => Color::from_rgb(0.760, 0.549, 0.302), // warm amber
    }
}

fn build_search_bar(state: &HistoryState) -> Element<'_, HistoryMessage> {
    let can_search = !state.is_searching && !state.search_query.trim().is_empty();
    let can_clear = !state.is_searching
        && (!state.search_query.trim().is_empty()
            || state.filtered_entries.len() != state.entries.len());

    Container::new(
        Column::new()
            .spacing(theme::spacing::SM)
            .push(text_input::styled(
                "输入搜索关键词...",
                &state.search_query,
                HistoryMessage::SetSearchQuery,
            ))
            .push(
                scrollable::styled_horizontal(
                    Row::new()
                        .spacing(theme::spacing::XS)
                        .push_maybe(state.current_branch_name.as_ref().map(|branch| {
                            widgets::info_chip::<HistoryMessage>(
                                format!("当前分支 {branch}"),
                                BadgeTone::Accent,
                            )
                        }))
                        .push_maybe(state.current_upstream_ref.as_ref().map(|upstream| {
                            widgets::info_chip::<HistoryMessage>(
                                format!("上游 {upstream}"),
                                BadgeTone::Neutral,
                            )
                        })),
                )
                .width(Length::Fill),
            )
            .push(
                scrollable::styled_horizontal(
                    Row::new()
                        .spacing(theme::spacing::XS)
                        .align_y(Alignment::Center)
                        .push(button::secondary(
                            "搜索",
                            can_search.then_some(HistoryMessage::Search),
                        ))
                        .push(button::ghost(
                            "清除",
                            can_clear.then_some(HistoryMessage::ClearSearch),
                        )),
                )
                .width(Length::Fill),
            ),
    )
    .padding([16, 16])
    .style(theme::panel_style(Surface::Panel))
    .into()
}

fn build_commit_detail(info: &git_core::commit::CommitInfo) -> Element<'_, HistoryMessage> {
    Container::new(
        Column::new()
            .spacing(theme::spacing::MD)
            .push(widgets::section_header(
                "详情",
                "提交详情",
                "查看作者、时间、父提交数量和完整提交消息。",
            ))
            .push(
                Row::new()
                    .spacing(theme::spacing::XS)
                    .push(widgets::info_chip::<HistoryMessage>(
                        format!("提交 {}", &info.id[..8]),
                        BadgeTone::Accent,
                    ))
                    .push(widgets::info_chip::<HistoryMessage>(
                        format!("父提交 {}", info.parent_ids.len()),
                        BadgeTone::Neutral,
                    )),
            )
            .push(widgets::status_banner::<HistoryMessage>(
                "作者",
                format!("{} <{}>", info.author_name, info.author_email),
                BadgeTone::Neutral,
            ))
            .push(widgets::status_banner::<HistoryMessage>(
                "时间",
                format_timestamp(info.author_time),
                BadgeTone::Neutral,
            ))
            .push(
                Container::new(
                    scrollable::styled(Text::new(&info.message).size(12))
                        .height(Length::Fixed(180.0)),
                )
                .padding([12, 14])
                .style(theme::panel_style(Surface::Raised)),
            ),
    )
    .padding([16, 16])
    .style(theme::panel_style(Surface::Panel))
    .into()
}

pub fn view(state: &HistoryState) -> Element<'_, HistoryMessage> {
    let status_panel = if state.is_loading {
        Some(build_status_panel::<HistoryMessage>(
            "加载中",
            "正在读取提交历史。",
            BadgeTone::Neutral,
        ))
    } else if state.is_searching {
        Some(build_status_panel::<HistoryMessage>(
            "搜索中",
            "正在按关键词筛选提交历史。",
            BadgeTone::Neutral,
        ))
    } else if let Some(error) = state.error.as_ref() {
        Some(build_status_panel::<HistoryMessage>(
            "失败",
            error,
            BadgeTone::Danger,
        ))
    } else if state.entries.is_empty() {
        Some(build_status_panel::<HistoryMessage>(
            "空状态",
            "当前仓库还没有可显示的提交历史；先创建一次提交，再回来查看时间线。",
            BadgeTone::Neutral,
        ))
    } else if state.filtered_entries.is_empty() && !state.search_query.trim().is_empty() {
        Some(build_status_panel::<HistoryMessage>(
            "无匹配结果",
            format!("没有找到与“{}”匹配的提交。", state.search_query.trim()),
            BadgeTone::Warning,
        ))
    } else if let Some(info) = state.selected_commit_info.as_ref() {
        Some(build_status_panel::<HistoryMessage>(
            "详情已加载",
            format!("当前正在查看提交 {} 的完整信息。", &info.id[..8]),
            BadgeTone::Success,
        ))
    } else if !state.search_query.trim().is_empty() {
        Some(build_status_panel::<HistoryMessage>(
            "搜索完成",
            format!(
                "关键词“{}”匹配到 {} 条提交。",
                state.search_query.trim(),
                state.filtered_entries.len()
            ),
            BadgeTone::Success,
        ))
    } else {
        let total = state.entries.len();
        let filtered = state.filtered_entries.len();
        Some(build_status_panel::<HistoryMessage>(
            "历史概览",
            if filtered < total {
                format!("当前显示 {} / {} 个提交。", filtered, total)
            } else {
                format!("当前显示 {} 个提交。", total)
            },
            BadgeTone::Accent,
        ))
    };

    if state.entries.is_empty() && !state.is_loading && state.error.is_none() {
        return Container::new(
            scrollable::styled(
                Column::new()
                    .spacing(theme::spacing::MD)
                    .push(widgets::section_header(
                        "历史",
                        "提交历史",
                        "在同一视图里完成搜索、浏览和提交详情查看。",
                    ))
                    .push(widgets::panel_empty_state(
                        "历史",
                        "当前仓库还没有提交历史",
                        "先完成一次提交，或刷新历史列表后再回来查看时间线。",
                        Some(button::ghost("刷新", Some(HistoryMessage::Refresh)).into()),
                    )),
            )
            .height(Length::Fill),
        )
        .padding([16, 18])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::panel_style(Surface::Panel))
        .into();
    }

    let detail_panel: Element<'_, HistoryMessage> =
        if let Some(info) = state.selected_commit_info.as_ref() {
            build_commit_detail(info)
        } else if !state.search_query.trim().is_empty() && state.filtered_entries.is_empty() {
            widgets::panel_empty_state(
                "详情",
                "没有匹配的提交",
                format!("没有找到与“{}”匹配的提交。", state.search_query.trim()),
                Some(button::ghost("清除搜索", Some(HistoryMessage::ClearSearch)).into()),
            )
        } else {
            widgets::panel_empty_state(
                "详情",
                "还没有选中任何提交",
                "选中一条提交后查看作者、时间和完整提交消息。",
                None,
            )
        };

    Container::new(
        scrollable::styled(
            Column::new()
                .spacing(theme::spacing::MD)
                .push(widgets::section_header(
                    "历史",
                    "提交历史",
                    "在同一视图里完成搜索、浏览和提交详情查看。",
                ))
                .push(
                    Row::new()
                        .spacing(theme::spacing::MD)
                        .push(widgets::stat_card(
                            "可见提交",
                            state.filtered_entries.len().to_string(),
                            "当前搜索结果中的提交数",
                        ))
                        .push(widgets::stat_card(
                            "当前分支",
                            state
                                .current_branch_name
                                .clone()
                                .unwrap_or_else(|| "detached HEAD".to_string()),
                            "历史动作会围绕这个分支继续展开",
                        )),
                )
                .push(
                    Column::new()
                        .spacing(theme::spacing::XS)
                        .push(
                            scrollable::styled_horizontal(
                                Row::new()
                                    .spacing(theme::spacing::XS)
                                    .push(button::ghost("刷新", Some(HistoryMessage::Refresh))),
                            )
                            .width(Length::Fill),
                        )
                        .push_maybe(status_panel),
                )
                .push(build_search_bar(state))
                .push(
                    stack([
                        build_history_list(state),
                        build_commit_context_menu_overlay(state),
                    ])
                    .width(Length::Fill)
                    .height(Length::Shrink),
                )
                .push(detail_panel),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, parents: &[&str]) -> HistoryEntry {
        HistoryEntry {
            id: id.to_string(),
            message: id.to_string(),
            author_name: "tester".to_string(),
            author_email: "tester@example.com".to_string(),
            timestamp: 0,
            parent_ids: parents.iter().map(|parent| parent.to_string()).collect(),
        }
    }

    #[test]
    fn build_history_graph_keeps_linear_history_on_single_lane() {
        let entries = vec![entry("c3", &["c2"]), entry("c2", &["c1"]), entry("c1", &[])];

        let graph = build_history_graph(&entries);

        assert_eq!(graph.lane_count, 1);
        assert_eq!(graph.rows.len(), 3);
        assert!(graph.rows.iter().all(|row| row.node_lane == 0));
        assert_eq!(graph.rows[0].parent_transitions.len(), 1);
        assert_eq!(graph.rows[0].parent_transitions[0].from_lane, 0);
        assert_eq!(graph.rows[0].parent_transitions[0].to_lane, 0);
        assert_eq!(graph.rows[1].parent_transitions.len(), 1);
        assert_eq!(graph.rows[1].parent_transitions[0].from_lane, 0);
        assert_eq!(graph.rows[1].parent_transitions[0].to_lane, 0);
        assert!(graph.rows[2].parent_transitions.is_empty());
    }

    #[test]
    fn build_history_graph_draws_merge_on_multiple_lanes() {
        let entries = vec![
            entry("merge", &["main", "feature"]),
            entry("main", &["base"]),
            entry("feature", &["base"]),
            entry("base", &[]),
        ];

        let graph = build_history_graph(&entries);

        assert_eq!(graph.lane_count, 2);
        assert_eq!(graph.rows[0].node_lane, 0);
        assert_eq!(graph.rows[0].parent_transitions.len(), 2);
        assert_eq!(graph.rows[0].parent_transitions[0].from_lane, 0);
        assert_eq!(graph.rows[0].parent_transitions[0].to_lane, 0);
        assert_eq!(graph.rows[0].parent_transitions[1].from_lane, 0);
        assert_eq!(graph.rows[0].parent_transitions[1].to_lane, 1);

        assert_eq!(graph.rows[1].continuing.len(), 1);
        assert_eq!(graph.rows[1].continuing[0].from_lane, 1);
        assert_eq!(graph.rows[1].continuing[0].to_lane, 1);

        assert_eq!(graph.rows[2].node_lane, 1);
        assert_eq!(graph.rows[2].parent_transitions.len(), 1);
        assert_eq!(graph.rows[2].parent_transitions[0].from_lane, 1);
        assert_eq!(graph.rows[2].parent_transitions[0].to_lane, 0);
    }
}
