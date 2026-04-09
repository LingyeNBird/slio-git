"""导航栏操作 — 辅助面板切换。"""

import driver
from .base import auto_screenshot_on_failure


# 导航栏按钮相对坐标 (基于 1728x1080 最大化窗口)
NAV_CHANGES = (0.012, 0.12)
NAV_REMOTES = (0.012, 0.82)
NAV_TAGS = (0.012, 0.86)
NAV_STASHES = (0.012, 0.90)
NAV_REBASE = (0.012, 0.94)


@auto_screenshot_on_failure
def open_remotes():
    """打开 Remotes 面板。"""
    driver.click_relative(*NAV_REMOTES)
    driver.sleep(1)


@auto_screenshot_on_failure
def open_tags():
    """打开 Tags 面板。"""
    driver.click_relative(*NAV_TAGS)
    driver.sleep(1)


@auto_screenshot_on_failure
def open_stashes():
    """打开 Stashes 面板。"""
    driver.click_relative(*NAV_STASHES)
    driver.sleep(1)


@auto_screenshot_on_failure
def open_rebase():
    """打开 Rebase 面板。"""
    driver.click_relative(*NAV_REBASE)
    driver.sleep(1)


@auto_screenshot_on_failure
def back_to_changes():
    """回到 Changes 主视图。"""
    driver.click_relative(*NAV_CHANGES)
    driver.sleep(0.5)


@auto_screenshot_on_failure
def close_panel():
    """ESC 关闭当前面板。"""
    driver.press("escape")
    driver.sleep(0.5)
