#[cfg(test)]
mod tests {
    use std::path::Path;
    use txtx_core::{
        manifest::WorkspaceManifest,
        validation::{ValidationResult, hcl_validator},
    };
    use txtx_addon_kit::indexmap::IndexMap;
    use crate::cli::{
        common::addon_registry,
        doctor::analyzer::RunbookAnalyzer,
    };

    // Helper to validate fixture content
    fn validate_fixture(content: &str) -> ValidationResult {
        let mut result = ValidationResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        // Load all addons to get their specifications
        let addons = addon_registry::get_all_addons();
        let addon_specs = addon_registry::extract_addon_specifications(&addons);
        let _ = hcl_validator::validate_with_hcl_and_addons(content, &mut result, "test.tx", addon_specs);
        result
    }

    #[test]
    fn test_problematic_transfer() {
        let content = include_str!("../../../../../../addons/evm/fixtures/doctor_demo/runbooks/problematic_transfer.tx");
        let result = validate_fixture(content);

        // This fixture has 4 errors: from, to, value, gas_used
        assert_eq!(result.errors.len(), 4, "Expected 4 errors in problematic_transfer.tx");

        // Check specific errors
        let error_messages: Vec<_> = result.errors.iter()
            .map(|e| &e.message)
            .collect();

        // Check that we have errors about invalid field access
        assert!(error_messages.iter().any(|m| m.contains("Field 'result' does not exist")));
        assert!(error_messages.iter().any(|m| m.contains("Field 'value' does not exist")));
        
        // All errors should mention the available outputs
        assert!(error_messages.iter().all(|m| m.contains("Available outputs: tx_hash")));
    }

    #[test]
    fn test_correct_transfer() {
        let content = include_str!("../../../../../../addons/evm/fixtures/doctor_demo/runbooks/correct_transfer.tx");
        let result = validate_fixture(content);

        // This fixture should have no errors
        assert_eq!(result.errors.len(), 0, "Expected no errors in correct_transfer.tx");
        assert_eq!(result.warnings.len(), 0, "Expected no warnings in correct_transfer.tx");
    }

    #[test]
    fn test_undefined_action() {
        // Take a valid fixture and break it by referencing undefined action
        let valid = include_str!("../../../../../../addons/evm/fixtures/doctor_demo/runbooks/correct_transfer.tx");
        let broken = valid.replace("action.transfer.tx_hash", "action.nonexistent.tx_hash");

        let result = validate_fixture(&broken);
        assert_eq!(result.errors.len(), 2, "Expected 2 errors (one for each reference to nonexistent action)");
        assert!(result.errors[0].message.contains("undefined action"));
        assert!(result.errors[0].context.is_some());
        assert!(result.errors[1].message.contains("undefined action"));
        assert!(result.errors[1].context.is_some());
    }

