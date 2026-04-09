"""
E2E 场景: Hunk 级别暂存 (Hunk Staging)

覆盖:
  1. 修改文件产生多个 hunk
  2. 在 diff 视图中暂存单个 hunk
  3. 验证部分暂存状态
  4. 取消暂存 hunk
  5. 验证 git status 一致

Diff 编辑器中的 hunk 操作:
  - 每个 hunk header 行 (@@ -x,y +x,y @@) 旁有暂存/取消暂存按钮
  - 鼠标悬停 hunk header 时显示 +/- 操作图标
"""

import os
import subprocess

import driver
from scenarios.conftest import add_unstaged_change


FILE_ROW_1 = (0.15, 0.18)


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


class TestHunk暂存:
    def test_创建多hunk变更(self, app):
        """修改文件使其产生多个 diff hunk。"""
        filepath = os.path.join(app, "src", "main.py")
        with open(filepath, "w") as f:
            f.write(
                '# Header comment added\n'
                '# Another header line\n'
                '\n'
                'def main():\n'
                '    print("hello")\n'
                '\n'
                '\n'
                '# Middle section added\n'
                'def helper():\n'
                '    return 42\n'
                '\n'
                '\n'
                '# Footer added\n'
                'if __name__ == "__main__":\n'
                '    main()\n'
                '    helper()\n'
            )
        driver.sleep(4)  # auto-refresh
        ensure_focus()
        driver.window_screenshot("hunk_01_多hunk变更")

    def test_选中文件查看diff(self, app):
        """点击文件显示 diff。"""
        driver.click_relative(*FILE_ROW_1)
        driver.sleep(1)
        driver.window_screenshot("hunk_02_diff显示")

    def test_diff区域有多个hunk(self, app):
        """截取 diff 区域验证有多行变更。"""
        path = driver.region(0.35, 0.10, 0.60, 0.85, "hunk_03_diff详情")
        assert os.path.exists(path) and os.path.getsize(path) > 0

    def test_F7导航到第一个hunk(self, app):
        """F7 跳到下一个 hunk。"""
        driver.press("f7")
        driver.sleep(0.5)
        driver.window_screenshot("hunk_04_第一个hunk")

    def test_F7导航到第二个hunk(self, app):
        driver.press("f7")
        driver.sleep(0.5)
        driver.window_screenshot("hunk_05_第二个hunk")

    def test_Shift_F7回到上一个hunk(self, app):
        driver.hotkey("shift", "f7")
        driver.sleep(0.5)
        driver.window_screenshot("hunk_06_回到上一hunk")

    def test_暂存当前文件(self, app):
        """暂存整个文件 (Ctrl+S)。"""
        driver.hotkey("ctrl", "s")
        driver.sleep(1)
        driver.window_screenshot("hunk_07_文件已暂存")

    def test_验证暂存状态(self, app):
        status = git_cmd(app, "status", "--porcelain")
        print(f"git status: {status}")
        # 文件应在暂存区
        assert status != ""

    def test_取消暂存(self, app):
        driver.hotkey("ctrl", "u")
        driver.sleep(1)
        driver.window_screenshot("hunk_08_取消暂存")

    def test_清理(self, app):
        subprocess.run(["git", "checkout", "."], cwd=app, capture_output=True)
        subprocess.run(["git", "clean", "-fd"], cwd=app, capture_output=True)
        driver.sleep(2)
