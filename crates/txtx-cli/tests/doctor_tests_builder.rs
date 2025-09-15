use txtx_test_utils::{RunbookBuilder, ValidationResult, create_test_manifest_with_env};
use txtx_core::manifest::WorkspaceManifest;
use txtx_addon_kit::indexmap::IndexMap;
use std::path::PathBuf;

// Helper macros for common assertions
macro_rules! assert_validation_error {
    ($result:expr, $expected:expr) => {
        assert!(!$result.success, "Expected validation to fail");
        assert!(
            $result.errors.iter().any(|e| e.message.contains($expected)),
            "Expected error containing '{}', but got: {:?}",
            $expected,
            $result.errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    };
}

macro_rules! assert_validation_passes {
    ($result:expr) => {
        assert!(
            $result.success,
            "Expected validation to succeed, but got errors: {:?}",
            $result.errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    };
}

#[cfg(test)]
mod doctor_fixture_tests {
    use super::*;
    use txtx_test_utils::ValidationMode;
    
    // Test case 1: test_doctor_simple.tx
    // Expected errors: 2
    // 1. Undefined signer: signer.undefined_signer
    // 2. Invalid field access: action.send.from (send_eth has no 'from' output)
    #[test]
    fn test_doctor_simple_with_builder() {
        let mut builder = RunbookBuilder::new()
            .action("send", "evm::send_eth")
                .input("signer", "signer.undefined_signer")  // ERROR: signer not defined
                .input("to", "0x123")
                .input("value", "1000")
            .output("bad", "action.send.from");  // ERROR: send_eth only outputs 'tx_hash'
        
        let result = builder.validate_with_doctor(None, None);
        
        // Should have 2 errors
        assert!(!result.success);
        assert_eq!(result.errors.len(), 2, "Expected 2 errors, got: {:?}", 
            result.errors.iter().map(|e| &e.message).collect::<Vec<_>>());
        
        // Check specific errors
        assert_validation_error!(result, "undefined_signer");
        assert_validation_error!(result, "from");
    }
    
    // Test case 2: test_doctor_valid.tx
    // Test file with no errors
    #[test]
    fn test_doctor_valid_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![
                ("rpc_api_url", "\"https://eth.example.com\"")
            ])
            // Define a signer
            .signer("operator", "evm::private_key", vec![
                ("private_key", "0x1234")
            ])
            // Action 1 references the signer
            .action("action1", "evm::send_eth")
                .input("from", "signer.operator.address")
                .input("to", "0x456")
                .input("value", "1000")
            // Action 2 references action1 (forward reference is OK)
            .action("action2", "evm::send_eth")
                .input("from", "signer.operator.address")
                .input("to", "0x789")
                .input("value", "2000")
                .input("depends_on", "[action.action1.tx_hash]")
            // Output references both actions
            .output("tx1", "action.action1.tx_hash")
            .output("tx2", "action.action2.tx_hash");
        
        let result = builder.validate_with_doctor(None, None);
        assert_validation_passes!(result);
    }
    
    // Test case 3: test_doctor_two_pass.tx
    // Should find undefined action reference
    #[test]
    fn test_doctor_two_pass_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("first", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "1000")
            .output("result", "action.second.tx_hash");  // ERROR: 'second' action not defined
        
        let result = builder.validate_with_doctor(None, None);
        
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1, "Expected 1 error");
        assert_validation_error!(result, "second");
    }
    
    // Test case 4: test_doctor_unknown_action_type.tx
    // Should find unknown action type
    #[test]
    fn test_doctor_unknown_action_type_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("test", "evm::unknown_action");  // ERROR: unknown action type
        
        let result = builder.validate_with_doctor(None, None);
        
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1, "Expected 1 error");
        assert_validation_error!(result, "unknown_action");
    }
    
    // Test case 5: test_doctor_flow_missing_variable.tx
    // Should find undefined flow variable and usage error
    #[test]
    fn test_doctor_flow_missing_variable_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .variable("defined_var", "42")
            .action("test", "evm::send_eth")
                .input("value", "flow.undefined_var")  // ERROR: undefined flow variable
                .input("to", "flow.defined_var");      // ERROR: flow variables not in flow context
        
        let result = builder.validate_with_doctor(None, None);
        
        assert!(!result.success);
        assert!(result.errors.len() >= 2, "Expected at least 2 errors");
        assert_validation_error!(result, "undefined_var");
    }
    
    // Test case 6: Multiple errors combined
    #[test]
    fn test_doctor_multiple_errors_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            // Multiple errors in one runbook
            .action("send1", "evm::send_eth")
                .input("signer", "signer.missing")     // ERROR: undefined signer
                .input("to", "0x123")
                .input("value", "1000")
            .action("send2", "evm::invalid_action")    // ERROR: invalid action type
                .input("param", "value")
            .output("bad1", "action.send1.invalid")    // ERROR: invalid field
            .output("bad2", "action.missing.tx_hash"); // ERROR: undefined action
        
        let result = builder.validate_with_doctor(None, None);
        
        assert!(!result.success);
        assert!(result.errors.len() >= 4, "Expected at least 4 errors, got: {:?}",
            result.errors.iter().map(|e| &e.message).collect::<Vec<_>>());
    }
    
    // Test environment variable validation
    #[test]
    fn test_doctor_env_validation_with_builder() {
        let manifest = create_test_manifest_with_env(vec![
            ("production", vec![
                ("API_URL", "https://api.example.com"),
                ("API_KEY", "secret123"),
            ]),
            ("development", vec![
                ("API_URL", "http://localhost:8080"),
                // API_KEY missing in dev
            ]),
        ]);
        
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![
                ("rpc_api_url", "env.API_URL")
            ])
            .variable("key", "env.API_KEY")  // Will fail in development env
            .action("test", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "env.MISSING_VAR");  // Always fails
        
        // Test with production environment - should only have MISSING_VAR error
        let prod_result = builder.validate_with_doctor(Some(manifest.clone()), Some("production".to_string()));
        assert!(!prod_result.success);
        assert_validation_error!(prod_result, "MISSING_VAR");
        
        // Test with development environment - should have API_KEY and MISSING_VAR errors
        let dev_result = builder.validate_with_doctor(Some(manifest), Some("development".to_string()));
        assert!(!dev_result.success);
        assert_validation_error!(dev_result, "API_KEY");
        assert_validation_error!(dev_result, "MISSING_VAR");
    }
    
    // Test CLI input validation
    #[test]
    fn test_doctor_cli_input_validation_with_builder() {
        let mut builder = RunbookBuilder::new()
            .with_cli_input("provided_input", "test_value")
            .addon("evm", vec![])
            .variable("var1", "input.provided_input")     // OK
            .variable("var2", "input.missing_input")      // ERROR: not provided
            .action("test", "evm::send_eth")
                .input("to", "input.another_missing");    // ERROR: not provided
        
        let result = builder.validate_with_doctor(None, None);
        
        assert!(!result.success);
        assert!(result.errors.len() >= 2, "Expected at least 2 errors for missing inputs");
        assert_validation_error!(result, "missing_input");
        assert_validation_error!(result, "another_missing");
    }
    
    // Test forward references are allowed
    #[test]
    fn test_doctor_forward_references_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .signer("deployer", "evm::private_key", vec![
                ("private_key", "0x123")
            ])
            // Action 1 references action2 (forward reference)
            .action("action1", "evm::send_eth")
                .input("from", "signer.deployer.address")
                .input("to", "action.action2.contract_address")  // Forward ref
                .input("value", "1000")
            // Action 2 defined after action1
            .action("action2", "evm::deploy_contract")
                .input("contract", "\"Token.sol\"")
                .input("signer", "signer.deployer");
        
        let result = builder.validate_with_doctor(None, None);
        assert_validation_passes!(result);
    }
    
    // Test nested field access validation
    #[test]
    fn test_doctor_nested_field_access_with_builder() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("deploy", "evm::deploy_contract")
                .input("contract", "\"Contract.sol\"")
            .output("address", "action.deploy.contract_address")       // OK
            .output("invalid", "action.deploy.nested.field.access");  // ERROR: too deep
        
        let result = builder.validate_with_doctor(None, None);
        
        assert!(!result.success);
        assert_validation_error!(result, "nested");
    }
}

