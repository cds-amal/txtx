#!/bin/bash

# Test go-to-definition functionality

echo "Testing go-to-definition functionality..."

# Create a test workspace
TEST_DIR="/tmp/txtx-goto-test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/deploy"

# Create a YAML manifest with environment variables
cat > "$TEST_DIR/txtx.yml" << 'EOF'
version: "1.0"

environments:
  prod:
    SOLANA_PRIVATE_KEY: "aGVsbG93b3JsZGhlbGxvd29ybGRoZWxsb3dvcmxkaGVsbG93b3JsZGhlbGxvd29ybGRoZWxsb3dvcmxkaGVsbG93b3JsZA=="
    SOLANA_RPC_URL: "https://api.mainnet-beta.solana.com"
    
  dev:
    SOLANA_PRIVATE_KEY: "test123"
    SOLANA_RPC_URL: "https://api.devnet.solana.com"

runbooks:
  deploy:
    name: Deploy
    default_network: solana_devnet
    path: deploy

EOF

# Create a multi-file runbook
cat > "$TEST_DIR/deploy/main.tx" << 'EOF'
include "flows.tx"

variable "private_key" {
    value = env.SOLANA_PRIVATE_KEY
    sensitive = true
}

variable "rpc_url" {
    value = env.SOLANA_RPC_URL
}

variable "payer_address" {
    value = get_address_from_private_key(input.private_key)
}

action "show_balance" "solana::balance" {
    address = input.payer_address
    rpc_url = input.rpc_url
}

output "balance" {
    value = actions.show_balance.balance
}
EOF

cat > "$TEST_DIR/deploy/flows.tx" << 'EOF'
action "test_flow" "core::print" {
    message = "Using RPC: {{ input.rpc_url }}"
}
EOF

# Build the LSP server
echo "Building LSP server..."
cd /Users/amal/dev/tx/txtx
cargo build-cli 2>&1 | grep -E "(error|warning|Finished)" | tail -10

# Start the LSP server and test go-to-definition
echo -e "\nTesting go-to-definition requests..."

# Test 1: Go to definition for input.private_key (should find variable definition)
echo "Test 1: Finding definition of input.private_key"
RESULT=$(echo '{"jsonrpc":"2.0","id":1,"method":"textDocument/definition","params":{"textDocument":{"uri":"file://'"$TEST_DIR"'/deploy/main.tx"},"position":{"line":11,"character":36}}}' | \
  timeout 2s ./target/debug/txtx lsp 2>/dev/null | \
  grep -o '"range":{[^}]*}' | head -1)

if [[ "$RESULT" =~ "line" ]]; then
  echo "✓ Found definition for input.private_key"
else
  echo "✗ Failed to find definition for input.private_key"
fi

# Test 2: Go to definition for env.SOLANA_PRIVATE_KEY (should find in YAML)
echo -e "\nTest 2: Finding definition of env.SOLANA_PRIVATE_KEY"
RESULT=$(echo '{"jsonrpc":"2.0","id":2,"method":"textDocument/definition","params":{"textDocument":{"uri":"file://'"$TEST_DIR"'/deploy/main.tx"},"position":{"line":4,"character":20}}}' | \
  timeout 2s ./target/debug/txtx lsp 2>/dev/null | \
  grep -o '"uri":[^,]*' | head -1)

if [[ "$RESULT" =~ "txtx.yml" ]]; then
  echo "✓ Found definition in txtx.yml for env.SOLANA_PRIVATE_KEY"
else
  echo "✗ Failed to find definition in YAML for env.SOLANA_PRIVATE_KEY"
fi

# Test 3: Go to definition for input.rpc_url in flows.tx
echo -e "\nTest 3: Finding definition of input.rpc_url in flows.tx"
RESULT=$(echo '{"jsonrpc":"2.0","id":3,"method":"textDocument/definition","params":{"textDocument":{"uri":"file://'"$TEST_DIR"'/deploy/flows.tx"},"position":{"line":2,"character":30}}}' | \
  timeout 2s ./target/debug/txtx lsp 2>/dev/null | \
  grep -o '"uri":[^,]*' | head -1)

if [[ "$RESULT" =~ "main.tx" ]]; then
  echo "✓ Found definition in main.tx for input.rpc_url"
else
  echo "✗ Failed to find definition in main.tx for input.rpc_url"
fi

echo -e "\nTest complete!"