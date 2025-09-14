#!/bin/bash

# Simple LSP test to check validation
echo "Testing LSP validation directly..."

# Test workspace
WORKSPACE_DIR="/Users/amal/dev/tx/txtx/test-workspace"
TEST_FILE="$WORKSPACE_DIR/test_validation.tx"

# Read the test file content
FILE_CONTENT=$(cat "$TEST_FILE" | sed 's/"/\\"/g' | sed ':a;N;$!ba;s/\n/\\n/g')

# Create initialize request
cat <<EOF | txtx lsp
Content-Length: 185

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file://${WORKSPACE_DIR}","capabilities":{"textDocument":{"publishDiagnostics":{"relatedInformation":true}}}}}
EOF

# Wait a moment
sleep 1

# Send initialized notification
echo -e "Content-Length: 52\r\n\r\n{\"jsonrpc\":2.0\",\"method\":\"initialized\",\"params\":{}}" | txtx lsp

# Send didOpen for the test file
OPEN_REQUEST=$(cat <<EOF
{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file://${TEST_FILE}","languageId":"txtx","version":1,"text":"${FILE_CONTENT}"}}}
EOF
)

CONTENT_LENGTH=${#OPEN_REQUEST}

echo -e "Content-Length: $CONTENT_LENGTH\r\n\r\n$OPEN_REQUEST" | txtx lsp

# Give it time to process
sleep 2

echo "Done. Check if any diagnostics were published."