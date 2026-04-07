#!/bin/bash
# E2E RPA 测试运行器
# 用法: ./e2e/run.sh [pytest参数...]
#
# 前提:
#   pip3 install pyautogui pillow pytest
#   系统设置 > 隐私与安全 > 辅助功能 > 允许终端
#
# 示例:
#   ./e2e/run.sh                      # 跑全部
#   ./e2e/run.sh -k "设置"            # 只跑设置相关
#   ./e2e/run.sh -v -s                # 详细输出 + 打印

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

rm -rf "$SCRIPT_DIR/screenshots"
mkdir -p "$SCRIPT_DIR/screenshots" "$SCRIPT_DIR/reference_images"

echo "=== slio-git E2E RPA 测试 ==="
echo "截图目录: $SCRIPT_DIR/screenshots/"
echo ""

cd "$SCRIPT_DIR"
python3 -m pytest "$@" -v --tb=short
