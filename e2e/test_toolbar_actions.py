"""
E2E: 工具栏操作 — 刷新、提交对话框、分支弹窗
"""
import pyautogui
import rpa


class Test刷新:
    def test_点击刷新状态(self, app):
        rpa.窗口内点击(0.77, 0.03)
        rpa.延时(2)
        rpa.截图对比("50_刷新后")


class Test提交对话框:
    def test_打开提交(self, app):
        rpa.窗口内点击(0.925, 0.03)
        rpa.延时(1)
        rpa.截图对比("51_提交对话框")

    def test_ESC关闭(self, app):
        rpa.按键("escape")
        rpa.延时(0.5)
        rpa.截图对比("52_提交已关闭")


class Test分支弹窗:
    def test_点击分支名(self, app):
        rpa.窗口内点击(0.14, 0.03)
        rpa.延时(1)
        rpa.截图对比("53_分支弹窗")

    def test_ESC关闭(self, app):
        rpa.按键("escape")
        rpa.延时(0.5)
        rpa.截图对比("54_分支已关闭")
