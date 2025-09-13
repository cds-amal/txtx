# Integration Tests

This directory contains Rust integration tests that test multiple components working together.

## Test Files

- `test_addon_functions.rs` - Tests for addon function integration
- `test_doc_links.rs` - Documentation link generation tests
- `test_doctor_pattern.rs` - Doctor command pattern matching
- `test_lsp_hover.rs` - LSP hover functionality integration
- `test_lsp_hover_functions.rs` - LSP function hover integration

## Running Tests

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test test_lsp_hover

# Run with output
cargo test --test test_addon_functions -- --nocapture
```

## Writing Integration Tests

Integration tests should:
1. Test interactions between multiple components
2. Use realistic test scenarios
3. Verify end-to-end functionality
4. Be independent and not rely on external state

## Test Organization

Each test file should:
- Focus on a specific feature area
- Include both positive and negative test cases
- Use descriptive test function names
- Include documentation comments