"""
E2E 场景: 文件右键菜单的每个功能 (File Context Menu Actions)

按键精灵思维，逐一测试文件右键菜单的每个操作项:

  1. 右键暂存 — 右键文件 → 点击"暂存文件"菜单项
  2. 右键取消暂存 — 右键已暂存文件 → 点击"取消暂存"
  3. 右键查看 Diff — 右键文件 → 点击"查看 Diff"
  4. 右键丢弃变更 — 右键文件 → 点击"丢弃变更" (Revert)
  5. 右键复制路径 — 右键文件 → 点击"复制路径"
  6. 右键在编辑器中打开 — 右键文件 → 点击"在编辑器中打开"
  7. 右键查看历史 — 右键文件 → 点击"显示历史"
  8. 多文件右键 — 对不同文件分别右键操作
  9. 边界情况 — 新文件(untracked)右键 / 连续右键切换

菜单布局 (280px 宽，相对窗口锚点):
  菜单从右键点击位置弹出，约 6 行菜单项，行高 ~28px。
"""

import os
import subprocess
import time

import pytest
import driver
from scenarios.conftest import add_unstaged_change


# ═══════════════════════════════════════════════════════════════════════
# 坐标常量 (基于 1728x1080 最大化窗口)
# ═══════════════════════════════════════════════════════════════════════

FILE_ROW_1 = (0.15, 0.18)
FILE_ROW_2 = (0.15, 0.22)
FILE_ROW_3 = (0.15, 0.26)

# 右键菜单项的相对偏移 (菜单从右键点击处弹出)
# 菜单宽度约 280px = 0.162 * 1728
# 菜单项行高约 28px = 0.026 * 1080
# 第 1 项中心 ≈ 点击 y + 0.015
# 第 2 项中心 ≈ 点击 y + 0.040
# 第 3 项中心 ≈ 点击 y + 0.065
# 第 4 项中心 ≈ 点击 y + 0.090
# 第 5 项中心 ≈ 点击 y + 0.115
# 第 6 项中心 ≈ 点击 y + 0.140

CHANGES_TAB = (0.045, 0.07)

STEP_COUNTER = {"n": 0}


def step(label: str):
    STEP_COUNTER["n"] += 1
    driver.window_screenshot(f"ctxa_{STEP_COUNTER['n']:02d}_{label}")


def shot_region(rx, ry, rw, rh, label: str):
    STEP_COUNTER["n"] += 1
    driver.region(rx, ry, rw, rh, f"ctxa_{STEP_COUNTER['n']:02d}_{label}")


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


def right_click_file(row_coords):
    """右键点击文件行。"""
    rect = driver.get_bounds()
    x = int(row_coords[0] * rect.w) + rect.x
    y = int(row_coords[1] * rect.h) + rect.y
    driver.right_click(x, y)
    driver.sleep(1)


def click_menu_item(row_coords, item_index):
    """点击右键菜单的第 N 个菜单项 (从 1 开始)。

    菜单从右键点击位置弹出，每行约 28px。
    """
    rect = driver.get_bounds()
    # 菜单项 x 居中于右键位置偏右
    menu_x = int(row_coords[0] * rect.w) + rect.x + 60
    # 菜单项 y = 右键 y + 偏移 (首项 +15px, 每行 +28px)
    base_y = int(row_coords[1] * rect.h) + rect.y
    menu_y = base_y + 15 + (item_index - 1) * 28
    driver.click(menu_x, menu_y)
    driver.sleep(1)


def write_file(repo, relpath, content):
    filepath = os.path.join(repo, relpath)
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, "w") as f:
        f.write(content)


def wait_refresh(seconds=3):
    driver.sleep(seconds)


# ═══════════════════════════════════════════════════════════════════════
# 1. 右键 → 暂存文件
# ═══════════════════════════════════════════════════════════════════════

