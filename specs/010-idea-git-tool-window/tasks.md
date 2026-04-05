# Tasks: 一比一还原 IntelliJ IDEA Git Tool Window 的交互布局与样式

**Feature Branch**: `010-idea-git-tool-window`
**Generated**: 2026-04-04
**Plan**: [plan.md](plan.md) | **Spec**: [spec.md](spec.md)

---

## Implementation Strategy

This feature is primarily a **layout restructuring** of existing UI components. Several pieces from the plan are already present in the current codebase on `009-branch-history-actions`:

- `widgets::commit_panel.rs` exists and is already embedded in `build_changes_body`
- `GitToolWindowTab { Changes, Log }` and its message routing exist in `state.rs` and `main.rs`
- Right-click context menu for changelist was recently added

Therefore, the implementation strategy is:
1. **Audit** the current state to confirm what's already done.
2. **Finish** the remaining structural gaps: single-toolbar chrome, full-height Log tab, and removal of bottom-docked History defaults.
3. **Polish** by running `cargo check` / `cargo test` and doing a manual walkthrough.

MVP scope = User Story 1 + User Story 2 (non-modal commit + Changes/Log tabs), since these are P1 and already 80% present.

---

## Phase 1: Audit & Setup

**Goal**: Understand exactly which plan items are already implemented and which require new work.

- [ ] T001 Read and audit current state of key files (`src-ui/src/main.rs`, `src-ui/src/state.rs`, `src-ui/src/views/main_window.rs`, `src-ui/src/views/commit_dialog.rs`, `src-ui/src/views/history_view.rs`, `src-ui/src/widgets/commit_panel.rs`, `src-ui/src/widgets/changelist.rs`) to document what is already present vs. what needs change.

---

## Phase 2: Foundational State Adjustments

**Goal**: Ensure `AppShellState` supports the new tab/layout model without legacy History docking assumptions.

- [ ] T002 [P] Update `src-ui/src/state.rs` to remove or generalize any `is_docked_auxiliary_view(AuxiliaryView::History)` special case so History is no longer forced into the bottom panel by default.
- [ ] T003 [P] Verify `src-ui/src/state.rs` restores the previous `git_tool_window_tab` (Changes or Log) when navigating back from `ShellSection::Conflicts` to `ShellSection::Changes`.

---

## Phase 3: User Story 1 — 非模态提交流程 (P1)

**Story Goal**: Committing happens inline in an embedded panel, never in a modal dialog.

**Independent Test**: Open a repo with changes → see commit panel in right-bottom pane → type message → click Commit → no modal appears, commit succeeds.

- [ ] T004 [US1] Verify `src-ui/src/widgets/commit_panel.rs` renders correctly inside the right-hand vertical split in `src-ui/src/main.rs` (diff on top, commit panel on bottom).
- [ ] T005 [P] [US1] Remove or disable the `AuxiliaryView::Commit` fullscreen modal path in `src-ui/src/main.rs`; ensure `ShowChanges` / `Commit` actions route to the inline panel instead.
- [ ] T006 [P] [US1] Refactor `src-ui/src/views/commit_dialog.rs` to extract any remaining modal-only UI into `src-ui/src/widgets/commit_panel.rs`, or delete the modal dialog view if it is fully superseded.
- [ ] T007 [US1] Ensure the Commit button in the toolbar (when implemented in US3) focuses the commit message field in `commit_panel.rs` without opening any modal.

---

## Phase 4: User Story 2 — Changes / Log 标签页切换 (P1)

**Story Goal**: Changes and Log are peer tabs inside the workspace body; Log occupies the full main-body height.

**Independent Test**: Click Log tab → full-height history view appears. Click Changes tab → returns to changes+diff+commit layout. Selected commit in Log persists.

