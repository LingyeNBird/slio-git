"""
E2E 场景: 项目切换器 (Project Switcher)

覆盖:
  1. 点击顶部项目名打开下拉菜单
  2. 截图下拉菜单内容
  3. 验证当前项目显示
  4. ESC 关闭下拉

项目切换器布局:
  - 顶部工具栏最左侧: 当前仓库名
  - 点击后下拉显示最近打开的仓库列表
  - 列表底部: "打开仓库" / "初始化仓库" 操作
"""

import driver


# 项目名称按钮 (工具栏最左侧，紧贴左边)
PROJECT_NAME_BTN = (0.04, 0.04)


def ensure_focus():
    driver.activate()
    driver.click_relative(0.3, 0.5)
    driver.sleep(0.3)


class Test项目下拉菜单:
    def test_确保窗口聚焦(self, app):
        ensure_focus()

    def test_点击项目名(self, app):
        """点击顶部仓库名打开下拉。"""
        driver.click_relative(*PROJECT_NAME_BTN)
        driver.sleep(1)
        driver.window_screenshot("project_01_下拉菜单")

    def test_下拉菜单截图(self, app):
        """截取下拉菜单区域。"""
        driver.region(0.0, 0.02, 0.25, 0.30, "project_02_下拉详情")

    def test_ESC关闭下拉(self, app):
        driver.press("escape")
        driver.sleep(0.5)
        driver.window_screenshot("project_03_下拉关闭")

    def test_再次打开并点击外部关闭(self, app):
        """验证点击外部区域也能关闭下拉。"""
        driver.click_relative(*PROJECT_NAME_BTN)
        driver.sleep(1)
        driver.click_relative(0.5, 0.5)
        driver.sleep(0.5)
        driver.window_screenshot("project_04_外部关闭")
