#!/bin/bash

# Test LSP server directly with JSON-RPC
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "Testing LSP server directly..."

# Create a temporary file for communication
TMPFILE=$(mktemp)

# Test workspace
WORKSPACE_DIR="$PROJECT_ROOT/test-workspace"

# Start the LSP server
echo "Starting LSP server..."

# Create the JSON-RPC request with dynamic path
INIT_REQUEST=$(cat <<EOF
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file://${WORKSPACE_DIR}","capabilities":{"textDocument":{"definition":{"dynamicRegistration":true}}}}}
EOF
)

# Calculate content length
CONTENT_LENGTH=${#INIT_REQUEST}

# Create the full request
cat > $TMPFILE << EOF
Content-Length: $CONTENT_LENGTH

$INIT_REQUEST
EOF

# Send initialize request
echo "Sending initialize request..."
cat $TMPFILE | txtx lsp > /tmp/lsp-response.txt 2>&1 &
LSP_PID=$!

sleep 2

# Check if server is running
if ps -p $LSP_PID > /dev/null; then
    echo "LSP server is running (PID: $LSP_PID)"
    
    # Send didOpen for txtx.yml
    echo "Opening txtx.yml..."
    
    OPEN_REQUEST=$(cat <<EOF
{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file://${WORKSPACE_DIR}/txtx.yml","languageId":"yaml","version":1,"text":"name: test-workspace\nid: test-workspace\n\nrunbooks:\n  - name: deploy\n    location: deploy.tx\n    description: Deploy contract\n\nenvironments:\n  default:\n    contract_address: \"0x1234567890abcdef\"\n    private_key: \"test_private_key\"\n    api_url: \"https://api.test.com\""}}}
EOF
)
    
    OPEN_LENGTH=${#OPEN_REQUEST}
    
    cat << EOF | nc -w1 localhost 8080
Content-Length: $OPEN_LENGTH

$OPEN_REQUEST
EOF
    
    kill $LSP_PID
else
    echo "LSP server failed to start"
fi

# Show response
echo "Response:"
cat /tmp/lsp-response.txt

# Cleanup
rm $TMPFILE