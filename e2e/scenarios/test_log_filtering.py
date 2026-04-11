"""
E2E 场景: Log Tab 过滤与多标签 (Log Filtering & Multi-Tab)

覆盖:
  1. 切换到 Log tab
  2. 点击搜索框输入关键词
  3. 验证搜索过滤
  4. 清除搜索
  5. 点击提交查看详情
  6. 右键提交查看上下文菜单
  7. 上下键导航提交列表
  8. 切回 Changes tab

History 视图布局:
  - 顶部: 搜索框 + 搜索/清除按钮 + 过滤下拉
  - 主体: 提交列表 (graph | hash | message | author | date)
  - 底部/右侧: 提交详情 (diff 预览)
"""

import os
import subprocess

import driver


LOG_TAB = (0.06, 0.07)
CHANGES_TAB = (0.045, 0.07)

# 历史视图坐标
SEARCH_INPUT = (0.25, 0.115)
COMMIT_ROW_1 = (0.30, 0.18)
COMMIT_ROW_2 = (0.30, 0.22)
COMMIT_ROW_3 = (0.30, 0.26)


def ensure_focus():
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


class Test提交列表浏览:
    def test_先创建几个提交(self, app):
        """确保有足够多的提交记录。"""
        for i in range(1, 4):
            filepath = os.path.join(app, f"log_filter_{i}.txt")
            with open(filepath, "w") as f:
                f.write(f"Log filter test {i}\n")
            subprocess.run(["git", "add", filepath], cwd=app, capture_output=True)
            subprocess.run(
                ["git", "commit", "-m", f"feat: log filter file {i}"],
                cwd=app, capture_output=True,
            )

    def test_切换到Log(self, app):
        ensure_focus()
        driver.click_relative(*LOG_TAB)
        driver.sleep(1.5)
        driver.window_screenshot("log_filter_01_log_tab")

    def test_提交列表截图(self, app):
        driver.region(0.03, 0.10, 0.94, 0.85, "log_filter_02_提交列表")

    def test_点击搜索框(self, app):
        """点击搜索框准备输入。"""
        driver.click_relative(*SEARCH_INPUT)
        driver.sleep(0.5)

    def test_输入搜索关键词(self, app):
        """搜索 'log filter'。"""
        driver.type_text("log filter", interval=0.05)
        driver.sleep(1)
        driver.window_screenshot("log_filter_03_搜索中")

    def test_按Enter搜索(self, app):
        driver.press("enter")
        driver.sleep(1)
        driver.window_screenshot("log_filter_04_搜索结果")

    def test_清除搜索(self, app):
        """全选+删除清除搜索。"""
        driver.click_relative(*SEARCH_INPUT)
        driver.sleep(0.3)
        driver.hotkey("command", "a")
        driver.sleep(0.1)
        driver.press("backspace")
        driver.sleep(0.5)
        driver.press("enter")
        driver.sleep(1)
        driver.window_screenshot("log_filter_05_清除搜索")


class Test提交详情与导航:
    def test_选中提交查看详情(self, app):
        driver.click_relative(*COMMIT_ROW_1)
        driver.sleep(1)
        driver.window_screenshot("log_filter_06_提交详情")

    def test_键盘上下导航(self, app):
        for _ in range(3):
            driver.press("down")
            driver.sleep(0.3)
        driver.window_screenshot("log_filter_07_导航下移")

        for _ in range(2):
            driver.press("up")
            driver.sleep(0.3)
        driver.window_screenshot("log_filter_08_导航上移")

    def test_右键上下文菜单(self, app):
        rect = driver.get_bounds()
        x = int(COMMIT_ROW_2[0] * rect.w) + rect.x
        y = int(COMMIT_ROW_2[1] * rect.h) + rect.y
        driver.right_click(x, y)
        driver.sleep(1)
        driver.window_screenshot("log_filter_09_提交右键菜单")

    def test_ESC关闭菜单(self, app):
        driver.press("escape")
        driver.sleep(0.5)

    def test_End键跳到底部(self, app):
        driver.press("end")
        driver.sleep(0.5)
        driver.window_screenshot("log_filter_10_跳到底部")

    def test_Home键跳到顶部(self, app):
        driver.press("home")
        driver.sleep(0.5)
        driver.window_screenshot("log_filter_11_跳到顶部")

    def test_切回Changes(self, app):
        driver.click_relative(*CHANGES_TAB)
        driver.sleep(1)
        driver.window_screenshot("log_filter_12_回到changes")
