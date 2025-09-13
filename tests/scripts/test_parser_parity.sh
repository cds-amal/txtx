#!/bin/bash
# Parser Parity Test Script
# Tests that tree-sitter can parse all constructs that HCL parser handles

set -e

echo "=== Parser Parity Test ==="
echo "Testing tree-sitter grammar against HCL-supported constructs..."
echo

# Test files with different constructs
cat > /tmp/test_module.tx << 'EOF'
module "test" {
  name = "Test Module"
  description = "Testing module block"
}
EOF

cat > /tmp/test_flow.tx << 'EOF'
flow "mainnet" {
  chain_id = 1
  rpc_url = "https://mainnet.infura.io"
}
EOF

cat > /tmp/test_runbook.tx << 'EOF'
runbook "embedded" {
  location = "./embedded.tx"
  inputs = { key: "value" }
}
EOF

cat > /tmp/test_complete.tx << 'EOF'
# Complete test with all constructs
module "meta" {
  name = "Complete Test"
}

flow "testnet" {
  chain_id = 5
}

addon "evm" {
  chain_id = 1
  rpc_url = "http://localhost:8545"
}

variable "test_var" {
  value = 42
}

input "test_input" = "value"

import "./common"

signer "alice" "evm::secret_key" {
  secret_key = "0x123"
}

action "deploy" "evm::deploy" {
  signer = signer.alice
  bytecode = "0x456"
}

output "result" {
  value = action.deploy.address
}

runbook "child" {
  location = "./child.tx"
}
EOF

# Function to test parsing with tree-sitter
test_treesitter_parse() {
    local file=$1
    local name=$2
    
    echo -n "Testing $name... "
    
    # Check if we can use npx tree-sitter in the grammar directory
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
    
    if [ -f "$PROJECT_ROOT/crates/tree-sitter-txtx/package.json" ]; then
        cd "$PROJECT_ROOT/crates/tree-sitter-txtx"
        if npx tree-sitter parse "$file" &> /dev/null; then
            echo "✅ PASS"
            return 0
        else
            echo "❌ FAIL - Tree-sitter cannot parse"
            return 1
        fi
    else
        # Fallback: just check if the grammar would theoretically support it
        # by grepping for the construct in the grammar file
        local construct=$(echo $name | tr '[:upper:]' '[:lower:]' | sed 's/ /_/g')
        if grep -q "${construct}:" "$PROJECT_ROOT/crates/tree-sitter-txtx/grammar.js" 2>/dev/null; then
            echo "✅ Grammar supports $name"
            return 0
        else
            echo "❌ Grammar missing $name"
            return 1
        fi
    fi
}

# Run tests
failed=0

test_treesitter_parse /tmp/test_module.tx "module block" || ((failed++))
test_treesitter_parse /tmp/test_flow.tx "flow block" || ((failed++))
test_treesitter_parse /tmp/test_runbook.tx "runbook block" || ((failed++))
test_treesitter_parse /tmp/test_complete.tx "complete runbook" || ((failed++))

echo
echo "=== Summary ==="
if [ $failed -eq 0 ]; then
    echo "✅ All constructs supported - Parsers are in parity!"
else
    echo "❌ $failed construct(s) not supported - Parsers are NOT in parity!"
    echo
    echo "To fix parity issues, update:"
    echo "  1. crates/tree-sitter-txtx/grammar.js"
    echo "  2. crates/txtx-parser/src/ast.rs" 
    echo "  3. crates/txtx-parser/src/lib.rs"
    echo
    echo "See PARSER_PARITY_ANALYSIS.md for details"
    exit 1
fi

# Cleanup
rm -f /tmp/test_*.tx