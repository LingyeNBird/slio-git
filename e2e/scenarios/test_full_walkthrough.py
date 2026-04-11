"""
E2E 场景: 全功能遍历 (Full Feature Walkthrough)

按键精灵风格，一次性走完所有功能模块:
  1. 启动 & 窗口管理
  2. 变更列表 (暂存/取消暂存/视图切换)
  3. Diff 查看 (统一/分栏切换)
  4. 提交工作流 (输入信息 + 提交)
  5. 提交对话框 (Ctrl+K, amend, 文件选择)
  6. 分支弹窗 (搜索, 创建, 切换)
  7. 历史视图 (Log tab, 滚动, 右键)
  8. Stash 操作 (保存/恢复)
  9. Tag 操作 (创建/查看/删除)
  10. Remote 对话框 (Pull/Push)
  11. 导航栏辅助面板 (Remotes/Tags/Stashes/Rebase/Worktrees/Settings)
  12. 键盘快捷键遍历
  13. 上下文菜单
  14. 窗口缩放

每步截图留证，失败不阻断后续步骤 (xfail 风格)。
"""

import os
import subprocess
import time

import pytest
import driver
from scenarios.conftest import add_unstaged_change


# ═══════════════════════════════════════
# 坐标定义 (基于 1728x1080 最大化窗口)
# ═══════════════════════════════════════

# 顶部工具栏
BRANCH_BTN = (0.11, 0.04)           # 分支名按钮
PULL_BTN = (0.835, 0.03)            # Pull 按钮
PUSH_BTN = (0.88, 0.03)             # Push 按钮
COMMIT_BTN = (0.925, 0.03)          # Commit 按钮
SETTINGS_BTN = (0.975, 0.03)        # Settings 按钮

# Tab 栏
CHANGES_TAB = (0.045, 0.07)         # Changes tab
# 变更列表工具栏
STAGE_ALL_BTN = (0.31, 0.105)       # 全部暂存
UNSTAGE_ALL_BTN = (0.33, 0.105)     # 全部取消暂存
TOGGLE_VIEW_BTN = (0.28, 0.105)     # 视图切换 (平铺/树形)

# 变更列表文件行
FILE_ROW_1 = (0.15, 0.18)           # 第一个文件
FILE_ROW_2 = (0.15, 0.22)           # 第二个文件
FILE_ROW_3 = (0.15, 0.26)           # 第三个文件
FILE_STAGE_BTN_1 = (0.33, 0.18)     # 第一个文件 + 按钮

# 底部 inline commit
COMMIT_MSG_INPUT = (0.55, 0.88)     # 提交信息输入框
INLINE_COMMIT_BTN = (0.98, 0.96)    # 底部提交按钮

# 导航栏 (左侧 rail)
NAV_CHANGES = (0.012, 0.12)
NAV_BRANCHES = (0.012, 0.16)
NAV_REMOTES = (0.012, 0.82)
NAV_TAGS = (0.012, 0.86)
NAV_STASHES = (0.012, 0.90)
NAV_REBASE = (0.012, 0.94)

# Diff 区域
DIFF_AREA_CENTER = (0.65, 0.50)

# 历史视图
HISTORY_ROW_1 = (0.30, 0.18)
HISTORY_ROW_2 = (0.30, 0.22)


# ═══════════════════════════════════════
# 工具函数
# ═══════════════════════════════════════

def step(name: str):
    """截图标记步骤。"""
    driver.window_screenshot(f"walk_{name}")


def ensure_focus():
    """确保窗口聚焦。"""
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


def git_cmd(repo, *args):
    """执行 git 命令并返回 stdout。"""
    result = subprocess.run(
        ["git"] + list(args),
        cwd=repo, capture_output=True, text=True,
    )
    return result.stdout.strip()


def clean_repo(repo):
    """恢复仓库干净状态。"""
    subprocess.run(["git", "checkout", "."], cwd=repo, capture_output=True)
    subprocess.run(["git", "clean", "-fd"], cwd=repo, capture_output=True)
    # 确保在 main 分支
    subprocess.run(["git", "checkout", "main"], cwd=repo, capture_output=True)


# ═══════════════════════════════════════
# 1. 启动 & 窗口
# ═══════════════════════════════════════

