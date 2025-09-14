#!/bin/bash

# Test LSP with doctor validation rules
echo "Testing LSP with doctor validation rules..."

# Test workspace
WORKSPACE_DIR="/Users/amal/dev/tx/txtx/test-workspace"
TEST_FILE="$WORKSPACE_DIR/test_doctor_validation.tx"

# Read the test file content - macOS compatible
FILE_CONTENT=$(cat "$TEST_FILE" | sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}' | sed '$ s/\\n$//')

# Run LSP and capture output
(
# Initialize
cat <<EOF
Content-Length: 185

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file://${WORKSPACE_DIR}","capabilities":{"textDocument":{"publishDiagnostics":{"relatedInformation":true}}}}}
EOF

sleep 0.5

# Initialized
echo -e "Content-Length: 52\r\n\r\n{\"jsonrpc\":\"2.0\",\"method\":\"initialized\",\"params\":{}}"

sleep 0.5

# Open document
OPEN_REQUEST="{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/didOpen\",\"params\":{\"textDocument\":{\"uri\":\"file://${TEST_FILE}\",\"languageId\":\"txtx\",\"version\":1,\"text\":\"${FILE_CONTENT}\"}}}"
CONTENT_LENGTH=${#OPEN_REQUEST}
echo -e "Content-Length: $CONTENT_LENGTH\r\n\r\n$OPEN_REQUEST"

sleep 1

# Shutdown
echo -e "Content-Length: 48\r\n\r\n{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"shutdown\"}"

sleep 0.5

# Exit
echo -e "Content-Length: 46\r\n\r\n{\"jsonrpc\":\"2.0\",\"method\":\"exit\",\"params\":null}"

) | txtx lsp 2>&1 | grep -E "(publishDiagnostics|diagnostic|error|warning|undefined|sensitive)" | grep -v "Starting txtx" | grep -v "Received"

echo -e "\nDone. Check output above for diagnostics."