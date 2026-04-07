"""
slio-git E2E conftest — 按键精灵风格

fixture 负责:
  1. 启动 app (session scope, 通过 open 命令)
  2. 每个 test 前激活窗口
  3. 首次运行时截取参考图
"""

import os
import subprocess
import time

import pytest
import rpa


APP_BUNDLE = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    "..", "dist", "slio-git.app",
)


@pytest.fixture(scope="session")
def app():
    """启动 slio-git，测试结束后关闭。"""
    # 先杀掉已有实例
    subprocess.run(["pkill", "-x", rpa.APP_PROCESS], capture_output=True)
    time.sleep(2)

    # 通过 open 命令启动 .app bundle (注册到 macOS 窗口管理)
    result = subprocess.run(["open", APP_BUNDLE], capture_output=True, text=True)
    if result.returncode != 0:
        pytest.fail(f"启动失败: {result.stderr}")
    time.sleep(5)

    # 激活并最大化
    rpa.窗口置顶并准备()

    yield

    # 清理
    subprocess.run(["pkill", "-x", rpa.APP_PROCESS], capture_output=True)


@pytest.fixture(autouse=True)
def _每个测试前准备(app):
    """每个测试前: 确认进程存活 + 激活窗口。"""
    assert rpa.进程存活(), "slio-git 进程已退出"
    rpa.激活窗口()
    rpa.延时(0.3)
