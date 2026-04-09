"""Diff 操作 — 文件选择、hunk 导航、视图切换。"""

import driver
from .base import auto_screenshot_on_failure


FILE_ROW_1 = (0.15, 0.18)


@auto_screenshot_on_failure
def select_file(row: int = 1):
    """点击变更列表中的文件。"""
    y_offsets = {1: 0.18, 2: 0.22, 3: 0.26}
    driver.click_relative(0.15, y_offsets.get(row, 0.18))
    driver.sleep(1)


@auto_screenshot_on_failure
def show_diff():
    """Ctrl+D 显示 diff。"""
    driver.hotkey("ctrl", "d")
    driver.sleep(1)


@auto_screenshot_on_failure
def next_hunk():
    """F7 跳到下一个 hunk。"""
    driver.press("f7")
    driver.sleep(0.3)


@auto_screenshot_on_failure
def prev_hunk():
    """Shift+F7 跳到上一个 hunk。"""
    driver.hotkey("shift", "f7")
    driver.sleep(0.3)


@auto_screenshot_on_failure
def next_file():
    """Ctrl+Alt+Right 下一个文件。"""
    driver.hotkey("ctrl", "alt", "right")
    driver.sleep(0.5)


@auto_screenshot_on_failure
def prev_file():
    """Ctrl+Alt+Left 上一个文件。"""
    driver.hotkey("ctrl", "alt", "left")
    driver.sleep(0.5)


@auto_screenshot_on_failure
def toggle_diff_mode():
    """切换统一/分栏 diff 视图。"""
    driver.click_relative(0.96, 0.115)
    driver.sleep(1)
