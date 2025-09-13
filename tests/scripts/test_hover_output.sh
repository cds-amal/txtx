#!/bin/bash

# This script tests the hover output for a specific function
# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

# Create a small test program that uses our hover functions
cat > /tmp/test_hover.rs << 'EOF'
use txtx_cli::cli::lsp::functions::get_function_hover;

fn main() {
    let functions = vec![
        "evm::get_contract_from_foundry_project",
        "evm::to_wei",
        "evm::address",
        "std::encode_hex",
    ];
    
    for func in functions {
        println!("=== {} ===", func);
        if let Some(hover) = get_function_hover(func) {
            println!("{}", hover);
        } else {
            println!("No documentation found");
        }
        println!();
    }
}
EOF

# Compile and run it
rustc /tmp/test_hover.rs -L target/release/deps \
    --extern txtx_cli=target/release/libtxtx_cli.rlib \
    --extern txtx_addon_kit=target/release/deps/libtxtx_addon_kit.rlib \
    --extern txtx_addon_network_evm=target/release/deps/libtxtx_addon_network_evm.rlib \
    --extern txtx_addon_network_svm=target/release/deps/libtxtx_addon_network_svm.rlib \
    --extern txtx_addon_network_bitcoin=target/release/deps/libtxtx_addon_network_bitcoin.rlib \
    --extern txtx_addon_telegram=target/release/deps/libtxtx_addon_telegram.rlib \
    --extern txtx_core=target/release/deps/libtxtx_core.rlib \
    --extern lazy_static=target/release/deps/liblazy_static.rlib \
    -o /tmp/test_hover 2>/dev/null

if [ -f /tmp/test_hover ]; then
    /tmp/test_hover
else
    echo "Failed to compile test program"
fi