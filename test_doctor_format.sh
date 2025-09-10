#!/bin/bash

# Test the doctor command with different format options

echo "=== Testing pretty format (default) ==="
cargo run --bin txtx --no-default-features --features cli -- doctor --format pretty

echo -e "\n=== Testing quickfix format ==="
cargo run --bin txtx --no-default-features --features cli -- doctor --format quickfix

echo -e "\n=== Testing JSON format ==="
cargo run --bin txtx --no-default-features --features cli -- doctor --format json

echo -e "\n=== Testing auto format (should be pretty in terminal) ==="
cargo run --bin txtx --no-default-features --features cli -- doctor --format auto

echo -e "\n=== Testing auto format (should be quickfix when piped) ==="
cargo run --bin txtx --no-default-features --features cli -- doctor --format auto | cat