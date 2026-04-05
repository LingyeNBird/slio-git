# Feature Specification: 一比一还原 IntelliJ IDEA Git Tool Window 的交互布局与样式

**Feature Branch**: `010-idea-git-tool-window`
**Created**: 2026-04-04
**Status**: Draft
**Input**: User description: "一比一还原 ～/git/idea 的git相关功能交互布局和样式"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 非模态提交流程 (Priority: P1)

作为仓库开发者，当我需要提交代码时，我希望在 Git 工具窗口内直接填写提交信息并执行提交，而不需要打开一个模态弹窗遮挡工作区，这样我可以同时查看差异、选择文件并编写提交说明。

**Why this priority**: 非模态提交是 IDEA Git Tool Window 的核心交互范式， Removing the modal dialog fundamentally changes the daily commit workflow and is the single biggest UX improvement for this feature.

**Independent Test**: 用户可以在 Changes 标签页中，看到右侧下半部分的提交信息输入框，输入消息后点击 Commit 按钮，成功生成一次提交，全程无弹窗。

**Acceptance Scenarios**:

1. **Given** 用户打开了一个有变更的仓库，**When** 用户切换到 Changes 标签页，**Then** 右侧区域应同时显示差异预览（上）和提交信息编辑区（下）。
2. **Given** 用户在提交面板中填写了提交信息，**When** 用户点击 Commit 或 Commit and Push，**Then** 系统应执行对应操作且界面保持非模态状态，不弹出全屏对话框。
3. **Given** 用户正在 Amend 模式，**When** 用户取消 Amend 复选框，**Then** 提交面板应切换回普通提交模式，并清空或重置消息内容。

---

### User Story 2 - Changes / Log 标签页切换 (Priority: P1)

作为仓库开发者，我希望在 Git 工具窗口内通过标签页在"变更列表"和"提交历史"之间切换，并且 Log 视图应占据整个工具窗口的主工作区高度，而不是被压缩在底部小面板中，这样我能像 IDEA 一样把历史浏览当作一等公民功能使用。

**Why this priority**: 将 Log 提升为与 Changes 同级的全高标签页，是 IDEA Git Tool Window 布局区别于当前实现的最显著特征，直接影响信息密度和导航效率。

**Independent Test**: 用户可以点击 Log 标签，看到完整的提交历史图谱和详情，历史视图占据整个主工作区（而非底部 290px 面板）。

**Acceptance Scenarios**:

1. **Given** 用户已打开仓库，**When** 用户点击主工作区顶部的 "Log" 标签，**Then** 界面应显示全高的提交历史视图（包含提交列表和右侧详情面板）。
2. **Given** 用户当前在 Log 标签页，**When** 用户点击 "Changes" 标签，**Then** 界面应切换回变更列表+差异+提交面板的组合布局。
3. **Given** 用户在 Log 标签页中选中了一条提交，**When** 用户切换到 Changes 标签后再切回 Log，**Then** 之前选中的提交应保持高亮状态。

---

### User Story 3 - 工具栏与头部简化 (Priority: P2)

作为仓库开发者，我希望 Git 工具窗口的顶部只有一个紧凑的工具栏：左侧显示当前分支，右侧集中放置最常用的动作（刷新、拉取、提交、推送、分支弹出），减少当前分散在两行头部和左侧导航栏中的冗余入口，使我能在单一视觉层级中完成高频操作。

**Why this priority**: 顶部 chrome 的简化大幅降低了认知负荷，是 IDEA Git Tool Window "干净" 视觉风格的关键，虽不如 P1 改变核心工作流，但直接影响长期使用时的操作效率。

**Independent Test**: 用户在仓库打开状态下，顶部只能看到一个紧凑工具栏，并能在其中完成刷新、提交、推送、分支切换等高频操作。

**Acceptance Scenarios**:

1. **Given** 用户已打开仓库，**When** 用户查看顶部区域，**Then** 应只看到一个高度紧凑的工具栏，包含分支选择器、刷新、Pull、Commit、Push、Branches 按钮，不再出现多余的两行头部结构。
2. **Given** 用户点击工具栏中的分支名称，**When** 分支弹出菜单展开，**Then** 用户可以在弹出菜单中快速切换分支或执行新建/检出操作。
3. **Given** 用户点击 Commit 按钮，**When** 当前不在 Changes 标签页，**Then** 界面应自动切回 Changes 标签页并将焦点移动到提交信息输入框。

---

### User Story 4 - 变更列表的上下文菜单 (Priority: P2)

作为仓库开发者，当我右键点击变更列表中的某个文件时，我希望看到一个类似 IDEA 的上下文菜单，包含查看差异、暂存/取消暂存、回滚修改、复制路径等常用动作，从而不需要移动鼠标到 distant toolbar 上完成这些高频文件级操作。

