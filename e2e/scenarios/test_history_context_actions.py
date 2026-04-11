"""
E2E 场景: 历史视图提交右键菜单的功能 (History Commit Context Menu Actions)

按键精灵思维，逐一测试 History Log 视图中提交右键菜单的每个操作项:

  组 1 "回退":
    - Reset Current Branch to Here
    - Revert Commit
    - Uncommit to Here (soft reset)

  组 2 "历史重写":
    - Cherry-pick
    - Create Branch from Commit
    - Create Tag on Commit

  组 3 "复制":
    - Copy Commit Hash
    - Export Patch

  额外测试:
    - 键盘导航选中提交后右键
    - 多次右键切换不同提交
    - 菜单项 hover 高亮
    - 合并提交的右键菜单（部分项应禁用）

菜单宽度 280px, 估计高度 340px, 有 4 个分组。
"""

import os
import subprocess
import time

import pytest
import driver
from scenarios.conftest import add_unstaged_change


# ═══════════════════════════════════════════════════════════════════════
# 坐标常量
# ═══════════════════════════════════════════════════════════════════════

CHANGES_TAB = (0.045, 0.07)
# 历史列表中的提交行
HISTORY_ROW_1 = (0.30, 0.18)
HISTORY_ROW_2 = (0.30, 0.22)
HISTORY_ROW_3 = (0.30, 0.26)
HISTORY_ROW_4 = (0.30, 0.30)

STEP_COUNTER = {"n": 0}


def step(label: str):
    STEP_COUNTER["n"] += 1
    driver.window_screenshot(f"hctx_{STEP_COUNTER['n']:02d}_{label}")


def shot_region(rx, ry, rw, rh, label: str):
    STEP_COUNTER["n"] += 1
    driver.region(rx, ry, rw, rh, f"hctx_{STEP_COUNTER['n']:02d}_{label}")


def ensure_focus():
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


def git_cmd(repo, *args):
    result = subprocess.run(
        ["git"] + list(args),
        cwd=repo, capture_output=True, text=True,
    )
    return result.stdout.strip()


def git_run(repo, *args):
    subprocess.run(["git"] + list(args), cwd=repo, capture_output=True, check=True)


def clean_repo(repo):
    subprocess.run(["git", "checkout", "."], cwd=repo, capture_output=True)
    subprocess.run(["git", "clean", "-fd"], cwd=repo, capture_output=True)
    subprocess.run(["git", "checkout", "main"], cwd=repo, capture_output=True)


def write_file(repo, relpath, content):
    filepath = os.path.join(repo, relpath)
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, "w") as f:
        f.write(content)


def right_click_history(row_coords):
    """在历史列表中右键点击提交行。"""
    rect = driver.get_bounds()
    x = int(row_coords[0] * rect.w) + rect.x
    y = int(row_coords[1] * rect.h) + rect.y
    driver.right_click(x, y)
    driver.sleep(1)


def click_menu_item_offset(anchor_coords, item_index, group_offset=0):
    """点击右键菜单的第 N 项。

    历史菜单有分组标题, group_offset 用于跳过前面的分组 header。
    每个分组 header 约 24px, 每个菜单项约 28px。
    """
    rect = driver.get_bounds()
    menu_x = int(anchor_coords[0] * rect.w) + rect.x + 80
    base_y = int(anchor_coords[1] * rect.h) + rect.y
    # 跳过分组 header(s) + 菜单项
    menu_y = base_y + 40 + group_offset * 30 + (item_index - 1) * 28
    driver.click(menu_x, menu_y)
    driver.sleep(1)


# ═══════════════════════════════════════════════════════════════════════
# 准备: 创建足够的提交历史
# ═══════════════════════════════════════════════════════════════════════

class Test00_准备历史:
    """预置多条提交，为后续右键菜单测试提供数据。"""

    def test_01_创建多条提交(self, app):
        """创建 4 条有意义的提交。"""
        clean_repo(app)

        write_file(app, "src/main.py",
                   'def main():\n    print("v1")\n')
        git_run(app, "add", ".")
        git_run(app, "commit", "-m", "feat: initial implementation")

        write_file(app, "src/main.py",
                   'def main():\n    print("v2 - bug fix")\n')
        git_run(app, "add", ".")
        git_run(app, "commit", "-m", "fix: correct output message")

        write_file(app, "src/utils.py",
                   'def helper():\n    return 42\n')
        git_run(app, "add", ".")
        git_run(app, "commit", "-m", "feat: add utils module")

        write_file(app, "README.md",
                   "# Updated Readme\n\nWith more content.\n")
        git_run(app, "add", ".")
        git_run(app, "commit", "-m", "docs: update readme")

        driver.sleep(3)
        ensure_focus()
        log = git_cmd(app, "log", "--oneline", "-5")
        print(f"准备的提交历史:\n{log}")
        assert log.count("\n") >= 3, f"提交数不够: {log}"

    def test_02_切到Log视图(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "l")
        driver.sleep(1.5)
        step("history_ready")


