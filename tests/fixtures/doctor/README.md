# Doctor Command Test Fixtures

This directory contains multi-file test fixtures for testing the `txtx doctor` command with file imports.

## Current Status

Most single-file test fixtures have been migrated to use `RunbookBuilder` in `crates/txtx-cli/tests/doctor_tests_builder.rs`. This provides:
- Better test maintainability
- Type-safe test construction
- No need to maintain separate fixture files

## Remaining Fixtures

### separate-flows-file/
- **Purpose**: Test multi-file runbooks with imports
- **Usage**: Used by `test_doctor_multi_file.rs` 
- **Why kept**: RunbookBuilder cannot easily simulate file imports and multi-file scenarios

## Migration Guide

If you need to add new doctor tests:
1. Use `RunbookBuilder` for single-file test scenarios
2. Only create fixtures for multi-file import scenarios
3. See `doctor_tests_builder.rs` for examples

Example using RunbookBuilder:
```rust
let mut builder = RunbookBuilder::new()
    .addon("evm", vec![])
    .action("send", "evm::send_eth")
    .input("to", "0x123")
    .input("value", "1000");

let result = builder.validate();
assert!(!result.success);
```