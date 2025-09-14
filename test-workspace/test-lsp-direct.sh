#!/bin/bash

# Test LSP directly with protocol messages

WORKSPACE_DIR="/Users/amal/dev/tx/txtx/test-workspace"
LSP_BIN="../target/release/txtx lsp"

echo "Starting LSP test..."

# Create a simple JSON-RPC request to initialize the LSP
INIT_REQUEST=$(cat <<EOF
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
  "capabilities": {
    "textDocument": {
      "synchronization": {
        "dynamicRegistration": false,
        "didSave": true
      },
      "publishDiagnostics": {
        "relatedInformation": true
      }
    }
  },
  "rootUri": "file://$WORKSPACE_DIR",
  "workspaceFolders": [{
    "uri": "file://$WORKSPACE_DIR",
    "name": "test-workspace"
  }]
}}
EOF
)

# Initialized notification
INITIALIZED=$(cat <<EOF
{"jsonrpc": "2.0", "method": "initialized", "params": {}}
EOF
)

# Open document request
OPEN_DOC=$(cat <<EOF
{"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
  "textDocument": {
    "uri": "file://$WORKSPACE_DIR/test_validation.tx",
    "languageId": "txtx",
    "version": 1,
    "text": "$(cat test_validation.tx | sed 's/"/\\"/g' | sed ':a;N;$!ba;s/\n/\\n/g')"
  }
}}
EOF
)

# Function to send LSP message with content-length header
send_message() {
    local msg="$1"
    local len=${#msg}
    printf "Content-Length: %d\r\n\r\n%s" "$len" "$msg"
}

# Run the LSP and send messages
echo "Sending initialize request..."
(
    send_message "$INIT_REQUEST"
    sleep 1
    send_message "$INITIALIZED"
    sleep 1
    echo "Sending didOpen for test_validation.tx..."
    send_message "$OPEN_DOC"
    sleep 2
) | bash -c "$LSP_BIN" 2>&1 | tee lsp-output.log

echo "LSP test complete. Check lsp-output.log for results."