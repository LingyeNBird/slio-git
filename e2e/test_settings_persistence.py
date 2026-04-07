"""
E2E: 设置面板 — 打开、修改、保存、验证持久化
"""
import os
import rpa


class Test设置面板:
    def test_打开设置(self, app):
        rpa.截图对比("10_设置前")
        # 点击右上角齿轮图标
        rpa.窗口内点击(0.975, 0.03)
        rpa.延时(1)
        rpa.截图对比("11_设置已打开")

    def test_勾选签署提交(self, app):
        # "签署提交 (--sign-off)" 是设置面板第一个 checkbox
        rpa.窗口内点击(0.06, 0.19)
        rpa.延时(0.5)
        rpa.截图对比("12_勾选签署提交")

    def test_点击保存(self, app):
        # "保存" 蓝色按钮在面板右下角
        rpa.窗口内点击(0.975, 0.965)
        rpa.延时(1)
        rpa.截图对比("13_保存后")

    def test_设置文件已写入(self, app):
        """保存后磁盘上应存在持久化文件。"""
        candidates = [
            os.path.expanduser("~/Library/Application Support/slio-git/git-settings-v1.txt"),
            os.path.expanduser("~/.local/share/slio-git/git-settings-v1.txt"),
        ]
        found = next((p for p in candidates if os.path.exists(p)), None)
        assert found, f"设置文件未找到: {candidates}"
        content = open(found).read()
        assert "update_method" in content
        assert "sign_off_commit" in content
        print(f"\n=== {found} ===\n{content}")
