# Quickstart: IDEA Git Tool Window Layout

## Build & Run

```bash
cargo check
cargo test
cargo run --bin src-ui
```

## Manual Verification Checklist

### 1. Changes Tab Layout
- Open a repository with working tree changes
- Verify the main body shows a "Changes" tab active by default
- Verify left side shows the change list (staged / unstaged / untracked)
- Verify right side is split vertically: diff preview on top, commit panel on bottom

### 2. Non-Modal Commit
- Type a message in the bottom-right commit panel
- Click "Commit"
- Verify no modal dialog appeared
- Verify the commit succeeded and the message box was cleared

### 3. Log Tab
- Click the "Log" tab in the workspace body
- Verify the history view occupies the full height of the main body
- Verify the commit graph and detail panel are visible
- Switch back to "Changes" and then back to "Log" — selected commit should persist

### 4. Toolbar
- Verify the top chrome is a single compact toolbar
- Verify it contains: branch selector, Refresh, Pull, Commit, Push, Branches
- Verify clicking "Commit" from Log tab auto-switches back to Changes tab

### 5. Right-Click Context Menu
- Right-click a file in the change list
- Verify menu contains: 查看差异, 暂存/取消暂存, 回滚修改, 复制路径
- Select "回滚修改" and verify the file disappears from the list

### 6. Conflicts
- Create a merge conflict in a test repo
- Open the repo in slio-git
- Verify the conflict resolver appears
- Resolve the conflict
- Verify the UI returns to the Changes tab

## Troubleshooting

- **Compilation errors in `main.rs`**: Check that `Message` enum variants for tab switching are correctly mapped in `build_body()`.
- **History view too short**: Ensure `history_view.rs` does not have a hardcoded `max_height` when used in the Log tab path.
- **Commit panel missing**: Verify `widgets::commit_panel` is registered in `src-ui/src/widgets/mod.rs`.