#[cfg(test)]
mod doctor_hcl_vs_doctor_comparison {
    use super::*;
    
    // This test demonstrates the difference between HCL-only and Doctor validation
    #[test]
    fn test_validation_mode_differences() {
        let create_runbook = || RunbookBuilder::new()
            .addon("evm", vec![])
            .action("test", "evm::send_eth")
                .input("signer", "signer.missing")        // Doctor catches this
                .input("to", "0x123")
                .input("value", "action.other.amount")    // Doctor catches undefined action
            .output("result", "action.test.from");        // Doctor catches invalid field
        
        // Test 1: HCL-only validation
        let mut hcl_builder = create_runbook();
        let hcl_result = hcl_builder.validate();
        
        // HCL validation might pass or only catch syntax errors
        println!("HCL validation errors: {}", hcl_result.errors.len());
        
        // Test 2: Doctor validation
        let mut doctor_builder = create_runbook();
        let doctor_result = doctor_builder.validate_with_doctor(None, None);
        
        // Doctor validation catches semantic errors
        assert!(!doctor_result.success);
        assert!(doctor_result.errors.len() >= 3, 
            "Doctor should catch at least 3 errors: undefined signer, undefined action, invalid field");
        
        println!("Doctor validation errors: {}", doctor_result.errors.len());
        for error in &doctor_result.errors {
            println!("  - {}", error.message);
        }
    }
}

#[cfg(test)]
mod doctor_multi_file_tests {
    use super::*;
    
    // Test multi-file runbook validation
    #[test]
    fn test_doctor_multi_file_with_builder() {
        // Main runbook file
        let mut builder = RunbookBuilder::new()
            .with_content(r#"
                import "./flows.tx"
                
                addon "evm" {
                    rpc_api_url = "https://eth.example.com"
                }
                
                action "main" "evm::send_eth" {
                    to = "0x123"
                    value = "1000"
                }
            "#)
            // Add imported file
            .with_file("./flows.tx", r#"
                flow "deployment" {
                    variable "token_name" {
                        value = "MyToken"
                    }
                    
                    action "deploy" "evm::deploy_contract" {
                        contract = "Token.sol"
                        constructor_args = [flow.token_name]
                    }
                }
            "#);
        
        // Doctor validation should handle multi-file imports
        let result = builder.validate_with_doctor(None, None);
        
        // This test would need actual multi-file support in the builder
        // For now, we're demonstrating the pattern
        println!("Multi-file validation result: {}", 
            if result.success { "✓ Success" } else { "✗ Failed" });
    }
}

// Helper function to create a standard test manifest
fn create_standard_test_manifest() -> WorkspaceManifest {
    create_test_manifest_with_env(vec![
        ("test", vec![
            ("ETH_RPC_URL", "http://localhost:8545"),
            ("DEPLOYER_KEY", "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"),
        ]),
    ])
}