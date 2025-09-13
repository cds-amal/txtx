# txtx Test Suite

This directory contains the comprehensive test suite for the txtx project.

## Directory Structure

```
tests/
├── fixtures/           # Test data files (.tx runbooks)
│   ├── doctor/        # Doctor command test files
│   ├── lsp/           # LSP and hover functionality test files
│   └── runbooks/      # General runbook test files
├── scripts/           # Shell-based integration tests
├── integration/       # Directory for integration test documentation
└── *.rs               # Workspace-level integration tests
```

## Test Categories

### Fixtures
Test data files organized by feature area:
- **doctor/**: Test cases for the `txtx doctor` command validation
- **lsp/**: Test cases for LSP hover and completion features
- **runbooks/**: General runbook execution test cases

### Scripts
Shell scripts for end-to-end testing:
- LSP protocol tests
- Doctor command tests
- Parser parity tests

### Integration
Rust integration tests that test multiple components together.

## Running Tests

```bash
# Run all Rust tests (without supervisor UI which requires npm)
cargo test --no-default-features --features cli

# Run specific package tests
cargo test --package txtx-cli --no-default-features --features cli

# Run a specific integration test
cargo test --package txtx-cli --test lsp_hover_test --no-default-features --features cli

# List available test targets
cargo test --package txtx-cli --no-default-features --features cli --list

# Run shell script tests
./tests/scripts/test-lsp-direct.sh
```

## Adding New Tests

1. Place test data files (.tx) in the appropriate `fixtures/` subdirectory
2. Place shell scripts in `scripts/`
3. Place Rust integration tests in `integration/`
4. Update this README if adding new test categories