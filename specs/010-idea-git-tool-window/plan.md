# Implementation Plan: 一比一还原 IntelliJ IDEA Git Tool Window 的交互布局与样式

**Branch**: `010-idea-git-tool-window` | **Date**: 2026-04-04 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/010-idea-git-tool-window/spec.md`

## Summary

将 slio-git 的主工作区布局重构为 IntelliJ IDEA Git Tool Window 的现代非模态范式：在主工作区内引入 Changes/Log 标签页切换；将提交面板从全屏模态对话框改为嵌入 Changes 标签页右侧下方的非模态面板；将 History 从底部 290px 停靠面板提升为 Log 标签页的全高主内容；简化顶部 chrome 为单一紧凑工具栏。同时保留现有的冲突解决、分支弹出菜单、历史右键菜单等全部功能。

## Technical Context

**Language/Version**: Rust 2021+
**Primary Dependencies**: Iced 0.14 (pure Rust UI framework), git2 0.19 (libgit2 bindings), notify 8 (file watching)
**Storage**: N/A (git repositories are file-based)
**Testing**: cargo test (unit + integration tests using real git repository fixtures)
**Target Platform**: Windows 10+, macOS 11+, Ubuntu 20.04+/equivalent Linux
**Project Type**: desktop-app
**Performance Goals**: Git status/diff/log operations under 100ms perceived latency; application startup under 300ms
**Constraints**: Memory footprint < 80MB idle; No WebView/Electron/Tauri; All UI text in Chinese with platform-appropriate CJK fonts
**Scale/Scope**: Single-user desktop Git client; layout restructuring of existing views without adding new git operations

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. IntelliJ Compatibility | ✅ PASS | Layout and interaction flow directly replicate IDEA Git Tool Window (non-modal commit, Changes/Log tabs, single toolbar). |
| II. Rust + Iced Stack | ✅ PASS | All changes remain within existing Rust + Iced architecture; no external UI runtime introduced. |
| III. Library-First Architecture | ✅ PASS | Git operations remain in `git-core`; UI layer only handles view routing and state mapping. |
| IV. Integration Testing for Git Parity | ✅ PASS | Existing integration tests cover commit, branch, diff, merge, stash workflows. Plan includes `cargo test` verification step. |
| V. Observability | ✅ PASS | Structured logging already present in git-core and UI operations; no changes needed. |
| VI. 中文本地化支持 | ✅ PASS | All new labels and菜单项 will use existing Chinese i18n strings (`i18n::ZH_CN`). |

## Project Structure

### Documentation (this feature)

```text
specs/010-idea-git-tool-window/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
src-ui/src/
├── main.rs                      # Body builder, message routing, tab switching
├── state.rs                     # GitToolWindowTab enum, AppShellState updates
├── views/
│   ├── main_window.rs           # Collapsed single-toolbar chrome
│   ├── commit_dialog.rs         # Extract embeddable commit panel UI
│   └── history_view.rs          # Full-height layout adjustments
├── widgets/
│   ├── commit_panel.rs          # New embedded non-modal commit widget
│   ├── changelist.rs            # Context menu (already completed)
│   └── menu.rs                  # Shared menu primitives
├── i18n.rs                      # Chinese labels
└── theme.rs                     # Darcula styling tokens

src/git-core/src/
└── index.rs                     # discard_file for rollback action

tests/
└── workflow_regressions.rs      # Existing workflow parity tests
```

**Structure Decision**: The project follows a library-first desktop app structure. `git-core` remains unchanged except for supporting utilities (`discard_file`). All UI restructuring happens in `src-ui/src` with clear separation: `state.rs` for app state, `views/` for page-level layouts, `widgets/` for reusable components, `main.rs` for message routing.

## Complexity Tracking

> No constitution violations or unjustified complexity deviations identified.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
