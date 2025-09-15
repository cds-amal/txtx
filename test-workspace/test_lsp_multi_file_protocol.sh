#!/bin/bash

# Test LSP with multi-file runbook using proper protocol

set -e

echo "Testing LSP with multi-file runbook (separate-flows-file)"
echo "========================================================="

# Get absolute path to test fixture
TEST_DIR=$(cd ../tests/fixtures/doctor/separate-flows-file && pwd)
echo "Test directory: $TEST_DIR"

# Build the LSP if needed
echo -e "\nBuilding txtx CLI..."
cd /Users/amal/dev/tx/txtx
cargo build-cli 2>/dev/null || echo "Build warnings ignored"

# Create named pipes for communication
PIPE_IN=$(mktemp -u)
PIPE_OUT=$(mktemp -u)
mkfifo "$PIPE_IN"
mkfifo "$PIPE_OUT"

# Start LSP server
echo -e "\nStarting LSP server..."
/Users/amal/dev/tx/txtx/target/debug/txtx lsp < "$PIPE_IN" > "$PIPE_OUT" 2>lsp_errors.log &
LSP_PID=$!

# Function to send request
send_request() {
    local content="$1"
    local length=${#content}
    printf "Content-Length: %d\r\n\r\n%s" "$length" "$content" > "$PIPE_IN"
}

# Function to read response
read_response() {
    timeout 2 cat "$PIPE_OUT" 2>/dev/null || true
}

# Give LSP time to start
sleep 1

# 1. Send initialize request
echo -e "\n1. Sending initialize request..."
INIT_REQ='{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "processId": '$$',
    "rootUri": "file://'$TEST_DIR'",
    "capabilities": {}
  }
}'
send_request "$INIT_REQ"
INIT_RESP=$(read_response)
echo "Initialize response: $INIT_RESP"

# 2. Send initialized notification
echo -e "\n2. Sending initialized notification..."
send_request '{"jsonrpc":"2.0","method":"initialized","params":{}}'
# No response expected for initialized notification
echo "Initialized notification sent (no response expected)"

# 3. Open main.tx file
echo -e "\n3. Opening deploy/main.tx..."
MAIN_CONTENT=$(cat "$TEST_DIR/deploy/main.tx" | jq -Rs .)
OPEN_MAIN='{
  "jsonrpc": "2.0",
  "method": "textDocument/didOpen",
  "params": {
    "textDocument": {
      "uri": "file://'$TEST_DIR'/deploy/main.tx",
      "languageId": "txtx",
      "version": 1,
      "text": '"$MAIN_CONTENT"'
    }
  }
}'
send_request "$OPEN_MAIN"
sleep 2

# 4. Read any diagnostics
echo -e "\n4. Reading diagnostics..."
DIAG=$(read_response)
echo "Diagnostics response: $DIAG"

# 5. Also open flows.tx to see if it helps
echo -e "\n5. Opening deploy/flows.tx..."
FLOWS_CONTENT=$(cat "$TEST_DIR/deploy/flows.tx" | jq -Rs .)
OPEN_FLOWS='{
  "jsonrpc": "2.0",
  "method": "textDocument/didOpen",
  "params": {
    "textDocument": {
      "uri": "file://'$TEST_DIR'/deploy/flows.tx",
      "languageId": "txtx",
      "version": 1,
      "text": '"$FLOWS_CONTENT"'
    }
  }
}'
send_request "$OPEN_FLOWS"
sleep 2

# Read any additional diagnostics
echo -e "\n6. Reading additional diagnostics..."
DIAG2=$(read_response)
echo "Additional diagnostics: $DIAG2"

# Shutdown
echo -e "\n7. Shutting down LSP..."
send_request '{"jsonrpc":"2.0","id":2,"method":"shutdown"}'
sleep 1
send_request '{"jsonrpc":"2.0","method":"exit"}'

# Cleanup
kill $LSP_PID 2>/dev/null || true
rm -f "$PIPE_IN" "$PIPE_OUT"

# Show any errors
if [ -f lsp_errors.log ] && [ -s lsp_errors.log ]; then
    echo -e "\nLSP Errors:"
    cat lsp_errors.log
fi
rm -f lsp_errors.log

echo -e "\nTest complete!"