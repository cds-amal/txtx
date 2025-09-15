# Test Conversion Examples

This document shows real examples of how tests are simplified using the new `RunbookBuilder` API and assertion macros.

## Example 1: Integration Test - Doctor Command

### Before (doctor_tests.rs)
```rust
fn run_doctor_on_file(content: &str) -> (String, bool) {
    // Create temporary file with unique name
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let test_file = format!("test_doctor_{}_{}.tx", std::process::id(), timestamp);
    let file_path = Path::new(&test_file);
    
    // Write test content
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Run doctor command
    let output = Command::new(env!("CARGO_BIN_EXE_txtx"))
        .arg("doctor")
        .arg(&test_file)
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("Failed to run doctor command");
    
    // Clean up
    let _ = fs::remove_file(&file_path);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let full_output = format!("{}{}", stdout, stderr);
    
    (full_output, output.status.success())
}

#[test]
fn test_doctor_finds_undefined_action() {
    let runbook = r#"
        output "bad" {
            value = action.nonexistent.result
        }
    "#;
    
    let (json, success) = run_doctor_json(runbook);
    assert!(!success, "Doctor should fail on undefined action");
    
    let errors = json["errors"].as_array().expect("Expected errors array");
    assert_eq!(errors.len(), 1, "Expected exactly one error");
    
    let error = &errors[0];
    assert_eq!(error["level"], "error");
    assert!(error["message"].as_str().unwrap().contains("undefined action"));
}
```

### After (doctor_tests_improved.rs)
```rust
#[test]
fn test_doctor_finds_undefined_action() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            output "bad" {
                value = action.nonexistent.result
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "undefined action");
}
```

**Improvements:**
- 17 lines → 10 lines (41% reduction)
- No file system operations
- No process spawning
- No JSON parsing
- Cleaner, more readable test

## Example 2: Unit Test - Environment Inheritance

### Before (mod.rs - 64 lines)
```rust
#[test]
fn test_environment_global_inheritance() {
    let runbook_content = r#"
        addon "evm" {
            network_id = input.CHAIN_ID
            rpc_api_url = input.RPC_URL
        }
        
        action "send" "evm::send_eth" {
            value = input.AMOUNT
        }
    "#;
    
    let mut result = ValidationResult {
        errors: Vec::new(),
        warnings: Vec::new(),
        suggestions: Vec::new(),
    };
    
    // Load all addons to get their specifications
    let addons = addon_registry::get_all_addons();
    let addon_specs = addon_registry::extract_addon_specifications(&addons);
    
    // Parse and validate with HCL validator
    let input_refs = hcl_validator::validate_with_hcl_and_addons(
        runbook_content, &mut result, "test.tx", addon_specs
    ).expect("Failed to parse");
    
    // Create a manifest with global and dev environments
    let mut manifest = WorkspaceManifest {
        name: "test".to_string(),
        id: "test-id".to_string(),
        runbooks: Vec::new(),
        environments: IndexMap::new(),
        location: None,
    };
    
    // Add global environment
    let mut global_env = IndexMap::new();
    global_env.insert("CHAIN_ID".to_string(), "1".to_string());
    global_env.insert("RPC_URL".to_string(), "https://mainnet.infura.io".to_string());
    manifest.environments.insert("global".to_string(), global_env);
    
    // Add dev environment that overrides CHAIN_ID
    let mut dev_env = IndexMap::new();
    dev_env.insert("CHAIN_ID".to_string(), "5".to_string());
    dev_env.insert("AMOUNT".to_string(), "1000".to_string());
    manifest.environments.insert("dev".to_string(), dev_env);
    
    // Test with dev environment - should inherit RPC_URL from global
    let analyzer = RunbookAnalyzer::new();
    analyzer.validate_inputs_against_manifest_with_locations(
        &input_refs,
        runbook_content,
        &manifest,
        Some(&"dev".to_string()),
        &mut result,
        Path::new("test.tx"),
        &[],
    );
    
    assert_eq!(result.errors.len(), 0, "All inputs should be found through inheritance");
}
```

### After (mod_improved.rs - 25 lines)
```rust
#[test]
fn test_environment_global_inheritance() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" {
                network_id = input.CHAIN_ID
                rpc_api_url = input.RPC_URL
            }
            
            action "send" "evm::send_eth" {
                value = input.AMOUNT
            }
        "#)
        .with_environment("global", vec![
            ("CHAIN_ID", "1"),
            ("RPC_URL", "https://mainnet.infura.io"),
        ])
        .with_environment("dev", vec![
            ("CHAIN_ID", "5"),  // Override from global
            ("AMOUNT", "1000"),
            // RPC_URL inherited from global
        ])
        .validate();
    
    assert_success!(result);
}
```

**Improvements:**
- 64 lines → 25 lines (61% reduction!)
- No manual manifest creation
- No addon registry interaction
- Clear environment setup
- Intent is immediately obvious

## Example 3: Table-Driven Tests

### Before
```rust
#[test]
fn test_invalid_action_fields() {
    let test_cases = vec![
        ("evm::send_eth", "from", "Field 'from' does not exist"),
        ("evm::send_eth", "to", "Field 'to' does not exist"),
    ];

    for (action_type, field, expected_error) in test_cases {
        let runbook = format!(r#"
            addon "evm" {{ network_id = 1 }}
            action "test" "{}" {{ value = "1000" }}
            output "bad" {{ value = action.test.{} }}
        "#, action_type, field);

        let result = validate_fixture(&runbook);
        assert_eq!(result.errors.len(), 1, 
            "Testing field '{}' on {}", field, action_type);
        assert!(
            result.errors[0].message.contains(expected_error),
            "Expected error message to contain '{}' when accessing field '{}' on {}",
            expected_error, field, action_type
        );
    }
}
```

### After
```rust
#[test]
fn test_invalid_action_fields() {
    let test_cases = vec![
        ("evm::send_eth", "from", "Field 'from' does not exist"),
        ("evm::send_eth", "to", "Field 'to' does not exist"),
    ];
    
    for (action_type, field, expected_error) in test_cases {
        let result = RunbookBuilder::new()
            .with_content(&format!(r#"
                addon "evm" {{ network_id = 1 }}
                action "test" "{}" {{ value = "1000" }}
                output "bad" {{ value = action.test.{} }}
            "#, action_type, field))
            .validate();
        
        assert_validation_error!(result, expected_error);
    }
}
```

**Improvements:**
- Cleaner test structure
- Better error messages from macros
- Less boilerplate in assertions

## Summary of Benefits

1. **Code Reduction**: 40-60% fewer lines of code on average
2. **No External Dependencies**: Tests run in-process without file I/O or process spawning
3. **Better Error Messages**: Assertion macros provide clear failure messages
4. **Consistent Patterns**: All tests follow the same builder pattern
5. **Easier Maintenance**: Changes to validation logic don't break test structure
6. **Faster Execution**: No process spawning or file system operations
7. **Better Test Coverage**: Easier to write means more tests get written

## Migration Guide

To convert existing tests:

1. Replace file-based test helpers with `RunbookBuilder`
2. Use `.with_content()` for simple runbooks
3. Use `.with_file()` for multi-file scenarios
4. Replace manual assertions with `assert_validation_error!` or `assert_success!`
5. Remove boilerplate setup code (manifest creation, addon loading, etc.)

The new approach makes tests focus on **what** is being tested rather than **how** to set up the test.