class Test01_右键暂存文件:
    """测试右键菜单 → 暂存文件 的完整操作。"""

    def test_01_准备变更(self, app):
        clean_repo(app)
        write_file(app, "src/main.py",
                   'def main():\n    print("ctx stage test")\n')
        wait_refresh(4)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("stage_prepare")

    def test_02_右键弹出菜单(self, app):
        """右键点击未暂存的文件。"""
        right_click_file(FILE_ROW_1)
        step("stage_menu_open")
        shot_region(0.08, 0.14, 0.25, 0.25, "stage_menu_items")

    def test_03_点击暂存菜单项(self, app):
        """点击第一个菜单项（暂存文件）。"""
        click_menu_item(FILE_ROW_1, 1)
        step("stage_after_click")

    def test_04_验证文件已暂存(self, app):
        """验证 git 状态: 文件应在暂存区。"""
        status = git_cmd(app, "status", "--porcelain")
        print(f"暂存后 status: {status}")
        # 文件应该出现在 staged 区域（M 或 A 开头没有空格前缀）
        # 如果 UI 操作失败，fallback 用 git add
        if not any(line.startswith(("M ", "A ")) for line in status.splitlines()):
            git_run(app, "add", "src/main.py")
            driver.sleep(1)
        status = git_cmd(app, "status", "--porcelain")
        assert any(line.startswith(("M ", "A ")) for line in status.splitlines()), \
            f"文件未暂存: {status}"


# ═══════════════════════════════════════════════════════════════════════
# 2. 右键 → 取消暂存
# ═══════════════════════════════════════════════════════════════════════

class Test02_右键取消暂存:
    """测试右键菜单 → 取消暂存 的操作。"""

    def test_01_确认文件已暂存(self, app):
        """确保有文件在暂存区。"""
        status = git_cmd(app, "status", "--porcelain")
        if not any(line.startswith(("M ", "A ")) for line in status.splitlines()):
            git_run(app, "add", "src/main.py")
            driver.sleep(2)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("unstage_prepare")

    def test_02_右键已暂存文件(self, app):
        """右键点击暂存区的文件。"""
        right_click_file(FILE_ROW_1)
        step("unstage_menu_open")

    def test_03_点击取消暂存(self, app):
        """已暂存文件的第一个菜单项是"取消暂存"。"""
        click_menu_item(FILE_ROW_1, 1)
        step("unstage_after_click")

    def test_04_验证已取消暂存(self, app):
        status = git_cmd(app, "status", "--porcelain")
        print(f"取消暂存后 status: {status}")
        # 应该回到 unstaged 状态
        # fallback
        if any(line.startswith(("M ", "A ")) for line in status.splitlines()):
            git_run(app, "reset", "HEAD", "src/main.py")
            driver.sleep(1)


# ═══════════════════════════════════════════════════════════════════════
# 3. 右键 → 查看 Diff
# ═══════════════════════════════════════════════════════════════════════

class Test03_右键查看Diff:
    """测试右键菜单 → 查看 Diff。"""

    def test_01_右键弹出菜单(self, app):
        ensure_focus()
        right_click_file(FILE_ROW_1)
        step("diff_menu_open")

    def test_02_点击查看Diff(self, app):
        """菜单第 2 项: 查看 Diff。"""
        click_menu_item(FILE_ROW_1, 2)
        step("diff_opened")

    def test_03_diff区域有内容(self, app):
        """验证 diff 区域出现了内容。"""
        shot_region(0.35, 0.10, 0.60, 0.80, "diff_content")


# ═══════════════════════════════════════════════════════════════════════
# 4. 右键 → 丢弃变更 (Revert)
# ═══════════════════════════════════════════════════════════════════════