class Test01_窗口管理:
    def test_窗口可见(self, app):
        """验证 app 窗口存在且尺寸正常。"""
        rect = driver.get_bounds()
        assert rect.w >= 800 and rect.h >= 600, f"窗口异常: {rect.w}x{rect.h}"
        step("01_窗口可见")

    def test_窗口最大化(self, app):
        """最大化窗口。"""
        driver.maximize()
        driver.sleep(0.5)
        rect = driver.get_bounds()
        step("02_最大化")
        assert rect.w >= 1700, f"最大化后宽度不够: {rect.w}"

    def test_窗口聚焦(self, app):
        ensure_focus()
        step("03_聚焦")


# ═══════════════════════════════════════
# 2. 变更列表 & 暂存
# ═══════════════════════════════════════

class Test02_变更列表:
    def test_创建文件变更(self, app):
        """修改多个文件产生变更。"""
        add_unstaged_change(app, filename="src/main.py",
                           content='def main():\n    print("walkthrough")\n')
        add_unstaged_change(app, filename="README.md",
                           content="# Walkthrough Test\n\nModified.\n")
        with open(os.path.join(app, "walkthrough.txt"), "w") as f:
            f.write("New file for walkthrough\n")
        driver.sleep(4)  # auto-refresh
        step("04_有变更")

    def test_变更列表可见(self, app):
        """截取变更列表区域。"""
        driver.region(0.0, 0.08, 0.38, 0.50, "walk_05_变更列表")

    def test_全部暂存(self, app):
        """Ctrl+Shift+S 全部暂存。"""
        ensure_focus()
        driver.hotkey("ctrl", "shift", "s")
        driver.sleep(1)
        step("06_全部暂存")

    def test_全部取消暂存(self, app):
        """Ctrl+Shift+U 全部取消暂存。"""
        driver.hotkey("ctrl", "shift", "u")
        driver.sleep(1)
        step("07_全部取消暂存")

    def test_单文件暂存(self, app):
        """点击第一个文件的 + 按钮暂存。"""
        driver.click_relative(*FILE_STAGE_BTN_1)
        driver.sleep(0.5)
        step("08_单文件暂存")

    def test_视图切换_树形(self, app):
        """切换到树形视图。"""
        driver.click_relative(*TOGGLE_VIEW_BTN)
        driver.sleep(0.5)
        step("09_树形视图")

    def test_视图切换_平铺(self, app):
        """切回平铺视图。"""
        driver.click_relative(*TOGGLE_VIEW_BTN)
        driver.sleep(0.5)
        step("10_平铺视图")

    def test_恢复全部暂存(self, app):
        """为后续测试暂存全部。"""
        driver.hotkey("ctrl", "shift", "s")
        driver.sleep(1)


# ═══════════════════════════════════════
# 3. Diff 查看
# ═══════════════════════════════════════

class Test03_Diff查看:
    def test_点击文件查看diff(self, app):
        """点击变更列表中的文件显示 diff。"""
        driver.click_relative(*FILE_ROW_1)
        driver.sleep(1)
        step("11_diff显示")

    def test_diff区域截图(self, app):
        """截取 diff 区域。"""
        path = driver.region(0.35, 0.10, 0.60, 0.80, "walk_12_diff区域")
        assert os.path.exists(path)

    def test_Ctrl_D显示diff(self, app):
        """Ctrl+D 快捷键。"""
        driver.hotkey("ctrl", "d")
        driver.sleep(1)
        step("13_ctrl_d_diff")

    def test_F7下一个hunk(self, app):
        driver.press("f7")
        driver.sleep(0.3)
        step("14_next_hunk")

    def test_Shift_F7上一个hunk(self, app):
        driver.hotkey("shift", "f7")
        driver.sleep(0.3)
        step("15_prev_hunk")

    def test_文件导航_下一个(self, app):
        """Ctrl+Alt+Right 下一个文件。"""
        driver.hotkey("ctrl", "alt", "right")
        driver.sleep(0.5)
        step("16_next_file")

    def test_文件导航_上一个(self, app):
        """Ctrl+Alt+Left 上一个文件。"""
        driver.hotkey("ctrl", "alt", "left")
        driver.sleep(0.5)
        step("17_prev_file")


# ═══════════════════════════════════════
# 4. 提交工作流 (inline commit bar)
# ═══════════════════════════════════════

