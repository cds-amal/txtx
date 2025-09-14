#!/bin/bash

# Test LSP with doctor validation rules
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKSPACE_DIR="$SCRIPT_DIR"

echo "Testing LSP doctor validation..."

# Build the LSP binary
echo "Building LSP..."
cd "$PROJECT_ROOT"
cargo build-cli-release

# Create test file that should trigger doctor validations
cat > "$WORKSPACE_DIR/test_doctor_lsp.tx" << 'EOF'
addon "evm" {
  chain_id = 1
  rpc_url = "https://eth.public-rpc.com"
}

// This should trigger "undefined input" error
action "deploy" "evm::deploy_contract" {
  chain_id = addon.evm.chain_id
  contract = inputs.undefined_contract  // undefined input
  deployer = inputs.undefined_deployer  // undefined input
}

// This should trigger "sensitive data" warning
output "private_key_exposed" {
  value = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
}

// Unknown action type
action "bad_action" "evm::unknown_action" {
  chain_id = 1
}
EOF

echo "Created test file with doctor validation issues"

# Run txtx doctor to verify the rules work
echo -e "\nRunning txtx doctor..."
"$PROJECT_ROOT/target/release/txtx" doctor test_doctor_lsp.tx

echo -e "\nTest complete."