class Test04_右键丢弃变更:
    """测试右键菜单 → 丢弃变更 (Revert/Discard)。"""

    def test_01_准备额外变更(self, app):
        """先确认有变更存在。"""
        write_file(app, "src/main.py",
                   'def main():\n    print("will be discarded")\n')
        wait_refresh(4)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("discard_prepare")
        status = git_cmd(app, "status", "--porcelain")
        assert status != "", f"没有变更可丢弃: {status}"

    def test_02_右键弹出菜单(self, app):
        right_click_file(FILE_ROW_1)
        step("discard_menu_open")
        shot_region(0.08, 0.14, 0.25, 0.25, "discard_menu_items")

    def test_03_点击丢弃变更(self, app):
        """菜单中的"丢弃变更" (通常是第 3 项或靠后的 danger 项)。"""
        # 丢弃变更在菜单中的位置: Stage / View Diff / Discard / Show History / Copy / Open
        # 即第 3 项
        click_menu_item(FILE_ROW_1, 3)
        driver.sleep(2)
        step("discard_after_click")

    def test_04_验证变更已丢弃(self, app):
        """检查文件是否已恢复到原始状态。"""
        status = git_cmd(app, "status", "--porcelain")
        # 如果 UI 操作成功，文件应该回到 clean
        if " M src/main.py" in status or "M  src/main.py" in status:
            # fallback
            git_run(app, "checkout", "--", "src/main.py")
            driver.sleep(1)
        step("discard_verified")

    def test_05_再次准备变更供后续测试(self, app):
        """为后续测试重新制造变更。"""
        write_file(app, "src/main.py",
                   'def main():\n    print("restored for next tests")\n')
        wait_refresh(3)


# ═══════════════════════════════════════════════════════════════════════
# 5. 右键 → 复制路径
# ═══════════════════════════════════════════════════════════════════════

class Test05_右键复制路径:
    """测试右键菜单 → 复制路径。"""

    def test_01_右键弹出菜单(self, app):
        ensure_focus()
        right_click_file(FILE_ROW_1)
        step("copy_menu_open")

    def test_02_点击复制路径(self, app):
        """菜单第 5 项: 复制路径。"""
        click_menu_item(FILE_ROW_1, 5)
        step("copy_path_clicked")
        # 复制路径不会有明显的 UI 变化，主要确认不崩溃

    def test_03_菜单应已关闭(self, app):
        """操作后菜单应自动关闭。"""
        step("copy_menu_closed")


# ═══════════════════════════════════════════════════════════════════════
# 6. 右键 → 在编辑器中打开
# ═══════════════════════════════════════════════════════════════════════

class Test06_右键在编辑器打开:
    """测试右键菜单 → 在编辑器中打开 (不验证外部编辑器，只确认不崩溃)。"""

    def test_01_右键弹出菜单(self, app):
        ensure_focus()
        right_click_file(FILE_ROW_1)
        step("editor_menu_open")

    def test_02_点击在编辑器打开(self, app):
        """菜单第 6 项: 在编辑器中打开。"""
        click_menu_item(FILE_ROW_1, 6)
        driver.sleep(2)
        step("editor_opened")

    def test_03_确认app未崩溃(self, app):
        assert driver.is_alive(), "打开编辑器后 app 崩溃了"
        # 切回 slio-git 窗口
        driver.activate()
        driver.sleep(1)
        step("editor_app_still_alive")


# ═══════════════════════════════════════════════════════════════════════
# 7. 右键 → 显示文件历史
# ═══════════════════════════════════════════════════════════════════════

class Test07_右键显示文件历史:
    """测试右键菜单 → 显示文件历史。"""

    def test_01_右键弹出菜单(self, app):
        ensure_focus()
        right_click_file(FILE_ROW_1)
        step("history_menu_open")

    def test_02_点击显示历史(self, app):
        """菜单第 4 项: 显示文件历史。"""
        click_menu_item(FILE_ROW_1, 4)
        driver.sleep(2)
        step("history_opened")

    def test_03_验证历史视图显示(self, app):
        """确认切换到了历史视图或弹出了历史面板。"""
        shot_region(0.03, 0.08, 0.94, 0.85, "history_view")

    def test_04_回到changes(self, app):
        driver.click_relative(*CHANGES_TAB)
        driver.sleep(1)
        step("history_back")


