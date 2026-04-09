"""
E2E 场景: Push/Pull 对话框选项 (Push/Pull Dialog Options)

覆盖:
  1. 打开 Push 对话框 (Ctrl+Shift+K)
  2. 截图 Push 对话框布局
  3. ESC 关闭
  4. 打开 Pull 按钮下拉
  5. 截图 Pull 对话框
  6. 验证 Pull 策略切换

Push 对话框布局:
  - 目标分支选择
  - Force push 开关
  - Push tags 开关
  - Set upstream 开关
  - 远程仓库选择

Pull 对话框布局:
  - Merge vs Rebase 选择
  - Fast-forward only 开关
  - No-ff 开关
  - Squash 开关
"""

import driver


PULL_BTN = (0.835, 0.03)
PUSH_BTN = (0.88, 0.03)

# 对话框内坐标
DIALOG_CENTER = (0.50, 0.50)
DIALOG_CLOSE = (0.97, 0.07)


def ensure_focus():
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


class TestPush对话框:
    def test_Ctrl_Shift_K打开Push(self, app):
        ensure_focus()
        driver.hotkey("ctrl", "shift", "k")
        driver.sleep(2)
        driver.window_screenshot("push_opt_01_push对话框")

    def test_Push对话框布局截图(self, app):
        """截取 Push 对话框各区域。"""
        # 整个对话框
        driver.region(0.20, 0.10, 0.60, 0.80, "push_opt_02_push全貌")

    def test_ESC关闭Push(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        driver.window_screenshot("push_opt_03_push关闭")


class TestPull对话框:
    def test_点击Pull按钮(self, app):
        ensure_focus()
        driver.click_relative(*PULL_BTN)
        driver.sleep(2)
        driver.window_screenshot("push_opt_04_pull对话框")

    def test_Pull对话框布局截图(self, app):
        driver.region(0.20, 0.10, 0.60, 0.80, "push_opt_05_pull全貌")

    def test_ESC关闭Pull(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        driver.window_screenshot("push_opt_06_pull关闭")


class TestPush按钮点击:
    def test_点击Push按钮(self, app):
        ensure_focus()
        driver.click_relative(*PUSH_BTN)
        driver.sleep(2)
        driver.window_screenshot("push_opt_07_push按钮")
        # 可能弹出错误 (没有 remote)，记录截图
        driver.press("escape")
        driver.sleep(0.5)

    def test_进程存活(self, app):
        assert driver.is_alive()
