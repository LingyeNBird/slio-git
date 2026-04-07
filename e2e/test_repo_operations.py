"""
E2E Test: Tab navigation between Changes and Log.
"""

import time
from conftest import take_screenshot, win_click


class TestTabNavigation:
    def test_switch_to_log_tab(self, app):
        """Click '日志' tab."""
        win_click(0.09, 0.07)
        time.sleep(1)
        take_screenshot("40_log_tab")

    def test_switch_back_to_changes(self, app):
        """Click '变更' tab."""
        win_click(0.05, 0.07)
        time.sleep(1)
        take_screenshot("41_changes_tab")
