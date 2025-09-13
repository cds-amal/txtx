# LSP Test Fixtures

This directory contains test runbook files for testing LSP functionality, particularly hover and completion features.

## Test Files

- `test_hover.tx` - Basic hover functionality
- `test_hover_complete.tx` - Comprehensive hover tests for functions and actions
- `test_hover_functions.tx` - Function-specific hover tests
- `test_action_hover.tx` - Action hover functionality

## Features Tested

### Function Hover
- EVM functions: `evm::get_contract_from_foundry_project`, `evm::to_wei`, etc.
- Standard functions: `std::encode_hex`, `std::decode_hex`
- Addon-specific functions from Bitcoin, Stacks, Solana

### Action Hover
- `evm::deploy_contract`
- `evm::call_contract`
- Other action documentation

## Usage

These files are used to test:
- Hover provider in `crates/txtx-cli/src/cli/lsp/backend.rs`
- Function documentation generation in `crates/txtx-cli/src/cli/lsp/functions.rs`

## Adding New Tests

When adding new LSP test fixtures:
1. Use descriptive names indicating what LSP feature is tested
2. Include examples of all hover scenarios you want to test
3. Add comments marking hover test points