#!/bin/bash

# Test the LSP server with a proper initialize request
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
WORKSPACE_DIR="$PROJECT_ROOT/test-workspace"

INIT_REQUEST=$(cat <<EOF
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file://${WORKSPACE_DIR}","capabilities":{"textDocument":{"definition":{"dynamicRegistration":true}}}}}
EOF
)

CONTENT_LENGTH=${#INIT_REQUEST}

cat << EOF | "$PROJECT_ROOT/target/debug/txtx" lsp 2>&1 &
Content-Length: $CONTENT_LENGTH

$INIT_REQUEST
EOF

LSP_PID=$!

# Give it time to respond
sleep 1

# Kill the server
kill $LSP_PID 2>/dev/null

echo "LSP test completed"