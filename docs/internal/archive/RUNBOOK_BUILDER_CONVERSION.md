# RunbookBuilder API Conversion Guide

This document outlines the conversion of all doctor and LSP tests to use the new RunbookBuilder API.

## Overview

The RunbookBuilder API provides a fluent interface for constructing and validating runbooks in tests, reducing boilerplate and improving readability.

## Conversion Summary

### Doctor Unit Tests
**Location**: `crates/txtx-cli/src/cli/doctor/tests/mod.rs`
**Converted to**: `mod_converted.rs`

Key improvements:
- Reduced code by ~40% 
- More readable test structure
- Direct validation without helper functions

### Doctor Analyzer Rules Tests  
**Location**: `crates/txtx-cli/src/cli/doctor/analyzer/rules.rs`
**Converted to**: `rules_converted.rs`

Key improvements:
- Simplified test context creation
- Direct testing of validation rules
- Added new test cases for edge cases

### Doctor Integration Tests
**Location**: `crates/txtx-cli/tests/doctor_tests.rs`
**Converted to**: `doctor_tests_converted.rs`

Key improvements:
- No longer need to spawn txtx binary
- Direct validation instead of JSON parsing
- Faster test execution

### LSP Diagnostic Tests
**Location**: `crates/txtx-cli/src/cli/lsp/diagnostics_enhanced.rs`
**Converted to**: `diagnostics_enhanced_converted.rs`

Key improvements:
- Simplified manifest creation
- Better environment handling
- More concise assertions

## API Examples

### Before (Old Style)
```rust
fn validate_fixture(content: &str) -> ValidationResult {
    let mut result = ValidationResult {
        errors: Vec::new(),
        warnings: Vec::new(),
        suggestions: Vec::new(),
    };
    let addons = addon_registry::get_all_addons();
    let addon_specs = addon_registry::extract_addon_specifications(&addons);
    let _ = hcl_validator::validate_with_hcl_and_addons(content, &mut result, "test.tx", addon_specs);
    result
}

#[test]
fn test_undefined_action() {
    let runbook = r#"
        output "bad" {
            value = action.nonexistent.result
        }
    "#;
    
    let result = validate_fixture(runbook);
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].message.contains("undefined action"));
}
```

### After (RunbookBuilder)
```rust
#[test]
fn test_undefined_action() {
    let result = RunbookBuilder::new()
        .output("bad", "action.nonexistent.result")
        .validate();
    
    assert_validation_error!(result, "undefined action");
}
```

## Common Patterns

### 1. Basic Validation
```rust
let result = RunbookBuilder::new()
    .addon("evm", vec![("network_id", "1")])
    .action("deploy", "evm::deploy_contract")
        .input("contract", "./Token.sol")
    .validate();

assert_success!(result);
```

### 2. Environment Variables
```rust
let result = RunbookBuilder::new()
    .variable("api_key", "env.API_KEY")
    .with_environment("production", vec![
        ("API_KEY", "prod_key_123")
    ])
    .validate();
```

### 3. Multi-file Runbooks
```rust
let result = RunbookBuilder::new()
    .with_file("main.tx", "include 'common.tx'")
    .with_file("common.tx", "addon 'evm' { network_id = 1 }")
    .validate();
```

### 4. CLI Input Override
```rust
let result = RunbookBuilder::new()
    .variable("key", "env.KEY")
    .with_environment("test", vec![("KEY", "env_value")])
    .with_cli_input("key", "cli_value")
    .validate();
```

## Assertion Macros

The test utils provide convenient assertion macros:

- `assert_success!(result)` - Assert validation succeeded
- `assert_validation_error!(result, "pattern")` - Assert error containing pattern
- `assert_validation_warning!(result, "pattern")` - Assert warning containing pattern

## Migration Steps

To convert existing tests:

1. Replace manual validation setup with `RunbookBuilder::new()`
2. Use builder methods instead of string concatenation
3. Replace error checking with assertion macros
4. Remove boilerplate helper functions
5. Run tests to verify behavior is preserved

## Benefits

1. **Reduced Code**: 40-60% less code in most tests
2. **Better Readability**: Fluent API clearly shows test structure
3. **Type Safety**: Builder pattern prevents invalid test construction
4. **Faster Development**: Less boilerplate to write
5. **Consistent Patterns**: All tests follow same structure