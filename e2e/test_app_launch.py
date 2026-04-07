"""
E2E Test: App launch and window verification.
"""

import os
from conftest import take_screenshot, get_window_bounds, is_app_running


class TestAppWindow:
    def test_screenshot_captures(self, app):
        """Take a screenshot to verify screen capture works."""
        path = take_screenshot("01_current_state")
        assert os.path.exists(path)
        assert os.path.getsize(path) > 0

    def test_window_exists(self, app):
        """Verify we can read the window bounds."""
        x, y, w, h = get_window_bounds()
        assert w > 100 and h > 100, f"Window too small: {w}x{h}"
        take_screenshot("02_window_bounds")

    def test_app_is_running(self, app):
        """App process has not crashed."""
        assert is_app_running()
