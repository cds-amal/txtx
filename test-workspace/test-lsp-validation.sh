#!/bin/bash

# Test LSP validation functionality
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKSPACE_DIR="$SCRIPT_DIR"

echo "Testing LSP validation with test_validation.tx..."

# Create a FIFO for bidirectional communication
FIFO_IN=$(mktemp -u)
FIFO_OUT=$(mktemp -u)
mkfifo "$FIFO_IN" "$FIFO_OUT"

# Start LSP server with input/output redirection
"$PROJECT_ROOT/target/release/txtx" lsp < "$FIFO_IN" > "$FIFO_OUT" 2>&1 &
LSP_PID=$!

# Function to send LSP message
send_message() {
    local msg="$1"
    local len=${#msg}
    printf "Content-Length: %d\r\n\r\n%s" "$len" "$msg" > "$FIFO_IN"
}

# Function to read response
read_response() {
    timeout 2 cat "$FIFO_OUT" 2>/dev/null || true
}

# Initialize request
INIT_REQUEST=$(cat <<EOF
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":$$,"rootUri":"file://$WORKSPACE_DIR","capabilities":{"textDocument":{"publishDiagnostics":{"relatedInformation":true}}}}}
EOF
)

echo "Sending initialize request..."
send_message "$INIT_REQUEST"
RESPONSE=$(read_response)
echo "Initialize response:"
echo "$RESPONSE" | grep -o '{.*}' | jq . 2>/dev/null || echo "$RESPONSE"

# Send initialized notification
INITIALIZED=$(cat <<EOF
{"jsonrpc":"2.0","method":"initialized","params":{}}
EOF
)

send_message "$INITIALIZED"
sleep 0.5

# Read the test_validation.tx file
FILE_CONTENT=$(cat test_validation.tx | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | sed ':a;N;$!ba;s/\n/\\n/g')

# Open document request
OPEN_REQUEST=$(cat <<EOF
{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file://$WORKSPACE_DIR/test_validation.tx","languageId":"txtx","version":1,"text":"$FILE_CONTENT"}}}
EOF
)

echo -e "\nOpening test_validation.tx..."
send_message "$OPEN_REQUEST"

# Wait for diagnostics
echo -e "\nWaiting for diagnostics..."
sleep 2

# Read any pending output
DIAGNOSTICS=$(read_response)
echo -e "\nDiagnostics response:"
echo "$DIAGNOSTICS" | grep -o '{.*}' | jq . 2>/dev/null || echo "$DIAGNOSTICS"

# Shutdown request
SHUTDOWN=$(cat <<EOF
{"jsonrpc":"2.0","id":2,"method":"shutdown","params":null}
EOF
)

send_message "$SHUTDOWN"
sleep 0.5

# Exit notification
EXIT=$(cat <<EOF
{"jsonrpc":"2.0","method":"exit","params":null}
EOF
)

send_message "$EXIT"

# Cleanup
kill $LSP_PID 2>/dev/null
rm -f "$FIFO_IN" "$FIFO_OUT"

echo -e "\nTest complete."