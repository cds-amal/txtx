#!/bin/bash

# Debug LSP go-to-definition

TEST_DIR="/tmp/txtx-goto-test"
cd "$TEST_DIR"

# Initialize LSP with proper request/response format
cat << 'EOF' | RUST_LOG=debug /Users/amal/dev/tx/txtx/target/debug/txtx lsp 2>&1 | tee /tmp/lsp_output.log
Content-Length: 58

{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}
Content-Length: 220

{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///tmp/txtx-goto-test/deploy/main.tx","languageId":"txtx","version":1,"text":"include \"flows.tx\"\n\nvariable \"private_key\" {\n    value = env.SOLANA_PRIVATE_KEY\n    sensitive = true\n}\n\nvariable \"rpc_url\" {\n    value = env.SOLANA_RPC_URL\n}\n\nvariable \"payer_address\" {\n    value = get_address_from_private_key(input.private_key)\n}\n\naction \"show_balance\" \"solana::balance\" {\n    address = input.payer_address\n    rpc_url = input.rpc_url\n}\n\noutput \"balance\" {\n    value = actions.show_balance.balance\n}"}}}
Content-Length: 164

{"jsonrpc":"2.0","id":1,"method":"textDocument/definition","params":{"textDocument":{"uri":"file:///tmp/txtx-goto-test/deploy/main.tx"},"position":{"line":11,"character":36}}}
Content-Length: 50

{"jsonrpc":"2.0","id":2,"method":"shutdown","params":null}
EOF

echo -e "\n\nAnalyzing output..."
grep -A5 -B5 "definition" /tmp/lsp_output.log