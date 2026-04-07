"""
slio-git RPA Engine — 按键精灵风格的桌面自动化核心

核心能力:
  1. 窗口管理: 激活、置顶、获取坐标
  2. 找图点击: locateOnScreen + 自动重试
  3. 区域截图: 截取窗口指定区域用于比对
  4. 智能等待: 等待元素出现/消失
  5. 键鼠模拟: 点击、拖拽、按键、组合键
"""

import os
import subprocess
import time
from dataclasses import dataclass
from typing import Optional, Tuple

import pyautogui
from PIL import Image

pyautogui.FAILSAFE = True
pyautogui.PAUSE = 0.15  # 每次操作间隔

APP_PROCESS = "slio-git"
SCREENSHOT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "screenshots")
REFERENCE_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "reference_images")


@dataclass
class Rect:
    x: int
    y: int
    w: int
    h: int

    @property
    def center(self) -> Tuple[int, int]:
        return (self.x + self.w // 2, self.y + self.h // 2)

    @property
    def right(self) -> int:
        return self.x + self.w

    @property
    def bottom(self) -> int:
        return self.y + self.h


# ═══════════════════════════════════════
# 1. 窗口管理
# ═══════════════════════════════════════

def 激活窗口():
    """将 slio-git 窗口置于最前。"""
    # 尝试 AppleScript
    r = subprocess.run(
        ["osascript", "-e", f'''
            tell application "System Events"
                set frontmost of (first process whose name is "{APP_PROCESS}") to true
            end tell
        '''],
        capture_output=True,
    )
    if r.returncode != 0:
        # 回退: 用 osascript activate
        subprocess.run(
            ["osascript", "-e", f'tell application "{APP_PROCESS}" to activate'],
            capture_output=True,
        )
    time.sleep(0.3)


def 获取窗口区域() -> Rect:
    """获取 slio-git 窗口的位置和尺寸。优先 AppleScript，回退 CGWindowList。"""
    # 方法1: AppleScript
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
        vals = [int(p.strip()) for p in parts]
        return Rect(*vals)

    # 方法2: CGWindowListCopyWindowInfo (Python + Quartz)
    try:
        import Quartz
        window_list = Quartz.CGWindowListCopyWindowInfo(
            Quartz.kCGWindowListOptionOnScreenOnly, Quartz.kCGNullWindowID
        )
        for win in window_list:
            owner = win.get("kCGWindowOwnerName", "")
            name = win.get("kCGWindowName", "")
            if APP_PROCESS in owner or APP_PROCESS in name:
                bounds = win.get("kCGWindowBounds", {})
                if bounds:
                    return Rect(
                        int(bounds["X"]), int(bounds["Y"]),
                        int(bounds["Width"]), int(bounds["Height"]),
                    )
    except ImportError:
        pass

    raise RuntimeError("无法获取窗口区域: AppleScript 和 CGWindowList 均失败")


def 最大化窗口():
    """将窗口尽量铺满屏幕（不用全屏模式，避免动画延迟）。"""
    subprocess.run(
        ["osascript", "-e", f'''
            tell application "System Events"
                tell (first process whose name is "{APP_PROCESS}")
                    set win to first window
                    set position of win to {{0, 25}}
                    set size of win to {{1728, 1080}}
                end tell
            end tell
        '''],
        capture_output=True,
    )
    time.sleep(0.3)


def 窗口置顶并准备():
    """激活 + 最大化 + 等待渲染。"""
    激活窗口()
    最大化窗口()
    time.sleep(0.5)


# ═══════════════════════════════════════
# 2. 截图与找图
# ═══════════════════════════════════════

def 全屏截图(name: str = "screen") -> str:
    """截取全屏并保存。返回文件路径。"""
    os.makedirs(SCREENSHOT_DIR, exist_ok=True)
    path = os.path.join(SCREENSHOT_DIR, f"{name}.png")
    img = pyautogui.screenshot()
    img.save(path)
    return path


def 窗口截图(name: str = "window") -> str:
    """只截取 slio-git 窗口区域。"""
    os.makedirs(SCREENSHOT_DIR, exist_ok=True)
    rect = 获取窗口区域()
    # macOS Retina: 屏幕坐标 × 2 = 像素坐标
    scale = _get_screen_scale()
    img = pyautogui.screenshot(region=(rect.x, rect.y, rect.w, rect.h))
    path = os.path.join(SCREENSHOT_DIR, f"{name}.png")
    img.save(path)
    return path


def 区域截图(rx: float, ry: float, rw: float, rh: float, name: str = "region") -> str:
    """截取窗口内指定比例区域 (0.0~1.0)。用于生成参考图。"""
    os.makedirs(REFERENCE_DIR, exist_ok=True)
    rect = 获取窗口区域()
    x = rect.x + int(rect.w * rx)
    y = rect.y + int(rect.h * ry)
    w = int(rect.w * rw)
    h = int(rect.h * rh)
    img = pyautogui.screenshot(region=(x, y, w, h))
    path = os.path.join(REFERENCE_DIR, f"{name}.png")
    img.save(path)
    return path


def 找图(image_path: str, confidence: float = 0.8, region=None) -> Optional[Rect]:
    """在屏幕上查找图片，返回匹配区域。找不到返回 None。"""
    try:
        location = pyautogui.locateOnScreen(image_path, confidence=confidence, region=region)
        if location:
            return Rect(location.left, location.top, location.width, location.height)
    except pyautogui.ImageNotFoundException:
        pass
    return None


def 等待图片出现(image_path: str, timeout: float = 5, confidence: float = 0.8) -> Optional[Rect]:
    """反复查找图片直到出现或超时。按键精灵的 '找图等待'。"""
    deadline = time.time() + timeout
    while time.time() < deadline:
        result = 找图(image_path, confidence=confidence)
        if result:
            return result
        time.sleep(0.3)
    return None


def 等待图片消失(image_path: str, timeout: float = 5, confidence: float = 0.8) -> bool:
    """等待图片从屏幕消失。"""
    deadline = time.time() + timeout
    while time.time() < deadline:
        if not 找图(image_path, confidence=confidence):
            return True
        time.sleep(0.3)
    return False


# ═══════════════════════════════════════
# 3. 鼠标操作
# ═══════════════════════════════════════

def 点击(x: int, y: int):
    """移动鼠标到指定位置并单击。"""
    pyautogui.click(x, y)


def 双击(x: int, y: int):
    pyautogui.doubleClick(x, y)


def 右键(x: int, y: int):
    pyautogui.rightClick(x, y)


def 窗口内点击(rx: float, ry: float):
    """点击窗口内的相对位置 (0.0~1.0)。"""
    rect = 获取窗口区域()
    x = rect.x + int(rect.w * rx)
    y = rect.y + int(rect.h * ry)
    pyautogui.click(x, y)


def 找图并点击(image_path: str, timeout: float = 5, confidence: float = 0.8) -> bool:
    """找到图片后点击其中心。按键精灵的核心操作。"""
    result = 等待图片出现(image_path, timeout, confidence)
    if result:
        cx, cy = result.center
        pyautogui.click(cx, cy)
        return True
    return False


def 拖拽(x1: int, y1: int, x2: int, y2: int, duration: float = 0.5):
    pyautogui.moveTo(x1, y1)
    pyautogui.drag(x2 - x1, y2 - y1, duration=duration)


# ═══════════════════════════════════════
# 4. 键盘操作
# ═══════════════════════════════════════

def 按键(key: str):
    """按下单个键。"""
    pyautogui.press(key)


def 组合键(*keys: str):
    """组合键，如 组合键('command', 'c')。"""
    pyautogui.hotkey(*keys)


def 输入文字(text: str, interval: float = 0.02):
    """逐字输入（仅 ASCII）。"""
    pyautogui.typewrite(text, interval=interval)


# ═══════════════════════════════════════
# 5. 智能等待与断言
# ═══════════════════════════════════════

def 延时(seconds: float):
    time.sleep(seconds)


def 断言图片存在(image_path: str, msg: str = "", timeout: float = 5, confidence: float = 0.8):
    """断言指定图片在屏幕上可见。"""
    result = 等待图片出现(image_path, timeout, confidence)
    assert result is not None, msg or f"屏幕上未找到: {image_path}"
    return result


def 断言图片不存在(image_path: str, msg: str = "", timeout: float = 3, confidence: float = 0.8):
    """断言指定图片不在屏幕上。"""
    gone = 等待图片消失(image_path, timeout, confidence)
    assert gone, msg or f"图片仍在屏幕上: {image_path}"


def 截图对比(name: str) -> str:
    """截取当前窗口并保存为测试步骤截图。"""
    return 窗口截图(name)


# ═══════════════════════════════════════
# 6. 辅助
# ═══════════════════════════════════════

def _get_screen_scale() -> int:
    """macOS Retina scale factor."""
    result = subprocess.run(
        ["system_profiler", "SPDisplaysDataType"],
        capture_output=True, text=True,
    )
    if "Retina" in result.stdout:
        return 2
    return 1


def 进程存活() -> bool:
    result = subprocess.run(["pgrep", "-f", APP_PROCESS], capture_output=True)
    return result.returncode == 0
