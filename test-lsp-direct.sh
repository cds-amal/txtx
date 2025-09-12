#!/bin/bash

# Test LSP server directly with JSON-RPC

echo "Testing LSP server directly..."

# Create a temporary file for communication
TMPFILE=$(mktemp)

# Test workspace
WORKSPACE_DIR="$(pwd)/test-workspace"

# Start the LSP server
echo "Starting LSP server..."

# Create the JSON-RPC request
cat > $TMPFILE << 'EOF'
Content-Length: 209

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file:///home/amal/dev/tx/txtx/test-workspace","capabilities":{"textDocument":{"definition":{"dynamicRegistration":true}}}}}
EOF

# Send initialize request
echo "Sending initialize request..."
cat $TMPFILE | tyty lsp > /tmp/lsp-response.txt 2>&1 &
LSP_PID=$!

sleep 2

# Check if server is running
if ps -p $LSP_PID > /dev/null; then
    echo "LSP server is running (PID: $LSP_PID)"
    
    # Send didOpen for txtx.yml
    echo "Opening txtx.yml..."
    cat << 'EOF' | nc -w1 localhost 8080
Content-Length: 458

{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///home/amal/dev/tx/txtx/test-workspace/txtx.yml","languageId":"yaml","version":1,"text":"name: test-workspace\nid: test-workspace\n\nrunbooks:\n  - name: deploy\n    location: deploy.tx\n    description: Deploy contract\n\nenvironments:\n  default:\n    contract_address: \"0x1234567890abcdef\"\n    private_key: \"test_private_key\"\n    api_url: \"https://api.test.com\""}}}
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