#!/bin/bash

echo "Opening multi-file runbook in VSCode for testing..."
echo "=============================================="

# Navigate to the test fixture
cd tests/fixtures/doctor/separate-flows-file

# First verify doctor works
echo "1. Testing with txtx doctor (baseline):"
txtx doctor

echo -e "\n2. Opening in VSCode..."
echo "Please check if VSCode shows any errors in deploy/main.tx"
echo "Expected: No errors (same as doctor)"
echo "If errors appear, the LSP multi-file support is not working correctly"

# Open VSCode with the directory
code . --goto deploy/main.tx

echo -e "\nVSCode opened. Please check:"
echo "- Look for any red squiggles in main.tx"
echo "- Check the Problems panel (View -> Problems)"
echo "- Compare with doctor output above"