# txtx-test-utils

Testing utilities for txtx runbooks, providing a fluent builder API and validation helpers.

## Overview

`txtx-test-utils` simplifies writing tests for txtx runbooks by providing:
- **RunbookBuilder**: A fluent API for constructing test runbooks
- **Validation modes**: HCL-only vs full manifest validation
- **Test assertions**: Helpers for checking validation results
- **Mock support**: Testing with simulated blockchain responses

## Validation Modes

### 1. HCL-Only Validation (Default)
Basic syntax and semantic validation without manifest checking:

```rust
let result = RunbookBuilder::new()
    .addon("evm", vec![])
    .action("deploy", "evm::deploy_contract")
        .input("contract", "Token.sol")
    .validate();  // Uses HCL validation only
```

This validates:
- ✅ HCL syntax correctness
- ✅ Known addon namespaces
- ✅ Valid action types
- ❌ Does NOT validate: signer references, action outputs, env variables

### 2. Manifest Validation
Full validation including environment variables and input checking:

```rust
let result = RunbookBuilder::new()
    .addon("evm", vec![])
    .action("deploy", "evm::deploy_contract")
        .input("signer", "signer.deployer")
    .with_environment("production", vec![
        ("API_KEY", "prod-key"),
        ("API_URL", "https://api.prod.com"),
    ])
    .set_current_environment("production")  // REQUIRED for manifest validation
    .validate();  // Now uses full manifest validation
```

This additionally validates:
- ✅ All `env.*` references have corresponding environment variables
- ✅ Environment inheritance (e.g., "defaults" → "production")
- ✅ CLI input overrides

## Important: Environment Specification

**When using manifest validation, you MUST specify which environment to validate against:**

```rust
// ❌ WRONG: Sets environments but doesn't specify which one
let result = RunbookBuilder::new()
    .with_environment("staging", vec![("API", "staging-api")])
    .with_environment("production", vec![("API", "prod-api")])
    .validate();  // Falls back to HCL-only validation!

// ✅ CORRECT: Explicitly sets the current environment
let result = RunbookBuilder::new()
    .with_environment("staging", vec![("API", "staging-api")])
    .with_environment("production", vec![("API", "prod-api")])
    .set_current_environment("production")  // Required!
    .validate();  // Uses manifest validation for "production"
```

Without specifying an environment, validation can only check against "defaults", which may not include all variables needed for actual environments. This partial validation can give false confidence.

## Builder API

### Basic Structure

```rust
RunbookBuilder::new()
    // Add blockchain configurations
    .addon("evm", vec![("network_id", "1")])
    
    // Add signers
    .signer("deployer", "evm::private_key", vec![
        ("private_key", "0x123...")
    ])
    
    // Add actions
    .action("deploy", "evm::deploy_contract")
        .input("contract", "Token.sol")
        .input("signer", "signer.deployer")
    
    // Add outputs
    .output("address", "action.deploy.contract_address")
    
    // Validate
    .validate()
```

### Environment and Manifest Support

```rust
// Create a custom manifest
let manifest = create_test_manifest_with_env(vec![
    ("defaults", vec![("BASE_URL", "https://api.test.com")]),
    ("production", vec![("BASE_URL", "https://api.prod.com")]),
]);

RunbookBuilder::new()
    .with_manifest(manifest)
    .set_current_environment("production")
    .validate_with_manifest()  // Explicit manifest validation
```

### CLI Input Overrides

```rust
RunbookBuilder::new()
    .with_environment("test", vec![("KEY", "env-value")])
    .with_cli_input("KEY", "cli-override")  // Overrides env value
    .set_current_environment("test")
    .validate()
```

## Assertions

```rust
use txtx_test_utils::{assert_validation_error, assert_validation_passes};

// Check for specific errors
assert_validation_error!(result, "undefined signer");

// Ensure validation passes
assert_validation_passes!(result);
```

## Advanced: Doctor Validation

For doctor-level validation (requires txtx-cli), implement the `RunbookBuilderExt` trait:

```rust
impl RunbookBuilderExt for RunbookBuilder {
    fn validate_with_doctor_impl(...) -> ValidationResult {
        // Use RunbookAnalyzer from txtx-cli
    }
}

// Then use:
result.validate_with_doctor(manifest, Some("production".to_string()));
```

## Testing Best Practices

1. **Always specify environment for manifest validation** - Don't rely on partial "defaults" validation
2. **Use appropriate validation mode** - HCL-only for syntax, manifest for full validation  
3. **Test error cases** - Ensure your runbooks fail appropriately
4. **Use CLI inputs for overrides** - Test precedence of configuration sources
5. **Mock external calls** - Use `with_mock()` for blockchain interactions