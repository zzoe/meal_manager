#!/usr/bin/env bash
# Cross-platform before-packaging wrapper
# Detects OS and sets the correct binary extension

set -e

BINARY_NAME="meal_manager"
BINARY_PATH="./target/release/${BINARY_NAME}"

# Add .exe on Windows (Git Bash / MSYS / Cygwin)
if [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* || "$OSTYPE" == "win32" ]]; then
    BINARY_PATH="${BINARY_PATH}.exe"
fi

robius-packaging-commands before-packaging \
    --force-makepad \
    --binary-name "${BINARY_NAME}" \
    --path-to-binary "${BINARY_PATH}"
