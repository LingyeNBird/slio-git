"""
slio-git E2E Test Framework — pyautogui RPA driver

Launches the app via fixture, activates its window, and provides
window-relative click helpers.
"""

import os
import subprocess
import signal
import time
import tempfile
import shutil

import pyautogui
import pytest

pyautogui.FAILSAFE = True
pyautogui.PAUSE = 0.3

# The .app bundle
APP_PROCESS = "slio-git"
APP_BUNDLE = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    "..", "dist", "slio-git.app",
)
APP_BINARY = os.path.join(APP_BUNDLE, "Contents", "MacOS", "slio-git")
if not os.path.exists(APP_BINARY):
    APP_BINARY = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "..", "target", "release", "src-ui",
    )
    APP_BUNDLE = None

SCREENSHOT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "screenshots")


def _ensure_screenshot_dir():
    os.makedirs(SCREENSHOT_DIR, exist_ok=True)


def take_screenshot(name: str) -> str:
    """Capture full screen and save to screenshots/. Returns path."""
    _ensure_screenshot_dir()
    path = os.path.join(SCREENSHOT_DIR, f"{name}.png")
    img = pyautogui.screenshot()
    img.save(path)
    return path


def activate_app():
    """Bring slio-git window to front using AppleScript."""
    subprocess.run(
        ["osascript", "-e", f'''
            tell application "System Events"
                set frontmost of (first process whose name is "{APP_PROCESS}") to true
            end tell
        '''],
        capture_output=True,
    )
    time.sleep(0.5)


def get_window_bounds():
    """Get window (x, y, w, h) via AppleScript."""
    result = subprocess.run(
        ["osascript", "-e", f'''
            tell application "System Events"
                tell (first process whose name is "{APP_PROCESS}")
                    set win to first window
                    set pos to position of win
                    set sz to size of win
                    return (item 1 of pos) & "," & (item 2 of pos) & "," & (item 1 of sz) & "," & (item 2 of sz)
                end tell
            end tell
        '''],
        capture_output=True, text=True,
    )
    parts = result.stdout.strip().split(",")
    if len(parts) == 4:
        return tuple(int(p.strip()) for p in parts)
    w, h = pyautogui.size()
    return (0, 0, w, h)


def win_click(rx, ry):
    """Click at ratio-based position relative to window (0.0–1.0)."""
    x, y, w, h = get_window_bounds()
    pyautogui.click(x + int(w * rx), y + int(h * ry))


@pytest.fixture(scope="session")
def test_repo():
    """Create a temporary git repo for E2E tests."""
    tmpdir = tempfile.mkdtemp(prefix="slio-e2e-")
    subprocess.run(["git", "init", tmpdir], check=True, capture_output=True)
    subprocess.run(
        ["git", "-C", tmpdir, "config", "user.email", "test@e2e.local"],
        check=True, capture_output=True,
    )
    subprocess.run(
        ["git", "-C", tmpdir, "config", "user.name", "E2E Test"],
        check=True, capture_output=True,
    )
    readme = os.path.join(tmpdir, "README.md")
    with open(readme, "w") as f:
        f.write("# E2E Test Repo\n")
    subprocess.run(["git", "-C", tmpdir, "add", "."], check=True, capture_output=True)
    subprocess.run(
        ["git", "-C", tmpdir, "commit", "-m", "initial commit"],
        check=True, capture_output=True,
    )
    yield tmpdir
    shutil.rmtree(tmpdir, ignore_errors=True)


@pytest.fixture(scope="session")
def app():
    """Launch slio-git via `open`, yield PID, then kill it after all tests."""
    # Kill any existing instance first
    subprocess.run(["pkill", "-f", APP_PROCESS], capture_output=True)
    time.sleep(1)

    if APP_BUNDLE and os.path.isdir(APP_BUNDLE):
        # Launch as a proper macOS app so it gets window management
        subprocess.run(["open", "-a", APP_BUNDLE], check=True)
    else:
        subprocess.Popen([APP_BINARY], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    # Wait for the window to appear
    time.sleep(4)
    activate_app()
    time.sleep(1)

    # Find the PID
    result = subprocess.run(
        ["pgrep", "-f", APP_PROCESS], capture_output=True, text=True,
    )
    pid = int(result.stdout.strip().split("\n")[0]) if result.stdout.strip() else None

    yield pid

    # Cleanup
    if pid:
        try:
            os.kill(pid, signal.SIGTERM)
        except ProcessLookupError:
            pass


def is_app_running():
    """Check if slio-git process is alive."""
    result = subprocess.run(["pgrep", "-f", APP_PROCESS], capture_output=True)
    return result.returncode == 0


@pytest.fixture(autouse=True)
def _focus_app(app):
    """Bring slio-git to front before every test."""
    assert is_app_running(), "App has crashed"
    activate_app()