**Why this priority**: 右键菜单是 IDEA 用户肌肉记忆的重要组成部分，能显著减少文件级操作的鼠标移动距离，提升 staging/discarding 等微任务的完成速度。

**Independent Test**: 用户在 Changes 列表中右键点击任意文件，弹出菜单中可执行查看差异、暂存、回滚、复制路径，点击后菜单消失并执行对应动作。

**Acceptance Scenarios**:

1. **Given** 变更列表中有未暂存的文件，**When** 用户右键点击该文件并选择"暂存"，**Then** 该文件应移动到已暂存区域。
2. **Given** 变更列表中有文件，**When** 用户右键点击该文件并选择"回滚修改"，**Then** 系统应恢复该文件到 HEAD 状态（或删除未跟踪文件），且变更列表中不再显示该文件。
3. **Given** 用户右键点击文件，**When** 选择"复制路径"，**Then** 该文件的相对路径应被写入系统剪贴板。

---

### Edge Cases

- **空工作区**: 当没有变更时，Changes 标签页的左侧面板应显示空状态提示（如 "Nothing to commit"），右侧面板的 diff 和 commit panel 应保持可用或显示合适的空状态。
- **冲突存在**: 当存在合并冲突时，系统应优先展示冲突解决界面；冲突解决完成后自动回到 Changes 标签页。
- **无选中文件**: 当用户未在变更列表中选中任何文件时，右侧 diff 面板应显示提示信息而非空白。
- **Log 加载失败**: 如果Log 标签页加载历史失败，应在历史视图内显示错误提示而不影响 Changes 标签页的使用。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统必须在主工作区内部提供 "Changes" 和 "Log" 两个标签页，并在标签栏中明确标识当前激活的标签。
- **FR-002**: 当 "Changes" 标签页激活时，系统必须展示左侧变更列表和右侧上下分栏的内容：上半部分为差异预览，下半部分为提交信息编辑区（包含消息框、Amend 复选框、Commit / Commit and Push 按钮）。
- **FR-003**: 当 "Log" 标签页激活时，系统必须将提交历史视图作为主工作区的全高内容展示，不再将其限制在底部固定高度面板中。
- **FR-004**: 系统必须在顶部提供一个紧凑的单一工具栏，包含：当前分支显示、刷新、拉取、提交、推送、分支管理入口；移除或隐藏原有的分散在两行头部中的冗余按钮。
- **FR-005**: 系统必须在变更列表中为每个文件项支持右键菜单，菜单项至少包含：查看差异、暂存 / 取消暂存、回滚修改、复制路径。
- **FR-006**: 当用户点击 Commit 按钮时，如果当前不在 Changes 标签页，系统必须自动切换回 Changes 标签页并将焦点设置到提交信息输入区，全程不弹出模态对话框。
- **FR-007**: 系统必须保留现有的冲突解决流程：当存在冲突时，用户仍可进入冲突解决视图；冲突处理完成后自动返回到 Changes 标签页。
- **FR-008**: 系统必须保留 Log 标签页中已有的提交右键菜单功能（如 Cherry-pick、Revert、Compare 等），并确保这些操作在历史视图中仍可正常使用。

### Assumptions

- "Console" 标签页（IDEA 中的 Git 输出控制台）不在本次范围内，将作为未来可选扩展。
- 远程管理（Remotes）、标签管理（Tags）、储藏管理（Stashes）、交互式 Rebase 编辑器等辅助视图在本次范围内仍可作为全屏/弹窗形式保持可用，不强制集成到 Git Tool Window 内部。
- 冲突处理优先于正常 Changes 展示，存在未解决冲突时系统应继续进入冲突视图而不阻塞用户操作。

### Key Entities

- **Git Tool Window Tab**: 表示主工作区的当前内容标签，当前包括 Changes 和 Log。
- **Change Item**: 表示工作区中的单个文件变更，包含文件路径、变更状态（已暂存 / 未暂存 / 未跟踪）、是否选中等属性。
- **Commit Panel**: 非模态的提交信息编辑与提交动作触发区域，包含提交消息、Amend 模式、作者信息提示、Commit / Commit and Push 动作。
- **History Entry**: 表示提交历史中的单条提交记录，包含提交哈希、提交消息、作者、时间戳、父提交列表等。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户可以在 2 次点击内完成从打开仓库到查看 Changes 标签页并选中一个文件查看差异的完整流程。
- **SC-002**: 用户执行一次提交的平均操作路径（从选中文件到点击 Commit 成功）相比旧版模态弹窗流程缩短至少 30%。
- **SC-003**: 95% 的情况下，用户不需要离开 Git 工具窗口主区域即可完成暂存、查看差异、提交、查看历史这四项核心操作。
- **SC-004**: 在 UI 走查中，主工作区布局（标签栏位置、左右分栏比例、顶部工具栏高度）与 IntelliJ IDEA Git Tool Window 的相似度达到可识别的水平，无明显布局错位或功能缺失。
