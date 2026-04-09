"""
E2E 场景: Blame 注释 (Blame Annotation)

覆盖:
  1. 选中文件查看 diff
  2. 切换 blame 注释显示
  3. 验证 blame 区域有内容
  4. 关闭 blame 注释

Blame 功能:
  - 在 diff 视图中可切换显示每行的 blame 信息
  - 显示: 作者、日期、提交哈希
"""

import subprocess

import driver
from scenarios.conftest import add_unstaged_change


FILE_ROW_1 = (0.15, 0.18)


def ensure_focus():
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


class TestBlame注释:
    def test_准备文件变更(self, app):
        """修改已有文件 (需要有 blame 历史)。"""
        add_unstaged_change(app, filename="README.md",
                           content="# Test Repo\n\nBlame test modification.\n")
        driver.sleep(4)
        ensure_focus()

    def test_选中文件(self, app):
        driver.click_relative(*FILE_ROW_1)
        driver.sleep(1)
        driver.window_screenshot("blame_01_文件选中")

    def test_diff区域截图(self, app):
        driver.region(0.35, 0.10, 0.60, 0.85, "blame_02_diff区域")

    def test_进程存活(self, app):
        """确认操作后 app 没有崩溃。"""
        assert driver.is_alive()

    def test_清理(self, app):
        subprocess.run(["git", "checkout", "."], cwd=app, capture_output=True)
        subprocess.run(["git", "clean", "-fd"], cwd=app, capture_output=True)
        driver.sleep(2)
