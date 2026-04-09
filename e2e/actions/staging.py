"""暂存操作 — 单文件/全部暂存取消暂存。"""

import driver
from .base import auto_screenshot_on_failure


@auto_screenshot_on_failure
def stage_all():
    """Ctrl+Shift+S 全部暂存。"""
    driver.hotkey("ctrl", "shift", "s")
    driver.sleep(1)


@auto_screenshot_on_failure
def unstage_all():
    """Ctrl+Shift+U 全部取消暂存。"""
    driver.hotkey("ctrl", "shift", "u")
    driver.sleep(1)


@auto_screenshot_on_failure
def stage_selected():
    """Ctrl+S 暂存选中文件。"""
    driver.hotkey("ctrl", "s")
    driver.sleep(0.5)


@auto_screenshot_on_failure
def unstage_selected():
    """Ctrl+U 取消暂存选中文件。"""
    driver.hotkey("ctrl", "u")
    driver.sleep(0.5)


@auto_screenshot_on_failure
def toggle_view_mode():
    """切换平铺/树形视图。"""
    driver.click_relative(0.28, 0.105)
    driver.sleep(0.5)
