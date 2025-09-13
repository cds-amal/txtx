#!/bin/bash

# Quick script to test LSP on any project
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

PROJECT_PATH="${1:-$(pwd)}"

echo "Testing txtx LSP on: $PROJECT_PATH"

# Build if needed
if [ ! -f "$PROJECT_ROOT/target/debug/txtx" ]; then
    echo "Building txtx..."
    cd "$PROJECT_ROOT"
    cargo build --package txtx-cli --bin txtx
fi

# Launch VSCode with the extension
code \
    --extensionDevelopmentPath="$PROJECT_ROOT/vscode-extension" \
    "$PROJECT_PATH"

echo "VSCode launched with txtx LSP extension!"
echo "Open any .tx file to test the language server"