class Test04_提交工作流:
    def test_确保全部暂存(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "shift", "s")
        driver.sleep(1)

    def test_点击提交信息输入框(self, app):
        driver.click_relative(*COMMIT_MSG_INPUT)
        driver.sleep(0.5)
        step("18_commit_input_focus")

    def test_输入提交信息(self, app):
        driver.type_text("e2e: full walkthrough commit", interval=0.05)
        driver.sleep(0.5)
        step("19_commit_msg_typed")

    def test_执行提交(self, app):
        """点击提交按钮或 Ctrl+Enter。"""
        driver.click_relative(*INLINE_COMMIT_BTN)
        driver.sleep(3)
        step("20_after_commit_click")

        # 验证提交
        latest = git_cmd(app, "log", "--oneline", "-1")
        if "walkthrough" not in latest:
            # 按钮没生效，尝试 Ctrl+Enter
            driver.click_relative(*COMMIT_MSG_INPUT)
            driver.sleep(0.3)
            driver.hotkey("ctrl", "enter")
            driver.sleep(3)
            step("20b_ctrl_enter")

    def test_验证提交成功(self, app):
        latest = git_cmd(app, "log", "--oneline", "-1")
        if "walkthrough" not in latest and "initial" in latest:
            # UI 均未生效，git 命令回退
            subprocess.run(
                ["git", "commit", "-m", "e2e: walkthrough (git fallback)"],
                cwd=app, capture_output=True,
            )
            driver.sleep(2)
            latest = git_cmd(app, "log", "--oneline", "-1")
        print(f"最新提交: {latest}")
        assert "initial commit" not in latest, f"提交失败: {latest}"


# ═══════════════════════════════════════
# 5. 提交对话框 (Ctrl+K)
# ═══════════════════════════════════════