    #[test]
    fn test_send_eth_invalid_field_access() {
        let runbook = r#"
            addon "evm" { 
                network_id = 1 
            }
            
            action "send" "evm::send_eth" { 
                from = "0x123"
                to = "0x456"
                value = "1000" 
            }
            
            output "bad" { 
                value = action.send.from 
            }
        "#;

        let result = validate_fixture(runbook);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("Field 'from' does not exist"));
        assert!(result.errors[0].message.contains("send_eth"));
        assert!(result.errors[0].message.contains("Available outputs: tx_hash"));
        assert!(result.errors[0].documentation_link.is_some());
    }

    #[test]
    fn test_invalid_action_fields() {
        // Table-driven test for common invalid field access patterns
        let test_cases = vec![
            ("evm::send_eth", "from", "Field 'from' does not exist"),
            ("evm::send_eth", "to", "Field 'to' does not exist"),
            ("evm::send_eth", "gas", "Field 'gas' does not exist"),
            ("evm::send_eth", "gas_used", "Field 'gas_used' does not exist"),
        ];

        for (action_type, field, expected_error) in test_cases {
            let runbook = format!(r#"
                addon "evm" {{ 
                    network_id = 1 
                }}
                
                action "test" "{}" {{ 
                    value = "1000" 
                }}
                
                output "bad" {{ 
                    value = action.test.{} 
                }}
            "#, action_type, field);

            let result = validate_fixture(&runbook);
            assert_eq!(
                result.errors.len(), 
                1, 
                "Testing field '{}' on {}", 
                field, 
                action_type
            );
            assert!(
                result.errors[0].message.contains(expected_error),
                "Expected error message to contain '{}' when accessing field '{}' on {}",
                expected_error,
                field,
                action_type
            );
        }
    }

    #[test]
    fn test_nested_invalid_field_access() {
        let runbook = r#"
            addon "evm" { 
                network_id = 1 
            }
            
            action "send" "evm::send_eth" { 
                value = "1000" 
            }
            
            output "nested_bad" { 
                value = action.send.result.from 
            }
        "#;

        let result = validate_fixture(runbook);
        // Should detect invalid field access even with nested path
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_multiple_errors_in_one_runbook() {
        let runbook = r#"
            addon "evm" { 
                network_id = 1 
            }
            
            action "send1" "evm::send_eth" { 
                value = "1000" 
            }
            
            output "error1" { 
                value = action.send1.from 
            }
            
            output "error2" { 
                value = action.send1.to 
            }
            
            output "error3" { 
                value = action.undefined.result
            }
        "#;

        let result = validate_fixture(runbook);
        assert_eq!(result.errors.len(), 3, "Expected 3 errors");
        
        // Should have different error types
        let has_field_errors = result.errors.iter()
            .filter(|e| e.message.contains("does not exist on action"))
            .count() == 2;
        let has_undefined_error = result.errors.iter()
            .any(|e| e.message.contains("undefined action"));
            
        assert!(has_field_errors);
        assert!(has_undefined_error);
    }

    #[test]
    fn test_valid_field_access() {
        let runbook = r#"
            addon "evm" { 
                network_id = 1 
            }
            
            action "send" "evm::send_eth" { 
                from = "0x123"
                to = "0x456"
                value = "1000" 
            }
            
            output "valid" { 
                value = action.send.tx_hash 
            }
        "#;

        let result = validate_fixture(runbook);
        assert_eq!(result.errors.len(), 0, "tx_hash is a valid output for send_eth");
    }

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
        
        // Parse and validate with HCL validator
        // Load all addons to get their specifications
        let addons = addon_registry::get_all_addons();
        let addon_specs = addon_registry::extract_addon_specifications(&addons);
        
        // Parse and validate with HCL validator
        let input_refs = hcl_validator::validate_with_hcl_and_addons(runbook_content, &mut result, "test.tx", addon_specs)
            .expect("Failed to parse");
        
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

    #[test]
    fn test_unknown_action_type() {
        let runbook = r#"
            addon "evm" { 
                network_id = 1 
            }
            
            action "deploy" "evm::deploy" { 
                value = "1000" 
            }
            
            output "result" { 
                value = action.deploy.tx_hash 
            }
        "#;

        let result = validate_fixture(runbook);
        assert_eq!(result.errors.len(), 1, "Expected 1 error for unknown action type");
        
        let error = &result.errors[0];
        assert!(error.message.contains("Unknown action type 'evm::deploy'"));
        assert!(error.message.contains("Available actions for 'evm':"));
        assert!(error.message.contains("evm::deploy_contract"));
        assert!(error.context.as_ref().unwrap().contains("Did you mean 'evm::deploy_contract'?"));
    }

    #[test]
    fn test_unknown_namespace() {
        let runbook = r#"
            action "test" "unknown::action" { 
                value = "1000" 
            }
            
            output "result" { 
                value = action.test.result 
            }
        "#;

        let result = validate_fixture(runbook);
        assert_eq!(result.errors.len(), 1, "Expected 1 error for unknown namespace");
        
        let error = &result.errors[0];
        assert!(error.message.contains("Unknown addon namespace 'unknown'"));
        assert!(error.message.contains("Available namespaces:"));
        assert!(error.message.contains("evm"));
    }

    #[test]
    fn test_invalid_action_type_format() {
        let runbook = r#"
            action "test" "invalid_format" { 
                value = "1000" 
            }
        "#;

        let result = validate_fixture(runbook);
        assert_eq!(result.errors.len(), 1, "Expected 1 error for invalid format");
        
        let error = &result.errors[0];
        assert!(error.message.contains("Invalid action type 'invalid_format'"));
        assert!(error.message.contains("must be in format 'namespace::action'"));
    }

    #[test]
    fn test_cascading_errors_suppressed() {
        let runbook = r#"
            addon "evm" { 
                network_id = 1 
            }
            
            # Invalid action type
            action "test" "unknown::action" { 
                value = "1000" 
            }
            
            # This should NOT generate additional errors
            output "out1" { 
                value = action.test.field1 
            }
            
            output "out2" { 
                value = action.test.field2.nested 
            }
            
            # Valid action - should still validate
            action "valid" "evm::send_eth" { 
                to = "0x123"
                amount = "1000"
            }
            
            # This SHOULD generate an error
            output "out3" { 
                value = action.valid.invalid_field 
            }
        "#;

        let result = validate_fixture(runbook);
        
        // Should have 2 errors: unknown namespace + invalid field on valid action
        assert_eq!(result.errors.len(), 2, "Expected 2 errors total");
        
        // Check that we have one primary error (unknown namespace)
        assert!(result.errors[0].message.contains("Unknown addon namespace"));
        
        // And one secondary error (invalid field)
        assert!(result.errors[1].message.contains("Field 'invalid_field' does not exist"));
    }

    #[test]
    fn test_missing_input_in_environment() {
        let runbook_content = r#"
            output "test" {
                value = input.MISSING_VAR
            }
        "#;
        
        let mut result = ValidationResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        // Parse and validate with HCL validator
        // Load all addons to get their specifications
        let addons = addon_registry::get_all_addons();
        let addon_specs = addon_registry::extract_addon_specifications(&addons);
        
        // Parse and validate with HCL validator
        let input_refs = hcl_validator::validate_with_hcl_and_addons(runbook_content, &mut result, "test.tx", addon_specs)
            .expect("Failed to parse");
        
        let mut manifest = WorkspaceManifest {
            name: "test".to_string(),
            id: "test-id".to_string(),
            runbooks: Vec::new(),
            environments: IndexMap::new(),
            location: None,
        };
        
        // Add environment without the required input
        let mut env = IndexMap::new();
        env.insert("OTHER_VAR".to_string(), "value".to_string());
        manifest.environments.insert("prod".to_string(), env);
        
        let analyzer = RunbookAnalyzer::new();
        analyzer.validate_inputs_against_manifest_with_locations(
            &input_refs,
            runbook_content,
            &manifest,
            Some(&"prod".to_string()),
            &mut result,
            Path::new("test.tx"),
            &[],
        );
        
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("MISSING_VAR"));
        assert!(result.suggestions.iter().any(|s| 
            s.example.as_ref().map_or(false, |e| e.contains("global"))),
            "Should suggest adding to global environment"
        );
    }

    #[test]
    fn test_cli_precedence_note() {
        let runbook_content = r#"
            output "test" {
                value = input.MY_VAR
            }
        "#;
        
        let mut result = ValidationResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        // Parse and validate with HCL validator
        // Load all addons to get their specifications
        let addons = addon_registry::get_all_addons();
        let addon_specs = addon_registry::extract_addon_specifications(&addons);
        
        // Parse and validate with HCL validator
        let input_refs = hcl_validator::validate_with_hcl_and_addons(runbook_content, &mut result, "test.tx", addon_specs)
            .expect("Failed to parse");
        
        let mut manifest = WorkspaceManifest {
            name: "test".to_string(),
            id: "test-id".to_string(),
            runbooks: Vec::new(),
            environments: IndexMap::new(),
            location: None,
        };
        
        let mut env = IndexMap::new();
        env.insert("MY_VAR".to_string(), "env_value".to_string());
        manifest.environments.insert("global".to_string(), env);
        
        // Pass CLI inputs to trigger the precedence note
        let cli_inputs = vec![("MY_VAR".to_string(), "cli_value".to_string())];
        
        let analyzer = RunbookAnalyzer::new();
        analyzer.validate_inputs_against_manifest_with_locations(
            &input_refs,
            runbook_content,
            &manifest,
            None,
            &mut result,
            Path::new("test.tx"),
            &cli_inputs,
        );
        
        assert!(result.suggestions.iter().any(|s| 
            s.message.contains("CLI inputs take precedence")),
            "Should mention CLI precedence"
        );
    }

    #[test]
    fn test_cli_inputs_override_environment() {
        let runbook_content = r#"
            output "test" {
                value = input.MY_VAR
            }
            output "test2" {
                value = input.ANOTHER_VAR
            }
        "#;
        
        let mut result = ValidationResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        // Parse and validate with HCL validator
        // Load all addons to get their specifications
        let addons = addon_registry::get_all_addons();
        let addon_specs = addon_registry::extract_addon_specifications(&addons);
        
        // Parse and validate with HCL validator
        let input_refs = hcl_validator::validate_with_hcl_and_addons(runbook_content, &mut result, "test.tx", addon_specs)
            .expect("Failed to parse");
        
        let mut manifest = WorkspaceManifest {
            name: "test".to_string(),
            id: "test-id".to_string(),
            runbooks: Vec::new(),
            environments: IndexMap::new(),
            location: None,
        };
        
        let mut env = IndexMap::new();
        env.insert("MY_VAR".to_string(), "env_value".to_string());
        // ANOTHER_VAR is not in environment
        manifest.environments.insert("global".to_string(), env);
        
        // Provide both values via CLI
        let cli_inputs = vec![
            ("MY_VAR".to_string(), "cli_override".to_string()),
            ("ANOTHER_VAR".to_string(), "cli_provided".to_string()),
        ];
        
        let analyzer = RunbookAnalyzer::new();
        analyzer.validate_inputs_against_manifest_with_locations(
            &input_refs,
            runbook_content,
            &manifest,
            None,
            &mut result,
            Path::new("test.tx"),
            &cli_inputs,
        );
        
        // Should have no errors since CLI provides all values
        assert_eq!(result.errors.len(), 0, "CLI inputs should provide all required values");
        
        // Should mention CLI inputs provided
        assert!(result.suggestions.iter().any(|s| 
            s.message.contains("2 CLI inputs provided")),
            "Should mention number of CLI inputs"
        );
    }
}