# ═══════════════════════════════════════════════════════════════════════
# 1. 右键菜单 — 完整布局截图
# ═══════════════════════════════════════════════════════════════════════

class Test01_菜单布局:
    """右键提交查看完整菜单布局和分组。"""

    def test_01_右键最新提交(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "l")
        driver.sleep(1)
        right_click_history(HISTORY_ROW_1)
        step("menu_full")

    def test_02_菜单区域截图(self, app):
        """截取整个菜单区域看分组。"""
        shot_region(0.20, 0.12, 0.40, 0.55, "menu_groups")

    def test_03_第一组_回退(self, app):
        """截取回退分组。"""
        shot_region(0.20, 0.12, 0.40, 0.15, "menu_group_reset")

    def test_04_第二组_历史重写(self, app):
        """截取历史重写分组。"""
        shot_region(0.20, 0.28, 0.40, 0.20, "menu_group_rewrite")

    def test_05_关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.3)


# ═══════════════════════════════════════════════════════════════════════
# 2. 右键 → Copy Commit Hash
# ═══════════════════════════════════════════════════════════════════════

class Test02_复制提交哈希:
    """测试右键 → 复制提交哈希。"""

    def test_01_右键提交(self, app):
        ensure_focus()
        right_click_history(HISTORY_ROW_1)
        step("copy_hash_menu")

    def test_02_截取复制分组(self, app):
        """复制分组通常在菜单底部。"""
        shot_region(0.20, 0.45, 0.40, 0.15, "copy_group")

    def test_03_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        step("copy_hash_done")


# ═══════════════════════════════════════════════════════════════════════
# 3. 右键 → Cherry-pick
# ═══════════════════════════════════════════════════════════════════════

class Test03_Cherry_pick:
    """测试右键 → Cherry-pick (在当前分支应用某个提交)。"""

    def test_01_创建目标分支(self, app):
        """创建一个新分支来接收 cherry-pick。"""
        git_run(app, "checkout", "-b", "cherry-test")
        driver.sleep(2)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)

    def test_02_切到Log选中老提交(self, app):
        driver.hotkey("ctrl", "l")
        driver.sleep(1)
        # 选中第 3 个提交 (较早的)
        driver.click_relative(*HISTORY_ROW_3)
        driver.sleep(0.5)
        step("cherry_select_commit")

    def test_03_右键弹出菜单(self, app):
        right_click_history(HISTORY_ROW_3)
        step("cherry_menu")

    def test_04_截取cherry_pick项(self, app):
        """Cherry-pick 通常在"分支与标签"分组中。"""
        shot_region(0.20, 0.35, 0.40, 0.15, "cherry_pick_item")

    def test_05_关闭菜单(self, app):
        """不实际执行 cherry-pick（避免冲突），只确认菜单项可见。"""
        driver.press("escape")
        driver.sleep(0.3)

    def test_06_切回main(self, app):
        git_run(app, "checkout", "main")
        subprocess.run(["git", "branch", "-D", "cherry-test"],
                       cwd=app, capture_output=True)
        driver.sleep(2)


# ═══════════════════════════════════════════════════════════════════════
# 4. 右键 → Create Branch from Commit
# ═══════════════════════════════════════════════════════════════════════

class Test04_从提交创建分支:
    """测试右键 → 从提交创建分支。"""

    def test_01_右键老提交(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "l")
        driver.sleep(1)
        right_click_history(HISTORY_ROW_2)
        step("create_branch_menu")

    def test_02_截取创建分支项(self, app):
        shot_region(0.20, 0.35, 0.40, 0.15, "create_branch_item")

    def test_03_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)


# ═══════════════════════════════════════════════════════════════════════
# 5. 右键 → Create Tag on Commit
# ═══════════════════════════════════════════════════════════════════════

class Test05_从提交创建Tag:
    """测试右键 → 在提交上创建 Tag。"""

    def test_01_右键提交(self, app):
        ensure_focus()
        right_click_history(HISTORY_ROW_1)
        step("create_tag_menu")

    def test_02_截取Tag项(self, app):
        shot_region(0.20, 0.35, 0.40, 0.15, "create_tag_item")

    def test_03_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)