- [ ] T008 [US2] Verify `src-ui/src/main.rs` renders a visible tab bar (`Changes` | `Log`) inside the workspace body and that `SwitchGitToolWindowTab` correctly routes to `build_changes_body` or `build_log_body`.
- [ ] T009 [P] [US2] Adjust `src-ui/src/views/history_view.rs` so it uses `Length::Fill` height and does not have a hardcoded 290px/320px cap when used as the Log tab content.
- [ ] T010 [P] [US2] Update `src-ui/src/views/main_window.rs` to default `bottom_tool_window` to `None` and remove History-specific bottom-rail placement from the navigation rail.
- [ ] T011 [US2] Ensure `HistoryState.selected_commit` and search query persist when switching away from Log and back.

---

## Phase 5: User Story 3 — 工具栏与头部简化 (P2)

**Story Goal**: A single compact toolbar at the top contains repo/branch selector + Refresh, Pull, Commit, Push, Branches.

**Independent Test**: Top chrome is one row. Clicking each main action works. Clicking Commit from Log auto-switches to Changes.

- [ ] T012 [US3] Collapse the two-tier chrome in `src-ui/src/views/main_window.rs` into a single compact toolbar row.
- [ ] T013 [P] [US3] Remove redundant utility buttons (`"历史"`, `"远程"`, `"标签"`, `"储藏"`, `"Rebase"`) from the top header in `src-ui/src/views/main_window.rs`; relocate them to overflow menus or the left rail if still needed.
- [ ] T014 [P] [US3] Wire the Commit button in the new single toolbar so that if `git_tool_window_tab` is `Log`, it sends `SwitchGitToolWindowTab(Changes)` and then focuses the commit panel message input.

---

## Phase 6: User Story 4 — 变更列表的上下文菜单 (P2)

**Story Goal**: Right-clicking a file in the changelist shows a context menu with Show Diff, Stage/Unstage, Rollback, Copy Path.

**Note**: The context menu implementation was recently added. This phase is mainly verification.

**Independent Test**: Right-click a file in Changes list → menu appears → clicking each action works.

- [ ] T015 [US4] Verify in `src-ui/src/widgets/changelist.rs` that `mouse_area().on_right_press()` emits `OpenChangeContextMenu(path)` and that cursor tracking is wired.
- [ ] T016 [US4] Verify in `src-ui/src/main.rs` that `build_change_context_menu_overlay()` includes all required actions (查看差异, 暂存/取消暂存, 回滚修改, 复制路径) and that `RevertFile` uses `git_core::index::discard_file`.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Goal**: Ensure compilation, tests, and manual verification pass.

- [ ] T017 Run `cargo check` in the repo root and fix any compilation errors introduced by the layout changes.
- [ ] T018 Run `cargo test` and ensure all existing unit/integration tests pass.
- [ ] T019 Perform the manual verification walkthrough from [quickstart.md](quickstart.md) (Changes tab layout, non-modal commit, Log tab, toolbar, context menu, conflicts) and fix any UI regressions.

---

## Dependency Graph

```text
Phase 1 (Audit)
    │
    ▼
Phase 2 (Foundational)
    │
    ├──► Phase 3 (US1 - Non-modal commit)
    │        └──► Phase 7 (Polish)
    │
    ├──► Phase 4 (US2 - Changes/Log tabs)
    │        └──► Phase 7 (Polish)
    │
    ├──► Phase 5 (US3 - Toolbar simplification)
    │        └──► Phase 7 (Polish)
    │
    └──► Phase 6 (US4 - Context menu verification)
             └──► Phase 7 (Polish)
```

## Parallel Execution Opportunities

- **T002** and **T003** can run in parallel (different files in `state.rs`).
- **T005** and **T006** can run in parallel (`main.rs` vs `commit_dialog.rs`).
- **T009** and **T010** can run in parallel (`history_view.rs` vs `main_window.rs`).
- **T012**, **T013**, and **T014** can largely run in parallel within `main_window.rs` or as coordinated edits.

## Summary

- **Total tasks**: 19
- **By User Story**: US1 = 4, US2 = 4, US3 = 3, US4 = 2
- **Setup + Foundational + Polish**: 6
- **MVP scope**: User Stories 1 + 2 (T001–T011) deliver the core IDEA-style non-modal commit and tab switching paradigm.
