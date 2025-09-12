#!/bin/bash

# Test the LSP server with a proper initialize request

cat << 'EOF' | ../target/debug/txtx lsp 2>&1 &
Content-Length: 205

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file:///home/amal/dev/tx/txtx/test-workspace","capabilities":{"textDocument":{"definition":{"dynamicRegistration":true}}}}}
EOF

LSP_PID=$!

# Give it time to respond
sleep 1

# Kill the server
kill $LSP_PID 2>/dev/null

echo "LSP test completed"