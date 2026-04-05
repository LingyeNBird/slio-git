# Data Model: IDEA Git Tool Window Layout

## State Entities

### GitToolWindowTab

Represents the active content tab inside the Git tool window body.

```
GitToolWindowTab
├── Changes
└── Log
```

**Transitions**:
- `Changes` ↔ `Log`: Triggered by user clicking the tab bar or by toolbar actions (e.g., clicking Commit button auto-switches to `Changes`).
- Any tab → overridden by `ShellSection::Conflicts` when unresolved conflicts exist.

### AppShellState (existing, extended)

The high-level navigation shell of the application.

```
AppShellState
├── active_section: ShellSection
│   ├── Changes
│   ├── Conflicts
│   └── Welcome
├── git_tool_window_tab: GitToolWindowTab
│   ├── Changes
│   └── Log
└── editor_tab_label: String
```

**Rules**:
- When `active_section` is `Changes`, the UI renders the tab bar and respects `git_tool_window_tab`.
- When `active_section` is `Conflicts`, the tab bar is hidden and the conflict resolver is shown.
- Returning from `Conflicts` to `Changes` restores the previously active `git_tool_window_tab`.

### ChangeItem (existing)

A single file in the working tree with modifications.

```
ChangeItem
├── path: String
└── status: FileStatus
```

**Used in**:
- `staged_changes: Vec<ChangeItem>`
- `unstaged_changes: Vec<ChangeItem>`
- `untracked_files: Vec<ChangeItem>`

### CommitDialogState (existing, reused)

Backs both the legacy modal dialog and the new embedded commit panel.

```
CommitDialogState
├── message: String
├── amend: bool
├── author_name_hint: Option<String>
├── author_email_hint: Option<String>
└── date_hint: Option<String>
```

**Transitions**:
- `amend` toggled ON → message field pre-filled with previous commit message.
- `amend` toggled OFF → message field cleared (or restored to draft if one existed).
- Commit executed → message cleared, `amend` reset to false.

### HistoryState (existing)

Backing state for the Log tab.

```
HistoryState
├── entries: Vec<HistoryEntry>
├── filtered_entries: Vec<HistoryEntry>
├── selected_commit: Option<String>
├── selected_commit_info: Option<CommitInfo>
├── search_query: String
├── context_menu_commit: Option<String>
├── context_menu_cursor: Point
└── context_menu_anchor: Option<Point>
```

**Persistence**:
- `selected_commit` and `search_query` persist across tab switches so returning to Log restores context.

### ChangeContextMenuState

Transient state for the file-level context menu in the changes list.

```
ChangeContextMenuState
├── path: Option<String>
├── cursor: Point
└── anchor: Option<Point>
```

**Lifecycle**:
- Open: set `path` and `anchor` on right-click.
- Close: clear all fields on menu dismissal or action execution.

## Relationships

```
AppShellState
│
├─► GitToolWindowTab (Changes)
│   ├─► ChangeItem[] (staged, unstaged, untracked)
│   │   └─► ChangeContextMenuState (transient)
│   ├─► CommitDialogState
│   └─► Diff
│
└─► GitToolWindowTab (Log)
    └─► HistoryState
```

## Validation Rules

1. **Tab availability**: `Log` tab is always available if a repository is open.
2. **Commit panel availability**: Commit panel is visible in `Changes` tab when repository is open, regardless of whether there are staged changes (IDEA shows it empty but ready).
3. **Context menu actions**:
   - "Stage" is enabled for unstaged/untracked files.
   - "Unstage" is enabled for staged files.
   - "Rollback" is enabled for all tracked files and untracked files.
4. **Conflict override**: If `conflict_files` is non-empty, `ShellSection::Conflicts` takes precedence over `active_section` and tab bar.
