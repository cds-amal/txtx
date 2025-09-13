#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "Direct test of doctor command..."

# Go to the fixture directory
FIXTURE_DIR="$PROJECT_ROOT/addons/evm/fixtures/kitchenSink"
if [ -d "$FIXTURE_DIR" ]; then
    cd "$FIXTURE_DIR"
    
    # Run doctor and capture both stdout and stderr
    echo "Running: txtx doctor problematic_test"
    "$PROJECT_ROOT/target/debug/txtx" doctor problematic_test 2>&1
    
    echo ""
    echo "Exit code: $?"
    echo ""
    
    # Also try without specifying a runbook
    echo "Running: txtx doctor"
    "$PROJECT_ROOT/target/debug/txtx" doctor 2>&1 | head -30
    
    echo ""
    echo "Content of problematic_test runbook:"
    cat runbooks/problematic_test/main.tx 2>/dev/null || echo "File not found"
else
    echo "Fixture directory not found: $FIXTURE_DIR"
    echo "Skipping doctor command test"
fi