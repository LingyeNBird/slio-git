"""
E2E: Tab 切换 — 变更 ↔ 日志
"""
import rpa


class TestTab切换:
    def test_切换到日志(self, app):
        rpa.窗口内点击(0.085, 0.07)
        rpa.延时(1)
        rpa.截图对比("40_日志tab")

    def test_切回变更(self, app):
        rpa.窗口内点击(0.045, 0.07)
        rpa.延时(1)
        rpa.截图对比("41_变更tab")
