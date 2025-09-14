#!/bin/bash

# Test direct doctor command functionality
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Testing doctor command..."

# Create a test file with issues
cat > "$SCRIPT_DIR/test_direct.tx" << 'EOF'
addon "evm" {
  chain_id = 1
  rpc_url = "https://eth.public-rpc.com"
}

// Test 1: Unknown action type
action "bad" "evm::does_not_exist" {
  chain_id = 1
}

// Test 2: Valid action to compare
action "send" "evm::send_eth" {
  chain_id = addon.evm.chain_id
  to = "0x742d35Cc6634C0532925a3b844Bc9e7595f89ed3"
  value = "1000000000000000000"
  private_key = "test_key"
}
EOF

echo "Running doctor on test file..."
"$PROJECT_ROOT/target/release/txtx" doctor test_direct.tx 2>&1

# Clean up
rm -f "$SCRIPT_DIR/test_direct.tx"

echo -e "\nTest complete."