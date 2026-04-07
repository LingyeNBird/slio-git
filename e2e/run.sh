#!/bin/bash
# E2E test runner for slio-git
# Usage: ./e2e/run.sh [pytest-args...]
#
# Prerequisites:
#   pip3 install pyautogui pillow pytest
#   Grant Accessibility permissions to Terminal/iTerm in System Preferences
#
# Examples:
#   ./e2e/run.sh                          # run all tests
#   ./e2e/run.sh -k "test_window"         # run specific test
#   ./e2e/run.sh -v --tb=short            # verbose with short traceback

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Build release binary if not present
BINARY="$PROJECT_DIR/target/release/src-ui"
if [ ! -f "$BINARY" ]; then
    echo "Building release binary..."
    cargo build --release --manifest-path "$PROJECT_DIR/Cargo.toml"
fi

export SLIO_GIT_BINARY="$BINARY"

# Clean previous screenshots
rm -rf "$SCRIPT_DIR/screenshots"
mkdir -p "$SCRIPT_DIR/screenshots"

echo "Running E2E tests with pyautogui RPA..."
echo "Binary: $BINARY"
echo "Screenshots: $SCRIPT_DIR/screenshots/"
echo ""

cd "$SCRIPT_DIR"
python3 -m pytest "$@" -v --tb=short
