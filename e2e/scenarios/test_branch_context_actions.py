"""
E2E 场景: 分支弹窗右键菜单功能 (Branch Popup Context Menu Actions)

按键精灵思维，测试分支弹窗中两种右键菜单:

  A. 分支右键菜单 (右键分支名):
    组 1 "常用":     Checkout / Create Branch from / Checkout and Rebase
    组 2 "比较":     Compare with / Show Worktree Diff
    组 3 "集成":     Rebase Current onto / Merge to Current
    组 4 "远程":     Fetch / Push / Set Upstream
    组 5 "维护":     Rename
    组 6 "危险动作": Delete

  B. 提交右键菜单 (右键提交行):
    组 1 "常用":     Copy Hash / Export Patch
    组 2 "定位":     View Diff / Compare / Jump Parent / Jump Child
    组 3 "派生":     Create Branch / Tag Commit
    组 4 "应用":     Cherry-pick / Revert
    组 5 "危险":     Reset to Here / Push to Here

分支弹窗坐标系 (1728x1080):
  弹窗区域: x=0.03~0.97, y=0.06~0.96
  左侧分支列表: x=0.03~0.32
  中间提交历史: x=0.32~0.62
  右侧详情:     x=0.62~0.97
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

BRANCH_BTN = (0.11, 0.04)
CHANGES_TAB = (0.045, 0.07)

# 分支弹窗内部坐标
SEARCH_INPUT = (0.15, 0.10)
BRANCH_ROW_1 = (0.15, 0.25)       # 第一个分支
BRANCH_ROW_2 = (0.15, 0.29)       # 第二个分支
BRANCH_ROW_3 = (0.15, 0.33)       # 第三个分支

# 提交历史行 (弹窗中间区域)
POPUP_COMMIT_1 = (0.45, 0.25)
POPUP_COMMIT_2 = (0.45, 0.29)
POPUP_COMMIT_3 = (0.45, 0.33)

STEP_COUNTER = {"n": 0}


def step(label: str):
    STEP_COUNTER["n"] += 1
    driver.window_screenshot(f"bctx_{STEP_COUNTER['n']:02d}_{label}")


def shot_region(rx, ry, rw, rh, label: str):
    STEP_COUNTER["n"] += 1
    driver.region(rx, ry, rw, rh, f"bctx_{STEP_COUNTER['n']:02d}_{label}")


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


def open_popup():
    """打开分支弹窗。"""
    ensure_focus()
    driver.click_relative(*BRANCH_BTN)
    driver.sleep(1.5)


def close_popup():
    """关闭分支弹窗。"""
    driver.press("escape")
    driver.sleep(0.5)


def right_click_at(coords):
    """在弹窗内右键点击。"""
    rect = driver.get_bounds()
    x = int(coords[0] * rect.w) + rect.x
    y = int(coords[1] * rect.h) + rect.y
    driver.right_click(x, y)
    driver.sleep(1)


# ═══════════════════════════════════════════════════════════════════════
# 准备: 创建多分支环境
# ═══════════════════════════════════════════════════════════════════════

class Test00_准备多分支环境:
    """为右键菜单测试准备多分支。"""

    def test_01_创建多分支(self, app):
        """创建几个分支用于测试。"""
        clean_repo(app)
        # 确保有足够提交
        write_file(app, "src/main.py", 'def main():\n    print("branch ctx")\n')
        git_run(app, "add", ".")
        git_run(app, "commit", "-m", "feat: branch context test base")

        # 创建分支
        for branch in ["feature/ctx-test", "bugfix/ctx-fix", "release/v1.0"]:
            subprocess.run(
                ["git", "branch", branch], cwd=app, capture_output=True,
            )

        driver.sleep(2)
        branches = git_cmd(app, "branch")
        print(f"已有分支:\n{branches}")
        step("branches_prepared")


# ═══════════════════════════════════════════════════════════════════════
# A1. 分支右键菜单 — 完整布局
# ═══════════════════════════════════════════════════════════════════════

class Test01_分支右键菜单布局:
    """打开分支弹窗，右键分支查看完整菜单。"""

    def test_01_打开弹窗(self, app):
        open_popup()
        step("popup_opened")

    def test_02_右键非当前分支(self, app):
        """右键一个非当前分支（应有完整菜单项）。"""
        right_click_at(BRANCH_ROW_2)
        step("branch_ctx_full")

    def test_03_菜单全貌截图(self, app):
        shot_region(0.08, 0.20, 0.35, 0.55, "branch_ctx_all_groups")

    def test_04_常用分组(self, app):
        """Checkout / Create Branch / Checkout and Rebase。"""
        shot_region(0.08, 0.20, 0.35, 0.12, "branch_ctx_common")

    def test_05_集成分组(self, app):
        """Rebase / Merge。"""
        shot_region(0.08, 0.38, 0.35, 0.10, "branch_ctx_integration")

    def test_06_危险动作分组(self, app):
        """Delete (红色 danger 样式)。"""
        shot_region(0.08, 0.55, 0.35, 0.08, "branch_ctx_danger")

    def test_07_关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.3)

    def test_08_关闭弹窗(self, app):
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# A2. 右键当前分支 (部分项应禁用)
# ═══════════════════════════════════════════════════════════════════════

class Test02_当前分支右键:
    """右键当前分支，某些项应该被禁用（如 Checkout、Delete）。"""

    def test_01_打开弹窗(self, app):
        open_popup()
        step("current_branch_popup")

    def test_02_右键当前分支(self, app):
        """当前分支 (main) 通常在列表顶部。"""
        right_click_at(BRANCH_ROW_1)
        step("current_branch_ctx")

    def test_03_截取菜单_禁用项(self, app):
        """Checkout 和 Delete 对当前分支应禁用。"""
        shot_region(0.08, 0.20, 0.35, 0.55, "current_branch_disabled")

    def test_04_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# A3. 分支右键 → Merge to Current
# ═══════════════════════════════════════════════════════════════════════

class Test03_右键合并分支:
    """测试右键分支 → Merge to Current。"""

    def test_01_准备_在feature分支添加提交(self, app):
        git_run(app, "checkout", "feature/ctx-test")
        write_file(app, "src/feature.py", 'def feature():\n    return "ctx"\n')
        git_run(app, "add", ".")
        git_run(app, "commit", "-m", "feat: ctx test feature")
        git_run(app, "checkout", "main")
        driver.sleep(2)

    def test_02_打开弹窗(self, app):
        open_popup()
        step("merge_popup")

    def test_03_搜索feature分支(self, app):
        driver.type_text("feature", interval=0.03)
        driver.sleep(1)
        step("merge_search")

    def test_04_右键feature分支(self, app):
        right_click_at(BRANCH_ROW_1)
        step("merge_ctx_menu")

    def test_05_截取合并选项(self, app):
        """Merge to Current 在"集成"分组中。"""
        shot_region(0.08, 0.20, 0.35, 0.55, "merge_option")

    def test_06_关闭_不实际合并(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# A4. 分支右键 → Delete (Danger)
# ═══════════════════════════════════════════════════════════════════════

class Test04_右键删除分支:
    """测试右键分支 → Delete。"""

    def test_01_打开弹窗(self, app):
        open_popup()
        step("delete_popup")

    def test_02_搜索bugfix分支(self, app):
        driver.type_text("bugfix", interval=0.03)
        driver.sleep(1)
        step("delete_search")

    def test_03_右键bugfix分支(self, app):
        right_click_at(BRANCH_ROW_1)
        step("delete_ctx")

    def test_04_截取删除项(self, app):
        """Delete 是最后一组，红色 danger 样式。"""
        shot_region(0.08, 0.50, 0.35, 0.10, "delete_danger")

    def test_05_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# B1. 弹窗内提交右键菜单 — 完整布局
# ═══════════════════════════════════════════════════════════════════════

class Test05_弹窗提交右键菜单:
    """在分支弹窗中，右键提交历史中的提交。"""

    def test_01_打开弹窗_选中分支(self, app):
        open_popup()
        driver.sleep(0.5)
        # 点击一个分支加载其提交历史
        driver.click_relative(*BRANCH_ROW_1)
        driver.sleep(1)
        step("commit_ctx_select_branch")

    def test_02_右键提交行(self, app):
        """右键中间区域的提交行。"""
        right_click_at(POPUP_COMMIT_1)
        step("commit_ctx_full")

    def test_03_菜单全貌(self, app):
        shot_region(0.30, 0.18, 0.40, 0.55, "commit_ctx_all")

    def test_04_常用分组_CopyHash_ExportPatch(self, app):
        shot_region(0.30, 0.18, 0.40, 0.10, "commit_ctx_common")

    def test_05_派生分组_CreateBranch_Tag(self, app):
        shot_region(0.30, 0.38, 0.40, 0.10, "commit_ctx_derive")

    def test_06_应用分组_CherryPick_Revert(self, app):
        shot_region(0.30, 0.48, 0.40, 0.10, "commit_ctx_apply")

    def test_07_关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.3)

    def test_08_关闭弹窗(self, app):
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# B2. 弹窗内连续右键不同提交
# ═══════════════════════════════════════════════════════════════════════

class Test06_弹窗提交快速切换:
    """在弹窗中连续右键不同提交，确保菜单正确切换。"""

    def test_01_打开弹窗_选分支(self, app):
        open_popup()
        driver.click_relative(*BRANCH_ROW_1)
        driver.sleep(1)

    def test_02_右键第一个提交(self, app):
        right_click_at(POPUP_COMMIT_1)
        driver.sleep(0.3)
        step("rapid_commit_ctx1")

    def test_03_直接右键第二个(self, app):
        """不关闭菜单直接右键另一个。"""
        right_click_at(POPUP_COMMIT_2)
        driver.sleep(0.3)
        step("rapid_commit_ctx2")

    def test_04_直接右键第三个(self, app):
        right_click_at(POPUP_COMMIT_3)
        driver.sleep(0.3)
        step("rapid_commit_ctx3")

    def test_05_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# A5. 分支右键 → Rebase Current onto
# ═══════════════════════════════════════════════════════════════════════

class Test07_右键Rebase:
    """测试右键分支 → Rebase Current onto (不实际执行)。"""

    def test_01_打开弹窗(self, app):
        open_popup()
        step("rebase_popup")

    def test_02_右键非当前分支(self, app):
        right_click_at(BRANCH_ROW_2)
        step("rebase_ctx")

    def test_03_截取rebase选项(self, app):
        """Rebase 在"集成"分组中，accent 样式。"""
        shot_region(0.08, 0.35, 0.35, 0.12, "rebase_option")

    def test_04_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# A6. 分支右键 → Rename
# ═══════════════════════════════════════════════════════════════════════

class Test08_右键重命名:
    """测试右键分支 → Rename (维护分组)。"""

    def test_01_打开弹窗(self, app):
        open_popup()
        step("rename_popup")

    def test_02_搜索release分支(self, app):
        driver.type_text("release", interval=0.03)
        driver.sleep(1)

    def test_03_右键release分支(self, app):
        right_click_at(BRANCH_ROW_1)
        step("rename_ctx")

    def test_04_截取重命名项(self, app):
        """Rename 在"维护"分组中。"""
        shot_region(0.08, 0.48, 0.35, 0.08, "rename_option")

    def test_05_关闭(self, app):
        driver.press("escape")
        driver.sleep(0.3)
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# 边界: 弹窗内右键 + ESC + 外部点击
# ═══════════════════════════════════════════════════════════════════════

class Test09_弹窗右键边界:
    """分支弹窗内右键菜单的边界行为。"""

    def test_01_打开弹窗(self, app):
        open_popup()

    def test_02_右键后ESC只关菜单不关弹窗(self, app):
        """ESC 应该先关闭菜单，弹窗仍然保留。"""
        right_click_at(BRANCH_ROW_1)
        driver.sleep(0.5)
        driver.press("escape")
        driver.sleep(0.5)
        step("esc_menu_only")
        # 弹窗应该还在 — 截取确认
        shot_region(0.03, 0.06, 0.94, 0.90, "popup_still_open")

    def test_03_再次ESC关闭弹窗(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        step("esc_popup_closed")

    def test_04_右键菜单外点击关闭菜单(self, app):
        """在菜单外点击应关闭菜单但弹窗保留。"""
        open_popup()
        right_click_at(BRANCH_ROW_1)
        driver.sleep(0.5)
        # 点击菜单外（弹窗内但菜单外的位置）
        driver.click_relative(0.80, 0.50)
        driver.sleep(0.5)
        step("click_outside_menu")

    def test_05_关闭弹窗(self, app):
        close_popup()


# ═══════════════════════════════════════════════════════════════════════
# 清理 & 验证
# ═══════════════════════════════════════════════════════════════════════

class Test99_清理:
    def test_清理分支(self, app):
        clean_repo(app)
        for branch in ["feature/ctx-test", "bugfix/ctx-fix", "release/v1.0"]:
            subprocess.run(
                ["git", "branch", "-D", branch], cwd=app, capture_output=True,
            )
        driver.sleep(2)

    def test_进程存活(self, app):
        assert driver.is_alive(), "分支右键菜单测试后 app 崩溃了"
        step("final_alive")
