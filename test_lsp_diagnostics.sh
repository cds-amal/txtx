#!/bin/bash

# Test LSP diagnostics functionality

echo "Building txtx LSP..."
cargo build --package txtx-cli --no-default-features --features cli --release

echo -e "\nTesting LSP diagnostics with test file..."
echo -e "Starting LSP server..."

# Run the LSP in the background and capture its output
./target/release/txtx lsp 2>&1 &
LSP_PID=$!

# Give the LSP time to start
sleep 2

echo -e "\nSending didOpen notification with test file content..."

# Send LSP messages to test diagnostics
# This would normally be done by the editor
cat << 'EOF' | nc localhost 9257
Content-Length: 59

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
Content-Length: 52

{"jsonrpc":"2.0","method":"initialized","params":{}}
Content-Length: 366

{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///Users/amal/dev/tx/txtx/test_lsp_diagnostics.tx","languageId":"txtx","version":1,"text":"// Test file with errors\n\naction \"test\" \"evm::deploy_contract\" {\n  description = \"Deploy a test contract\"\n  \n\n// Invalid syntax\noutput \"result {\n  value = action.test.result\n}"}}}
EOF

echo -e "\nWaiting for diagnostics..."
sleep 3

# Kill the LSP server
kill $LSP_PID 2>/dev/null

echo -e "\nTest complete. Check the LSP output above for diagnostics."