#!/bin/bash

echo "Direct test of doctor command..."

# Go to the fixture directory
cd /Users/amal/dev/tx/txtx/addons/evm/fixtures/kitchenSink

# Run doctor and capture both stdout and stderr
echo "Running: txtx doctor problematic_test"
/Users/amal/dev/tx/txtx/target/debug/txtx doctor problematic_test 2>&1

echo ""
echo "Exit code: $?"
echo ""

# Also try without specifying a runbook
echo "Running: txtx doctor"
/Users/amal/dev/tx/txtx/target/debug/txtx doctor 2>&1 | head -30

echo ""
echo "Content of problematic_test runbook:"
cat runbooks/problematic_test/main.tx 2>/dev/null || echo "File not found"