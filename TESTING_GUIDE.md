# txtx Testing Guide

This guide helps you write and run tests for the txtx project efficiently.

## Quick Start

### Running Tests

```bash
# Run all tests quickly (excludes supervisor UI and problematic packages)
cargo test-quick

# Run tests for a specific package
cargo test-cli              # CLI tests only, no supervisor UI
cargo test --package txtx-core

# Run specific test by name
cargo test test_undefined_variable

# Run tests with output
cargo test-quick -- --nocapture
```

### Common Test Commands

| Command | Description |
|---------|-------------|
| `cargo test-quick` | Run all tests excluding UI and stacks |
| `cargo test-cli` | Test CLI without supervisor UI |
| `cargo build-cli` | Build CLI without supervisor UI |
| `cargo test --package <name>` | Test specific package |

## Writing Tests

### Using RunbookBuilder (Recommended)

The `RunbookBuilder` provides a simple API for creating test scenarios:

```rust
use txtx_test_utils::RunbookBuilder;

#[test]
fn test_undefined_variable() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            action "test" "core::print" {
                message = input.undefined_var
            }
        "#)
        .validate();
        
    assert!(!result.success);
    assert!(result.errors.iter().any(|e| 
        e.to_string().contains("undefined variable")
    ));
}
```

### Multi-file Runbook Tests

```rust
#[test]
fn test_multi_file_runbook() {
    let result = RunbookBuilder::new()
        .with_file("main.tx", r#"
            include "flows.tx"
            variable "api_key" { value = env.API_KEY }
        "#)
        .with_file("flows.tx", r#"
            action "test" "core::print" { 
                message = input.api_key 
            }
        "#)
        .with_environment("test", vec![
            ("API_KEY", "secret123")
        ])
        .execute();
        
    assert!(result.success);
}
```

### Testing with Mocks (Coming Soon)

```rust
#[test]
fn test_blockchain_interaction() {
    let mock = MockBlockchain::new()
        .with_account("0x123", 1000000)
        .with_gas_price(100);
        
    let result = RunbookBuilder::new()
        .with_content(include_str!("../fixtures/deploy.tx"))
        .with_mock("ethereum", mock)
        .execute();
        
    assert!(result.success);
}
```

## Test Organization

### Directory Structure

```
crates/txtx-core/
├── src/
│   └── parser/
│       └── mod.rs
└── tests/          # Unit tests next to code
    └── parser/
        └── test_validation.rs

tests/              # Integration tests
├── fixtures/       # Test data files
└── integration/    # Cross-crate tests
```

### Test Categories

1. **Unit Tests**: Test individual functions/modules
   - Place in `tests/` subdirectory next to code
   - Use `#[cfg(test)]` modules for private API tests

2. **Integration Tests**: Test multiple components
   - Place in workspace-level `tests/` directory
   - Use `RunbookBuilder` for complex scenarios

3. **Snapshot Tests**: Test complex outputs
   - Use for doctor command output
   - Use for LSP responses
   - Review changes with `cargo insta review`

## Common Patterns

### Testing Validation Errors

```rust
#[test]
fn test_validation_error() {
    let runbook = r#"
        action "deploy" "evm::deploy_contract" {
            signer = undefined_signer
        }
    "#;
    
    let result = validate_runbook(runbook);
    assert_error!(result, ValidationError::UndefinedSigner { 
        name: "undefined_signer".to_string() 
    });
}
```

### Testing Parser Errors

```rust
#[test]
fn test_parse_error() {
    let invalid = r#"
        action "test" {  // Missing construct type
            foo = "bar"
        }
    "#;
    
    let result = parse_runbook(invalid);
    assert!(result.is_err());
}
```

## Troubleshooting

### Build Failures

If you see supervisor UI build errors:
```bash
# Use CLI-only commands
cargo test-cli
cargo build-cli
```

### Slow Tests

For faster iteration:
```bash
# Run only your specific test
cargo test test_my_function

# Skip integration tests
cargo test --lib
```

### Test Output

To see println! output:
```bash
cargo test -- --nocapture
```

## Best Practices

1. **Use Descriptive Names**: `test_undefined_variable_in_action_input` not `test1`
2. **Test One Thing**: Each test should verify a single behavior
3. **Use Builders**: Prefer `RunbookBuilder` over string manipulation
4. **Mock External Dependencies**: Don't rely on network/filesystem
5. **Keep Tests Fast**: Mock slow operations
6. **Use Snapshots**: For complex outputs that change frequently

## Next Steps

- See `QA_IMPROVEMENT_PLAN.md` for upcoming test infrastructure improvements
- Check existing tests in `crates/txtx-core/src/tests/` for examples
- Join #testing channel for help and discussions