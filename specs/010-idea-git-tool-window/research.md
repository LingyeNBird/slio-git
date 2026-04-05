# Phase 0 Research: IDEA Git Tool Window Layout Restructuring

## Decision: Tab bar lives inside the workspace body, not in the global chrome

**Rationale**: IntelliJ IDEA places "Local Changes", "Log", and "Console" as content tabs within the Git Tool Window body area, below any window-level header. The global chrome only contains repo/branch selectors and action buttons. This separation keeps navigation scoped to the tool-window contents.

**Alternatives considered**:
- Tabs in the top chrome alongside branch selector → Rejected because it blends content navigation with window-level actions, violating IDEA's visual hierarchy.
- Left vertical tabs for Changes/Log → Rejected because IDEA uses horizontal tabs at the top of the tool window body.

## Decision: Commit panel is embedded non-modal; CommitDialogState is reused

**Rationale**: The existing `CommitDialogState` already holds all necessary data (message, amend flag, author hint). Extracting the UI into `widgets::commit_panel.rs` allows embedding without state duplication. The modal `AuxiliaryView::Commit` path is removed to match IDEA's non-modal paradigm.

**Alternatives considered**:
- Keep modal dialog but auto-open it → Rejected because it contradicts the core spec requirement of a non-modal workflow.
- Create a new state struct for the embedded panel → Rejected because `CommitDialogState` is already well-tested and contains all required fields.

## Decision: History moves from bottom docked panel to full-height Log tab

**Rationale**: The existing `history_view.rs` is already a self-contained view with its own toolbar, list, and detail panels. Moving it to a full-height tab requires only layout adjustments (removing the 290px height cap) and removing the special-case docking logic. The `MainWindow::bottom_tool_window` infrastructure is retained for future Console use but defaults to `None`.

**Alternatives considered**:
- Keep History in bottom panel and add a "maximize" button → Rejected because it still makes Log a second-class citizen; IDEA treats Log as a primary peer tab.

## Decision: Top chrome collapses to a single compact toolbar

**Rationale**: IDEA's Git tool window header is a single row: repo/branch on the left, then Refresh, Pull, Commit, Push, Branches. The current two-tier chrome (primary + secondary bars) plus left rail creates visual noise. Collapsing to one bar reduces mouse travel and cognitive load.

**Alternatives considered**:
- Keep two bars but hide less-used buttons → Rejected because it still fragments the action area and doesn't achieve IDEA's clean single-header look.

## Decision: Context menu reuses existing `widgets::menu` primitives

**Rationale**: The project already has `widgets::menu::group`, `widgets::menu::action_row`, and `widgets::menu::panel_style` used by `history_view.rs`. Reusing these for the changelist context menu ensures visual consistency and minimizes new code. The pattern uses `mouse_area().on_right_press()` + `opaque()` overlay for menu dismissal.

**Alternatives considered**:
- Iced-native context menu widget → Rejected because Iced 0.14 does not provide a native context menu primitive; the overlay pattern is the established project convention.

## Decision: Conflicts view remains a transient ShellSection overlay

**Rationale**: When conflicts exist, the user's immediate need is resolution. IDEA also temporarily switches the tool window context to conflict information. Keeping `ShellSection::Conflicts` as a transient state that overrides the tab bar avoids confusion between "Changes" and "Conflicts" when the workspace is not committable.

**Alternatives considered**:
- Add "Conflicts" as a third permanent tab → Rejected because conflicts are transient; adding a permanent tab implies users can navigate there at any time, which is misleading when no conflicts exist.

## Technology Constraints Confirmed

- **Iced 0.14 layout model**: Uses `Column`, `Row`, `Container`, `Length::FillPortion` for responsive splits. No CSS/flexbox; ratios are explicit.
- **Message-driven state updates**: All user interactions produce `Message` variants handled in `main.rs`. No direct widget callbacks cross state boundaries.
- **git-core discard_file**: Already implemented using `git checkout HEAD -- file` for tracked files and `std::fs::remove_file`/`remove_dir_all` for untracked files.
