# PhpStorm Compact Density Checklist

## Reference anchors

- 顶部区域保持两层以内的紧凑 chrome：仓库 / 分支上下文 + 次级导航，不出现大块留白
- 中央工作区优先突出 changes 列表与 diff 内容，面板 padding 与标题栏厚度接近 PhpStorm
- 顶部文件标签页、底部工具窗标签与状态栏保持 IDE 式低高度节奏
- 分支弹层、提交对话框、远程菜单与历史右键菜单都以 dense list 语言呈现
- 底部日志区以 tool window 方式停靠，不替换当前 changes / diff 上下文
- 状态栏、滚动条、badge、列表项高度保持低噪声、低圆角、低阴影

## Compactness hotspots

- [x] 主壳层包含左侧 rail、紧凑顶栏、editor tab strip、中央工作区、底部日志工具窗
- [x] 顶部仓库 / 分支上下文条、快速操作与次级导航压缩到更接近 PhpStorm 的高度与间距
- [x] changes 列表、diff header、diff 行高、status bar、scrollbar 已整体收紧
- [x] 提交对话框压成更紧凑的两栏工作流，状态条、消息区和动作栏不再像厚卡片
- [x] 分支弹层、历史右键菜单、远程菜单共享同一套 dense menu primitives
- [x] 历史视图以底部 docked tool window 呈现，保留当前仓库 / 分支 / 选中文件上下文

## Review evidence

- `src-ui/src/theme.rs`：统一收紧 toolbar、tab、tool-window、scrollbar、badge 与 compact density token
- `src-ui/src/widgets/menu.rs`：新增共享 menu group / action row / trigger-row highlight primitives
- `src-ui/src/views/main_window.rs`：顶部 chrome、editor tab strip、remote split menu 与 docked history tool window 统一进入更紧凑节奏
- `src-ui/src/main.rs`：历史视图改为 docked surface，不再整页替换 changes / diff
- `src-ui/src/views/branch_popup.rs`：分支 / 提交菜单与触发行高亮切到共享 menu language
- `src-ui/src/views/history_view.rs`：历史面板压成底部工具窗密度，并复用共享菜单样式
- `src-ui/src/views/commit_dialog.rs`：文件列表 / diff 预览 / message editor / action row 进入紧凑两栏布局，并改成单行状态 strip
- `src-ui/src/widgets/changelist.rs`、`src-ui/src/widgets/diff_file_header.rs`、`src-ui/src/widgets/diff_viewer.rs`、`src-ui/src/widgets/split_diff_viewer.rs`、`src-ui/src/widgets/statusbar.rs`：主工作区密度进一步收紧

## Validation

- [x] `cargo test`
- [x] 现有 UI / workflow regression tests 覆盖 repository open、change review、commit dialog、branch popup、history tool window 等关键路径

## Optional next check

- 如需进一步做截图级对照，可直接启动桌面应用并把主工作台、提交对话框和分支弹层与 PhpStorm 参考图逐项比较