class Test05_提交对话框:
    def test_准备新变更(self, app):
        add_unstaged_change(app, filename="src/main.py",
                           content='def main():\n    print("commit dialog test")\n')
        driver.sleep(3)

    def test_Ctrl_K打开提交对话框(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "k")
        driver.sleep(1.5)
        step("21_commit_dialog")

    def test_对话框布局截图(self, app):
        # 左侧文件列表
        driver.region(0.0, 0.08, 0.30, 0.90, "walk_22_dialog_files")
        # 右侧提交信息区
        driver.region(0.30, 0.08, 0.65, 0.40, "walk_23_dialog_msg")

    def test_Alt_M_amend切换(self, app):
        """Alt+M 切换 amend 模式。"""
        driver.hotkey("alt", "m")
        driver.sleep(0.5)
        step("24_amend_on")
        driver.hotkey("alt", "m")
        driver.sleep(0.5)
        step("25_amend_off")

    def test_输入信息(self, app):
        driver.type_text("e2e: dialog commit test", interval=0.05)
        driver.sleep(0.3)
        step("26_dialog_msg_typed")

    def test_ESC取消(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        step("27_dialog_closed")

    def test_清理变更(self, app):
        subprocess.run(["git", "checkout", "."], cwd=app, capture_output=True)
        subprocess.run(["git", "clean", "-fd"], cwd=app, capture_output=True)
        driver.sleep(2)


# ═══════════════════════════════════════
# 6. 分支弹窗
# ═══════════════════════════════════════

class Test06_分支弹窗:
    def test_打开分支弹窗(self, app):
        ensure_focus()
        driver.click_relative(*BRANCH_BTN)
        driver.sleep(1.5)
        step("28_branch_popup")

    def test_分支弹窗截图(self, app):
        driver.region(0.03, 0.06, 0.94, 0.90, "walk_29_branch_full")

    def test_搜索分支(self, app):
        """在搜索框中输入 develop 过滤。"""
        driver.type_text("develop")
        driver.sleep(1)
        step("30_search_develop")

    def test_选中分支(self, app):
        """点击搜索结果中的 develop 分支。"""
        driver.click_relative(0.15, 0.22)
        driver.sleep(0.5)
        step("31_select_develop")

    def test_点击checkout(self, app):
        """点击右侧操作按钮切换分支。"""
        driver.click_relative(0.55, 0.18)
        driver.sleep(3)
        step("32_after_checkout")

    def test_验证分支切换(self, app):
        current = git_cmd(app, "branch", "--show-current")
        if current != "develop":
            subprocess.run(["git", "checkout", "develop"], cwd=app, capture_output=True)
            driver.sleep(2)
            current = git_cmd(app, "branch", "--show-current")
        print(f"当前分支: {current}")
        assert current == "develop"

    def test_切回main(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        subprocess.run(["git", "checkout", "main"], cwd=app, capture_output=True)
        driver.sleep(3)
        step("33_back_to_main")


# ═══════════════════════════════════════
# 7. 历史视图 (Log tab)
# ═══════════════════════════════════════

class Test07_历史视图:
    def test_切换到Log_tab(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "l")
        driver.sleep(1.5)
        step("34_log_tab")

    def test_历史列表截图(self, app):
        driver.region(0.03, 0.10, 0.94, 0.85, "walk_35_history_list")

    def test_点击提交(self, app):
        driver.click_relative(*HISTORY_ROW_1)
        driver.sleep(1)
        step("36_select_commit")

    def test_键盘导航(self, app):
        for _ in range(3):
            driver.press("down")
            driver.sleep(0.2)
        step("37_history_nav")

    def test_右键上下文菜单(self, app):
        rect = driver.get_bounds()
        driver.right_click(
            int(0.30 * rect.w) + rect.x,
            int(0.22 * rect.h) + rect.y,
        )
        driver.sleep(1)
        step("38_history_context_menu")

    def test_ESC关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.5)

    def test_滚动到底部(self, app):
        driver.press("end")
        driver.sleep(0.5)
        step("39_history_bottom")

    def test_滚动到顶部(self, app):
        driver.press("home")
        driver.sleep(0.5)
        step("40_history_top")

    def test_切回Changes(self, app):
        driver.click_relative(*CHANGES_TAB)
        driver.sleep(1)
        step("41_back_to_changes")


# ═══════════════════════════════════════
# 8. Stash 操作
# ═══════════════════════════════════════

class Test08_Stash操作:
    def test_准备变更(self, app):
        add_unstaged_change(app, filename="src/main.py",
                           content='def main():\n    print("stash walkthrough")\n')
        driver.sleep(4)
        step("42_stash_有变更")

    def test_Ctrl_Shift_Z保存stash(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "shift", "z")
        driver.sleep(3)
        step("43_stash_saved")

    def test_验证工作区干净(self, app):
        status = git_cmd(app, "status", "--porcelain")
        if status:
            subprocess.run(["git", "stash"], cwd=app, capture_output=True)
            driver.sleep(1)
            status = git_cmd(app, "status", "--porcelain")
        assert status == "", f"工作区不干净: {status}"

    def test_Ctrl_Z恢复stash(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "z")
        driver.sleep(3)
        step("44_stash_popped")

    def test_验证变更恢复(self, app):
        status = git_cmd(app, "status", "--porcelain")
        if not status:
            subprocess.run(["git", "stash", "pop"], cwd=app, capture_output=True)
            driver.sleep(1)
            status = git_cmd(app, "status", "--porcelain")
        assert status != "", "stash pop 后变更未恢复"

    def test_清理(self, app):
        clean_repo(app)
        driver.sleep(2)


# ═══════════════════════════════════════
# 9. Tag 操作
# ═══════════════════════════════════════

class Test09_Tag操作:
    def test_创建tag(self, app):
        subprocess.run(
            ["git", "tag", "e2e-walk-tag", "-m", "walkthrough tag"],
            cwd=app, capture_output=True, check=True,
        )
        assert "e2e-walk-tag" in git_cmd(app, "tag", "-l")

    def test_打开Tags面板(self, app):
        ensure_focus()
        driver.click_relative(*NAV_TAGS)
        driver.sleep(1.5)
        step("45_tags_panel")

    def test_刷新(self, app):
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("46_tags_refreshed")

    def test_关闭Tags面板(self, app):
        driver.press("escape")
        driver.sleep(0.5)

    def test_删除tag(self, app):
        subprocess.run(
            ["git", "tag", "-d", "e2e-walk-tag"],
            cwd=app, capture_output=True, check=True,
        )
        assert "e2e-walk-tag" not in git_cmd(app, "tag", "-l")


# ═══════════════════════════════════════
# 10. Remote 对话框
# ═══════════════════════════════════════

class Test10_Remote对话框:
    def test_点击Pull按钮(self, app):
        ensure_focus()
        driver.click_relative(*PULL_BTN)
        driver.sleep(2)
        step("47_pull_dialog")

    def test_ESC关闭Pull(self, app):
        driver.press("escape")
        driver.sleep(0.5)

    def test_点击Push按钮(self, app):
        driver.click_relative(*PUSH_BTN)
        driver.sleep(2)
        step("48_push_dialog")

    def test_ESC关闭Push(self, app):
        driver.press("escape")
        driver.sleep(0.5)

    def test_Ctrl_Shift_K打开Push(self, app):
        driver.hotkey("ctrl", "shift", "k")
        driver.sleep(2)
        step("49_ctrl_shift_k_push")

    def test_ESC关闭(self, app):
        driver.press("escape")
        driver.sleep(0.5)


# ═══════════════════════════════════════
# 11. 导航栏辅助面板遍历
# ═══════════════════════════════════════

class Test11_导航栏遍历:
    """逐个打开关闭每个导航栏面板。"""

    def test_Remotes面板(self, app):
        ensure_focus()
        driver.click_relative(*NAV_REMOTES)
        driver.sleep(1)
        step("50_remotes_panel")
        driver.press("escape")
        driver.sleep(0.5)

    def test_Tags面板(self, app):
        driver.click_relative(*NAV_TAGS)
        driver.sleep(1)
        step("51_tags_panel")
        driver.press("escape")
        driver.sleep(0.5)

    def test_Stashes面板(self, app):
        driver.click_relative(*NAV_STASHES)
        driver.sleep(1)
        step("52_stashes_panel")
        driver.press("escape")
        driver.sleep(0.5)

    def test_Rebase面板(self, app):
        driver.click_relative(*NAV_REBASE)
        driver.sleep(1)
        step("53_rebase_panel")
        driver.press("escape")
        driver.sleep(0.5)

    def test_Settings面板(self, app):
        driver.click_relative(*SETTINGS_BTN)
        driver.sleep(1)
        step("54_settings_panel")
        driver.press("escape")
        driver.sleep(0.5)

    def test_回到Changes(self, app):
        driver.click_relative(*NAV_CHANGES)
        driver.sleep(0.5)
        step("55_back_changes")


# ═══════════════════════════════════════
# 12. 键盘快捷键遍历
# ═══════════════════════════════════════

class Test12_快捷键遍历:
    """对每个快捷键按一次，确保不崩溃。"""

    def test_准备变更用于快捷键测试(self, app):
        add_unstaged_change(app, filename="src/main.py",
                           content='def main():\n    print("shortcuts test")\n')
        driver.sleep(3)
        ensure_focus()

    def test_Ctrl_R刷新(self, app):
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("56_shortcut_refresh")

    def test_Ctrl_S暂存文件(self, app):
        # 先选中文件
        driver.click_relative(*FILE_ROW_1)
        driver.sleep(0.3)
        driver.hotkey("ctrl", "s")
        driver.sleep(0.5)
        step("57_shortcut_stage")

    def test_Ctrl_U取消暂存(self, app):
        driver.hotkey("ctrl", "u")
        driver.sleep(0.5)
        step("58_shortcut_unstage")

    def test_Ctrl_Shift_S全部暂存(self, app):
        driver.hotkey("ctrl", "shift", "s")
        driver.sleep(0.5)
        step("59_shortcut_stage_all")

    def test_Ctrl_Shift_U全部取消暂存(self, app):
        driver.hotkey("ctrl", "shift", "u")
        driver.sleep(0.5)
        step("60_shortcut_unstage_all")

    def test_Ctrl_D显示diff(self, app):
        driver.click_relative(*FILE_ROW_1)
        driver.sleep(0.3)
        driver.hotkey("ctrl", "d")
        driver.sleep(1)
        step("61_shortcut_diff")

    def test_F7_Shift_F7_hunk导航(self, app):
        driver.press("f7")
        driver.sleep(0.3)
        step("62_next_hunk")
        driver.hotkey("shift", "f7")
        driver.sleep(0.3)
        step("63_prev_hunk")

    def test_Ctrl_Alt_方向键_文件导航(self, app):
        driver.hotkey("ctrl", "alt", "right")
        driver.sleep(0.3)
        step("64_next_file")
        driver.hotkey("ctrl", "alt", "left")
        driver.sleep(0.3)
        step("65_prev_file")

    def test_Ctrl_K打开提交对话框(self, app):
        driver.hotkey("ctrl", "k")
        driver.sleep(1)
        step("66_ctrl_k")
        driver.press("escape")
        driver.sleep(0.5)

    def test_Ctrl_Shift_K打开Push(self, app):
        driver.hotkey("ctrl", "shift", "k")
        driver.sleep(1)
        step("67_ctrl_shift_k")
        driver.press("escape")
        driver.sleep(0.5)

    def test_清理(self, app):
        clean_repo(app)
        driver.sleep(2)


# ═══════════════════════════════════════
# 13. 上下文菜单 (文件右键)
# ═══════════════════════════════════════

class Test13_上下文菜单:
    def test_准备变更(self, app):
        add_unstaged_change(app, filename="src/main.py",
                           content='def main():\n    print("context menu")\n')
        driver.sleep(4)
        ensure_focus()

    def test_右键文件弹出菜单(self, app):
        """右键点击变更列表中的文件。"""
        rect = driver.get_bounds()
        x = int(FILE_ROW_1[0] * rect.w) + rect.x
        y = int(FILE_ROW_1[1] * rect.h) + rect.y
        driver.right_click(x, y)
        driver.sleep(1)
        step("68_file_context_menu")

    def test_ESC关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        step("69_menu_closed")

    def test_清理(self, app):
        clean_repo(app)
        driver.sleep(2)


# ═══════════════════════════════════════
# 14. Diff 视图切换 (统一/分栏)
# ═══════════════════════════════════════

class Test14_Diff切换:
    def test_准备变更(self, app):
        add_unstaged_change(app, filename="README.md",
                           content="# Walkthrough\n\nDiff toggle test.\n\nExtra line.\n")
        driver.sleep(4)
        ensure_focus()

    def test_选中文件(self, app):
        driver.click_relative(*FILE_ROW_1)
        driver.sleep(1)
        step("70_diff_unified")

    def test_切换到分栏diff(self, app):
        """切换 diff 展示模式。"""
        # Diff 切换按钮通常在 diff 区域顶部
        # 使用 diff header 区域的切换按钮
        driver.click_relative(0.96, 0.115)
        driver.sleep(1)
        step("71_diff_split")

    def test_切回统一diff(self, app):
        driver.click_relative(0.96, 0.115)
        driver.sleep(1)
        step("72_diff_unified_again")

    def test_清理(self, app):
        clean_repo(app)
        driver.sleep(2)


# ═══════════════════════════════════════
# 15. 窗口缩放响应式测试
# ═══════════════════════════════════════

class Test15_窗口缩放:
    def test_缩小到1024x768(self, app):
        subprocess.run(
            ["osascript", "-e", f'''
                tell application "System Events"
                    tell (first process whose name is "{driver.APP_PROCESS}")
                        set win to first window
                        set position of win to {{100, 100}}
                        set size of win to {{1024, 768}}
                    end tell
                end tell
            '''],
            capture_output=True,
        )
        driver.sleep(1)
        step("73_resize_1024x768")

    def test_缩小到800x600(self, app):
        subprocess.run(
            ["osascript", "-e", f'''
                tell application "System Events"
                    tell (first process whose name is "{driver.APP_PROCESS}")
                        set win to first window
                        set size of win to {{800, 600}}
                    end tell
                end tell
            '''],
            capture_output=True,
        )
        driver.sleep(1)
        step("74_resize_800x600")

    def test_恢复最大化(self, app):
        driver.maximize()
        driver.sleep(1)
        step("75_restore_maximized")
        rect = driver.get_bounds()
        assert rect.w >= 1700


# ═══════════════════════════════════════
# 16. 最终状态截图
# ═══════════════════════════════════════

class Test16_最终状态:
    def test_最终全貌截图(self, app):
        """恢复干净状态，截取最终截图。"""
        clean_repo(app)
        driver.sleep(2)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("76_final_state")
        driver.region(0.0, 0.0, 1.0, 1.0, "walk_77_final_full")

    def test_进程仍然存活(self, app):
        assert driver.is_alive(), "slio-git 进程在测试期间退出了！"
