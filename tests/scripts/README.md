# Test Scripts

This directory contains shell scripts for integration and end-to-end testing.

## Scripts

### LSP Tests
- `test-lsp-anywhere.sh` - Test LSP from any directory
- `test-lsp-direct.sh` - Direct LSP protocol testing
- `test-lsp-protocol.sh` - LSP protocol compliance tests

### Doctor Tests
- `test_doctor_direct.sh` - Direct doctor command testing
- `test_doctor_format.sh` - Doctor output formatting tests
- `test_doctor_simple.sh` - Simple doctor command tests

### Other Tests
- `test_hover_output.sh` - Hover output verification
- `test_parser_parity.sh` - Parser parity testing

## Running Scripts

```bash
# Make script executable if needed
chmod +x tests/scripts/test-lsp-direct.sh

# Run a specific test
./tests/scripts/test-lsp-direct.sh

# Run all LSP tests
for script in tests/scripts/test-lsp*.sh; do
    echo "Running $script"
    ./$script
done
```

## Writing New Scripts

When creating new test scripts:
1. Add proper shebang: `#!/bin/bash`
2. Set error handling: `set -e` 
3. Include descriptive comments
4. Return appropriate exit codes
5. Make the script executable