#!/bin/bash

# This script tests the LSP protocol by sending messages directly to the txtx LSP server

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting txtx LSP protocol test...${NC}"

# Get absolute path to test file
TEST_FILE="$(cd "$(dirname "$0")" && pwd)/test_validation.tx"
echo -e "${YELLOW}Test file: $TEST_FILE${NC}"

# Function to send LSP request and read response
send_lsp_request() {
    local request="$1"
    local content_length=${#request}
    
    # Send header and content
    printf "Content-Length: %d\r\n\r\n%s" "$content_length" "$request"
}

# Create a named pipe for communication
PIPE_IN=/tmp/txtx-lsp-in.$$
PIPE_OUT=/tmp/txtx-lsp-out.$$
mkfifo "$PIPE_IN" "$PIPE_OUT"

# Clean up on exit
cleanup() {
    echo -e "\n${YELLOW}Cleaning up...${NC}"
    rm -f "$PIPE_IN" "$PIPE_OUT"
    kill $LSP_PID 2>/dev/null
}
trap cleanup EXIT

# Start LSP server
echo -e "${GREEN}Starting LSP server...${NC}"
../target/debug/txtx lsp < "$PIPE_IN" > "$PIPE_OUT" 2>lsp-error.log &
LSP_PID=$!

# Give it time to start
sleep 1

# Connect to pipes
exec 3>"$PIPE_IN"
exec 4<"$PIPE_OUT"

# Read responses in background
(
    while IFS= read -r line <&4; do
        echo -e "${GREEN}Response:${NC} $line"
    done
) &
READER_PID=$!

echo -e "\n${YELLOW}Sending initialize request...${NC}"
send_lsp_request '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "processId": null,
    "rootUri": "file:///Users/amal/dev/tx/txtx/test-workspace",
    "capabilities": {
      "textDocument": {
        "publishDiagnostics": {
          "relatedInformation": true
        }
      }
    }
  }
}' >&3

sleep 2

echo -e "\n${YELLOW}Sending initialized notification...${NC}"
send_lsp_request '{
  "jsonrpc": "2.0",
  "method": "initialized",
  "params": {}
}' >&3

sleep 1

echo -e "\n${YELLOW}Opening test file with validation errors...${NC}"
send_lsp_request '{
  "jsonrpc": "2.0",
  "method": "textDocument/didOpen",
  "params": {
    "textDocument": {
      "uri": "file://'"$TEST_FILE"'",
      "languageId": "txtx",
      "version": 1,
      "text": "addon \"evm\" { \n  chain_id = env.VITE_EVM_CHAIN_ID\n}\nvariable \"test\" {\n  chain_id = flow.chain_id\n}\n\nvariable \"computed\" {\n  value = 1 + 2\n}\n\noutput \"result\" {\n  value = action.nonexistent.tx_hash\n}\n"
    }
  }
}' >&3

echo -e "\n${YELLOW}Waiting for diagnostics...${NC}"
# Try to read any messages from the LSP server
for i in {1..3}; do
    if read -t 1 -r line <&3 2>/dev/null; then
        echo -e "${GREEN}Diagnostic:${NC} $line"
    fi
done

echo -e "\n${YELLOW}Sending shutdown request...${NC}"
send_lsp_request '{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "shutdown"
}' >&3

sleep 1

echo -e "\n${YELLOW}Sending exit notification...${NC}"
send_lsp_request '{
  "jsonrpc": "2.0",
  "method": "exit"
}' >&3

# Wait a bit for responses
sleep 2

# Kill reader
kill $READER_PID 2>/dev/null

echo -e "\n${GREEN}Test complete!${NC}"
echo -e "${YELLOW}Check lsp-error.log for any error output${NC}"