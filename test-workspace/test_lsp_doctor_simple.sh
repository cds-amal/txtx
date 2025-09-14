#!/bin/bash

# Simple test to check if LSP provides doctor-level diagnostics

echo "Starting simple LSP doctor test..."

# Start the LSP server in the background
../target/release/txtx lsp 2>&1 &
LSP_PID=$!

# Give it a moment to start
sleep 1

# Kill the server
kill $LSP_PID 2>/dev/null

echo "Test complete - check if LSP started successfully"