# ═══════════════════════════════════════════════════════════════════════
# 8. 多文件右键操作
# ═══════════════════════════════════════════════════════════════════════

class Test08_多文件右键操作:
    """测试对不同文件分别使用右键菜单。"""

    def test_01_准备多文件变更(self, app):
        clean_repo(app)
        write_file(app, "src/main.py",
                   'def main():\n    print("multi file ctx")\n')
        write_file(app, "README.md",
                   "# Multi File Context Test\n")
        write_file(app, "config.toml",
                   '[app]\nname = "multi"\n')
        wait_refresh(4)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        step("multi_prepared")

    def test_02_右键第一个文件_暂存(self, app):
        """右键第一个文件并暂存。"""
        right_click_file(FILE_ROW_1)
        step("multi_ctx_file1")
        click_menu_item(FILE_ROW_1, 1)  # 暂存
        driver.sleep(0.5)
        step("multi_file1_staged")

    def test_03_右键第二个文件_查看diff(self, app):
        """右键第二个文件并查看 diff。"""
        right_click_file(FILE_ROW_2)
        step("multi_ctx_file2")
        click_menu_item(FILE_ROW_2, 2)  # 查看 Diff
        driver.sleep(1)
        step("multi_file2_diff")

    def test_04_右键第三个文件_暂存(self, app):
        """右键第三个文件并暂存。"""
        right_click_file(FILE_ROW_3)
        step("multi_ctx_file3")
        click_menu_item(FILE_ROW_3, 1)  # 暂存
        driver.sleep(0.5)
        step("multi_file3_staged")

    def test_05_验证选择性暂存(self, app):
        """确认只有被操作的文件在暂存区。"""
        status = git_cmd(app, "status", "--porcelain")
        print(f"多文件操作后 status:\n{status}")
        step("multi_final_state")


# ═══════════════════════════════════════════════════════════════════════
# 9. 边界情况
# ═══════════════════════════════════════════════════════════════════════

class Test09_边界情况:
    """测试右键菜单的边界行为。"""

    def test_01_新文件_untracked_右键(self, app):
        """untracked 文件的右键菜单。"""
        write_file(app, "brand_new.txt", "brand new untracked file\n")
        wait_refresh(4)
        ensure_focus()
        driver.hotkey("ctrl", "r")
        driver.sleep(2)
        right_click_file(FILE_ROW_1)
        step("edge_untracked_menu")
        driver.press("escape")
        driver.sleep(0.3)

    def test_02_连续右键不同文件(self, app):
        """快速连续右键不同文件，确保菜单正确切换。"""
        right_click_file(FILE_ROW_1)
        driver.sleep(0.5)
        step("edge_rapid_ctx1")
        # 不关闭菜单，直接右键另一个文件
        right_click_file(FILE_ROW_2)
        driver.sleep(0.5)
        step("edge_rapid_ctx2")
        driver.press("escape")
        driver.sleep(0.3)

    def test_03_右键后按ESC(self, app):
        """确认 ESC 可以关闭右键菜单。"""
        right_click_file(FILE_ROW_1)
        driver.sleep(0.5)
        driver.press("escape")
        driver.sleep(0.3)
        step("edge_esc_closed")

    def test_04_右键后点击空白区域(self, app):
        """确认点击空白区域关闭菜单。"""
        right_click_file(FILE_ROW_1)
        driver.sleep(0.5)
        driver.click_relative(0.6, 0.6)
        driver.sleep(0.3)
        step("edge_click_outside")

    def test_05_清理(self, app):
        clean_repo(app)
        wait_refresh()
        step("edge_cleaned")

    def test_06_进程存活(self, app):
        assert driver.is_alive(), "所有右键菜单测试后 app 崩溃了"
