"""
E2E Test: Settings panel — open, toggle, save, verify persistence file.
"""

import os
import time
from conftest import take_screenshot, win_click


class TestSettingsPanel:
    def test_open_settings_via_gear_icon(self, app):
        """Click the gear icon (top-right of window) to open settings."""
        take_screenshot("10_before_settings")
        win_click(0.98, 0.035)
        time.sleep(1)
        take_screenshot("11_settings_opened")

    def test_toggle_a_checkbox(self, app):
        """Toggle '签署提交' checkbox."""
        win_click(0.06, 0.17)
        time.sleep(0.5)
        take_screenshot("12_toggled_checkbox")

    def test_click_save(self, app):
        """Click '保存' button at the bottom-right."""
        win_click(0.98, 0.97)
        time.sleep(1)
        take_screenshot("13_after_save")

    def test_settings_file_created(self, app):
        """Settings persistence file should exist on disk after save."""
        candidate_paths = [
            os.path.expanduser("~/Library/Application Support/slio-git/git-settings-v1.txt"),
            os.path.expanduser("~/.local/share/slio-git/git-settings-v1.txt"),
        ]
        found = None
        for p in candidate_paths:
            if os.path.exists(p):
                found = p
                break
        assert found is not None, f"Settings file not found at: {candidate_paths}"
        with open(found) as f:
            content = f.read()
        assert "update_method" in content
        assert "sign_off_commit" in content
        print(f"\n--- Settings file ({found}) ---\n{content}")
