"""
E2E: 启动验证
"""
import os
import rpa


class Test启动:
    def test_进程存活(self, app):
        assert rpa.进程存活()

    def test_窗口可获取(self, app):
        rect = rpa.获取窗口区域()
        assert rect.w > 400 and rect.h > 300, f"窗口太小: {rect.w}x{rect.h}"
        print(f"窗口: ({rect.x}, {rect.y}) {rect.w}x{rect.h}")

    def test_窗口截图(self, app):
        path = rpa.截图对比("01_启动状态")
        assert os.path.exists(path) and os.path.getsize(path) > 0
