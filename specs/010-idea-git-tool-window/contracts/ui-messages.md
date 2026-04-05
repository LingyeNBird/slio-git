# UI Message Contract

## Tab Navigation Messages

| Message | Triggered By | State Effect | View Effect |
|---------|-------------|--------------|-------------|
| `SwitchGitToolWindowTab(GitToolWindowTab)` | Clicking Changes/Log tab bar | `shell.git_tool_window_tab` updated | Body renders corresponding tab content |
| `ShowChanges` | Toolbar "Changes" action, Conflict resolved | `active_section = Changes`, `git_tool_window_tab = Changes` | Switches to Changes tab |
| `ShowHistory` | Toolbar or left rail history action | `active_section = Changes`, `git_tool_window_tab = Log` | Switches to Log tab |

## Commit Panel Messages

| Message | Triggered By | State Effect | View Effect |
|---------|-------------|--------------|-------------|
| `CommitDialogMessage(CommitDialogMessage)` | User types in commit message box, toggles amend | `commit_dialog` updated | Commit panel re-renders with new text/amend state |
| `Commit` | Toolbar Commit button or commit panel Commit button | Executes git commit using `commit_dialog.message` | Message cleared, workspace refreshed |

## Change List Messages

| Message | Triggered By | State Effect | View Effect |
|---------|-------------|--------------|-------------|
| `SelectChange(String)` | Clicking a file row | `selected_change_path` updated, diff recomputed | Diff viewer displays selected file |
| `StageFile(String)` | Checkbox toggle or context menu | File moved from unstaged/untracked to staged | Change list sections re-render |
| `UnstageFile(String)` | Checkbox toggle or context menu | File moved from staged to unstaged | Change list sections re-render |
| `StageAll` | Toolbar "Stage All" button | All unstaged/untracked moved to staged | Change list updated |
| `UnstageAll` | Toolbar "Unstage All" button | All staged moved to unstaged | Change list updated |
| `RevertFile(String)` | Context menu "Rollback" | `git_core::index::discard_file` called, workspace refreshed | File removed from change list |
| `CopyChangePath(String)` | Context menu "Copy Path" | Path copied to system clipboard | None (clipboard change) |

## Context Menu Messages

| Message | Triggered By | State Effect | View Effect |
|---------|-------------|--------------|-------------|
| `TrackChangeContextMenuCursor(Point)` | Mouse move over changes list | `change_context_menu_cursor` updated | None (tracks position for anchor) |
| `OpenChangeContextMenu(String)` | Right-click on file row | `change_context_menu_path` and anchor set | Context menu overlay appears at cursor |
| `CloseChangeContextMenu` | Click outside menu or action executed | Path and anchor cleared | Menu overlay disappears |

## Conflict Messages

| Message | Triggered By | State Effect | View Effect |
|---------|-------------|--------------|-------------|
| `ShowConflicts` | Detection of unresolved conflicts | `active_section = Conflicts` | Conflicts view replaces workspace |
| `OpenConflictResolver` / `CloseConflictResolver` | User actions | `conflict_resolver` open/closed | Modal or inline conflict UI shown/hidden |

## Invariants

1. `GitToolWindowTab` is only honored when `active_section == Changes`.
2. `Commit` message never opens a modal; it always operates inline.
3. `change_context_menu_path` being `Some` guarantees the context menu overlay is rendered.
4. `selected_change_path` drives both the diff viewer and optionally the change context menu actions.
