"""
E2E Test: Toolbar actions — refresh, commit dialog, branch popup.
"""

import time
import pyautogui
from conftest import take_screenshot, win_click


class TestToolbarRefresh:
    def test_click_refresh(self, app):
        win_click(0.78, 0.035)
        time.sleep(2)
        take_screenshot("50_after_refresh")


class TestCommitDialog:
    def test_open_commit_dialog(self, app):
        win_click(0.93, 0.035)
        time.sleep(1)
        take_screenshot("51_commit_dialog")

    def test_close_with_escape(self, app):
        pyautogui.press("escape")
        time.sleep(0.5)
        take_screenshot("52_commit_closed")


class TestBranchPopup:
    def test_click_branch_indicator(self, app):
        win_click(0.14, 0.035)
        time.sleep(1)
        take_screenshot("53_branch_popup")

    def test_close_with_escape(self, app):
        pyautogui.press("escape")
        time.sleep(0.5)
        take_screenshot("54_branch_closed")