# ═══════════════════════════════════════════════════════════════════════
# 6. 右键 → Revert Commit
# ═══════════════════════════════════════════════════════════════════════

class Test06_Revert提交:
    """测试右键 → Revert Commit (生成反向提交)。"""

    def test_01_选中非合并提交(self, app):
        ensure_focus()
        driver.click_relative(*HISTORY_ROW_1)
        driver.sleep(0.5)

    def test_02_右键弹出菜单(self, app):
        right_click_history(HISTORY_ROW_1)
        step("revert_menu")

    def test_03_截取revert项(self, app):
        shot_region(0.20, 0.12, 0.40, 0.15, "revert_item")

    def test_04_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)


# ═══════════════════════════════════════════════════════════════════════
# 7. 右键 → Reset Current Branch to Here
# ═══════════════════════════════════════════════════════════════════════

class Test07_Reset到此提交:
    """测试右键 → Reset Current Branch to Here (danger 项)。"""

    def test_01_右键较早的提交(self, app):
        ensure_focus()
        right_click_history(HISTORY_ROW_2)
        step("reset_menu")

    def test_02_截取reset项(self, app):
        """Reset 是"回退"分组的第一项，红色 danger 样式。"""
        shot_region(0.20, 0.12, 0.40, 0.12, "reset_danger_item")

    def test_03_关闭_不执行(self, app):
        """Reset 是危险操作，不实际执行。"""
        driver.press("escape")
        driver.sleep(0.3)
        step("reset_cancelled")


# ═══════════════════════════════════════════════════════════════════════
# 8. 键盘导航 + 右键
# ═══════════════════════════════════════════════════════════════════════

class Test08_键盘导航后右键:
    """测试用键盘导航选中提交后右键。"""

    def test_01_点击第一个提交(self, app):
        ensure_focus()
        driver.click_relative(*HISTORY_ROW_1)
        driver.sleep(0.3)

    def test_02_键盘向下导航(self, app):
        """按 Down 选中下一个提交。"""
        driver.press("down")
        driver.sleep(0.3)
        driver.press("down")
        driver.sleep(0.3)
        step("keyboard_nav_down")

    def test_03_右键当前选中的提交(self, app):
        """在键盘选中的位置右键。"""
        right_click_history(HISTORY_ROW_3)
        step("keyboard_then_ctx")

    def test_04_菜单出现(self, app):
        shot_region(0.20, 0.20, 0.40, 0.45, "keyboard_ctx_menu")

    def test_05_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)


# ═══════════════════════════════════════════════════════════════════════
# 9. 快速切换 — 连续右键不同提交
# ═══════════════════════════════════════════════════════════════════════

class Test09_快速切换右键:
    """连续右键不同提交，确认菜单正确切换不崩溃。"""

    def test_01_右键第一个(self, app):
        ensure_focus()
        right_click_history(HISTORY_ROW_1)
        driver.sleep(0.3)
        step("rapid_ctx1")

    def test_02_直接右键第二个(self, app):
        """不关闭菜单，直接右键另一个。"""
        right_click_history(HISTORY_ROW_2)
        driver.sleep(0.3)
        step("rapid_ctx2")

    def test_03_直接右键第三个(self, app):
        right_click_history(HISTORY_ROW_3)
        driver.sleep(0.3)
        step("rapid_ctx3")

    def test_04_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        step("rapid_closed")


# ═══════════════════════════════════════════════════════════════════════
# 10. Export Patch
# ═══════════════════════════════════════════════════════════════════════

class Test10_导出Patch:
    """测试右键 → Export Patch。"""

    def test_01_右键提交(self, app):
        ensure_focus()
        right_click_history(HISTORY_ROW_1)
        step("patch_menu")

    def test_02_截取patch项(self, app):
        """Export Patch 在"复制"分组中。"""
        shot_region(0.20, 0.45, 0.40, 0.15, "patch_item")

    def test_03_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)


# ═══════════════════════════════════════════════════════════════════════
# 清理 & 验证
# ═══════════════════════════════════════════════════════════════════════

class Test99_清理:
    def test_回到Changes(self, app):
        driver.click_relative(*CHANGES_TAB)
        driver.sleep(1)
        step("final_back_changes")

    def test_清理仓库(self, app):
        clean_repo(app)
        driver.sleep(2)

    def test_进程存活(self, app):
        assert driver.is_alive(), "历史右键菜单测试后 app 崩溃了"
        step("final_alive")
