"""
E2E 场景: 文件上下文菜单 (Context Menu)

覆盖:
  1. 右键变更文件弹出菜单
  2. 菜单项截图验证
  3. ESC 关闭菜单
  4. 右键暂存区文件
  5. 菜单外点击关闭

上下文菜单项 (未暂存文件):
  - 暂存文件
  - 查看 Diff
  - 复制路径
  - 在编辑器中打开
  - 丢弃变更 (Revert)

上下文菜单项 (已暂存文件):
  - 取消暂存
  - 查看 Diff
  - 复制路径
"""

import os
import subprocess

import driver
from scenarios.conftest import add_unstaged_change


FILE_ROW_1 = (0.15, 0.18)
FILE_ROW_2 = (0.15, 0.22)


def ensure_focus():
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


class Test未暂存文件右键菜单:
    def test_准备变更(self, app):
        add_unstaged_change(app, filename="src/main.py",
                           content='def main():\n    print("context menu test")\n')
        add_unstaged_change(app, filename="README.md",
                           content="# Context Menu Test\n")
        driver.sleep(4)
        ensure_focus()

    def test_右键第一个文件(self, app):
        """右键点击未暂存文件。"""
        rect = driver.get_bounds()
        x = int(FILE_ROW_1[0] * rect.w) + rect.x
        y = int(FILE_ROW_1[1] * rect.h) + rect.y
        driver.right_click(x, y)
        driver.sleep(1)
        driver.window_screenshot("ctx_01_未暂存文件菜单")

    def test_菜单区域截图(self, app):
        """截取菜单区域详情。"""
        driver.region(0.10, 0.15, 0.25, 0.30, "ctx_02_菜单详情")

    def test_ESC关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        driver.window_screenshot("ctx_03_菜单关闭")


class Test已暂存文件右键菜单:
    def test_全部暂存(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "shift", "s")
        driver.sleep(1)

    def test_右键已暂存文件(self, app):
        """右键点击已暂存区域的文件。"""
        rect = driver.get_bounds()
        x = int(FILE_ROW_1[0] * rect.w) + rect.x
        y = int(FILE_ROW_1[1] * rect.h) + rect.y
        driver.right_click(x, y)
        driver.sleep(1)
        driver.window_screenshot("ctx_04_已暂存文件菜单")

    def test_关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.5)


class Test菜单外点击关闭:
    def test_右键弹出菜单(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "shift", "u")
        driver.sleep(0.5)

        rect = driver.get_bounds()
        x = int(FILE_ROW_1[0] * rect.w) + rect.x
        y = int(FILE_ROW_1[1] * rect.h) + rect.y
        driver.right_click(x, y)
        driver.sleep(1)

    def test_点击菜单外关闭(self, app):
        """点击菜单外部区域应关闭菜单。"""
        driver.click_relative(0.5, 0.5)
        driver.sleep(0.5)
        driver.window_screenshot("ctx_05_菜单外点击关闭")

    def test_清理(self, app):
        subprocess.run(["git", "checkout", "."], cwd=app, capture_output=True)
        subprocess.run(["git", "clean", "-fd"], cwd=app, capture_output=True)
