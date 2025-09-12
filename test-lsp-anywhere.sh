#!/bin/bash

# Quick script to test LSP on any project

PROJECT_PATH="${1:-$(pwd)}"

echo "Testing txtx LSP on: $PROJECT_PATH"

# Build if needed
if [ ! -f "/home/amal/dev/tx/txtx/target/debug/txtx" ]; then
    echo "Building txtx..."
    cd /home/amal/dev/tx/txtx
    cargo build --package txtx-cli --bin txtx
fi

# Launch VSCode with the extension
code \
    --extensionDevelopmentPath=/home/amal/dev/tx/txtx/vscode-extension \
    "$PROJECT_PATH"

echo "VSCode launched with txtx LSP extension!"
echo "Open any .tx file to test the language server"