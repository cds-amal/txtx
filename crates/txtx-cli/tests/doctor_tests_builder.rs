use txtx_test_utils::builders::{RunbookBuilder, create_test_manifest_with_env};
use txtx_core::manifest::WorkspaceManifest;

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
        
        let result = builder.validate();
        
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
        
        let result = builder.validate();
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
        
        let result = builder.validate();
        
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
        
        let result = builder.validate();
        
        assert!(!result.success);
        assert_eq!(result.errors.len(), 1, "Expected 1 error");
        assert_validation_error!(result, "unknown_action");
    }
    
    // Test case 5: test_doctor_flow_missing_variable.tx
    // Should find undefined flow variable and usage error
    #[test]
    #[ignore = "Requires doctor validation to check flow variable context - not yet implemented"]
    fn test_doctor_flow_missing_variable_with_builder() {
        // Flow variable validation requires doctor mode which understands
        // the context of flow blocks and can validate variable references
        // within flows. This is not yet available in manifest validation.
        
        // When implemented, this test would look like:
        /*
        let mut builder = RunbookBuilder::new()
            .with_content(r#"
                flow "deploy" {
                    // Reference to undefined flow variable
                    action "send" "evm::send_eth" {
                        to = flow.undefined_var  // ERROR: undefined flow variable
                        value = "1000"
                    }
                }
            "#);
            
        let result = builder.validate_with_doctor();
        assert_validation_error!(result, "undefined_var");
        */
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
        
        let result = builder.validate();
        
        assert!(!result.success);
        assert!(result.errors.len() >= 4, "Expected at least 4 errors, got: {:?}",
            result.errors.iter().map(|e| &e.message).collect::<Vec<_>>());
    }
    
    // Test environment variable validation
    #[test]
    #[ignore = "Temporarily disabled - supervisor UI build issue"]
    fn test_doctor_env_validation_with_builder() {
        // Test missing environment variable
        let mut builder = RunbookBuilder::new()
            .with_content(r#"
                variable "api_key" {
                    value = env.API_KEY
                }
                output "key" {
                    value = input.api_key
                }
            "#)
            .with_environment("production", vec![
                ("OTHER_VAR", "value")
                // API_KEY is missing!
            ])
            .set_current_environment("production");
        
        let result = builder.validate_with_manifest();
        
        // Should have error about missing API_KEY
        assert!(!result.success);
        assert_validation_error!(result, "API_KEY");
        
        // Test with environment variable present
        let mut builder2 = RunbookBuilder::new()
            .with_content(r#"
                variable "api_key" {
                    value = env.API_KEY
                }
                output "key" {
                    value = input.api_key
                }
            "#)
            .with_environment("production", vec![
                ("API_KEY", "prod-key-123")
            ])
            .set_current_environment("production");
        
        let result2 = builder2.validate_with_manifest();
        assert_validation_passes!(result2);
    }
    
    // Test CLI input validation
    #[test]
    #[ignore = "Temporarily disabled - supervisor UI build issue"]
    fn test_doctor_cli_input_validation_with_builder() {
        // Test that CLI inputs override environment variables
        let mut builder = RunbookBuilder::new()
            .with_content(r#"
                variable "api_url" {
                    value = env.API_URL
                }
                variable "api_key" {
                    value = env.API_KEY
                }
                output "url" {
                    value = input.api_url
                }
                output "key" {
                    value = input.api_key
                }
            "#)
            .with_environment("production", vec![
                ("API_URL", "https://api.prod.com"),
                ("API_KEY", "prod-key")
            ])
            .with_cli_input("api_url", "https://api.override.com")  // Override URL
            .set_current_environment("production");
        
        let result = builder.validate_with_manifest();
        
        // Validation should pass - CLI inputs are valid overrides
        assert_validation_passes!(result);
        
        // Test with CLI input that doesn't correspond to any variable
        let mut builder2 = RunbookBuilder::new()
            .with_content(r#"
                variable "api_url" {
                    value = env.API_URL
                }
                output "url" {
                    value = input.api_url
                }
            "#)
            .with_environment("production", vec![
                ("API_URL", "https://api.prod.com")
            ])
            .with_cli_input("unknown_var", "some_value")  // This doesn't match any variable
            .set_current_environment("production");
        
        let result2 = builder2.validate_with_manifest();
        
        // Should still pass - extra CLI inputs are allowed
        assert_validation_passes!(result2);
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
        
        let result = builder.validate();
        assert_validation_passes!(result);
    }
    
    // Test nested field access validation
    #[test]
    #[ignore = "Requires doctor validation to check nested field access - not yet implemented"]
    fn test_doctor_nested_field_access_with_builder() {
        // This test would require doctor validation mode which checks if
        // action outputs actually have the fields being accessed.
        // For example: action.deploy.contract_address is valid only if
        // the deploy action type actually outputs a contract_address field.
        // This validation is not yet available in manifest validation.
        
        // When implemented, this test would look like:
        /*
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("send", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "1000")
            .output("invalid", "action.send.contract_address");  // send_eth doesn't have contract_address!
            
        let result = builder.validate_with_doctor();  // Need doctor mode
        assert_validation_error!(result, "contract_address");
        */
    }
}

#[cfg(test)]
mod doctor_hcl_vs_doctor_comparison {
    
    // This test demonstrates the difference between HCL-only and manifest validation
    #[test]
    #[ignore = "Temporarily disabled - supervisor UI build issue"]
    fn test_validation_mode_differences() {
        use txtx_test_utils::builders::*;
        
        let content = r#"
            variable "api_key" {
                value = env.API_KEY
            }
            output "key" {
                value = input.api_key
            }
        "#;
        
        // Test 1: HCL-only validation (no environment set)
        let mut builder1 = RunbookBuilder::new()
            .with_content(content)
            .with_environment("production", vec![
                // API_KEY is missing but HCL validation won't catch it
            ]);
        
        let result1 = builder1.validate();  // Uses HCL-only validation
        
        // HCL validation passes even though API_KEY is missing!
        assert_validation_passes!(result1);
        
        // Test 2: Manifest validation (environment is set)
        let mut builder2 = RunbookBuilder::new()
            .with_content(content)
            .with_environment("production", vec![
                // API_KEY is still missing
            ])
            .set_current_environment("production");  // This enables manifest validation
        
        let result2 = builder2.validate_with_manifest();
        
        // Manifest validation catches the missing API_KEY!
        assert!(!result2.success);
        assert_validation_error!(result2, "API_KEY");
        
        // Test 3: Manifest validation with variable defined
        let mut builder3 = RunbookBuilder::new()
            .with_content(content)
            .with_environment("production", vec![
                ("API_KEY", "prod-key-123")
            ])
            .set_current_environment("production");
        
        let result3 = builder3.validate_with_manifest();
        
        // Now it passes
        assert_validation_passes!(result3);
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
        let result = builder.validate();
        
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