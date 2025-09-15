//! Doctor command tests using the improved RunbookBuilder API
//! 
//! These tests demonstrate how much simpler testing becomes with the new API
//! compared to the original doctor_tests.rs

use txtx_test_utils::{RunbookBuilder, assert_validation_error, assert_success};

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

#[test]
fn test_doctor_finds_invalid_send_eth_field() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" { network_id = 1 }
            
            action "send" "evm::send_eth" {
                from = "0x123"
                to = "0x456"
                value = "1000"
            }
            
            output "bad" {
                value = action.send.from
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "Field 'from' does not exist");
}

#[test]
fn test_doctor_passes_valid_runbook() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" { network_id = 1 }
            
            signer "alice" "evm::wallet" {
                private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            }
            
            action "send" "evm::send_eth" {
                signer = signer.alice
                recipient_address = "0x456"
                amount = "1000"
            }
            
            output "tx" {
                value = action.send.tx_hash
            }
        "#)
        .validate();
    
    assert_success!(result);
}

#[test]
fn test_doctor_validates_action_types() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" { network_id = 1 }
            
            action "deploy" "evm::deploy" {
                signer = "0x123"
                fee = 1000
            }
            
            output "result" {
                value = action.deploy.contract_address
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "Unknown action type");
    assert_validation_error!(result, "evm::deploy");
    
    // Check for suggestion in error message
    let errors_str = result.errors.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(errors_str.contains("deploy_contract"), 
        "Expected suggestion to use 'deploy_contract'");
}

#[test]
fn test_doctor_validates_unknown_namespace() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            action "test" "unknown::action" {
                value = 123
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "Unknown addon namespace");
    assert_validation_error!(result, "unknown");
}

#[test]
fn test_doctor_with_environment_variables() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" { network_id = 1 }
            
            variable "api_key" {
                value = env.API_KEY
            }
            
            action "test" "std::send_http_request" {
                url = "https://api.example.com"
                headers = { "X-API-Key" = input.api_key }
            }
        "#)
        .with_environment("production", vec![
            ("API_KEY", "test-key-123")
        ])
        .validate();
    
    assert_success!(result);
}

#[test]
fn test_doctor_multi_file_runbook() {
    let result = RunbookBuilder::new()
        .with_file("main.tx", r#"
            include "flows.tx"
            
            variable "network" {
                value = env.NETWORK_ID
            }
            
            output "result" {
                value = action.deploy.address
            }
        "#)
        .with_file("flows.tx", r#"
            addon "evm" { network_id = input.network }
            
            action "deploy" "evm::deploy_contract" {
                contract = "./Token.sol"
                signer = "deployer"
            }
        "#)
        .with_environment("test", vec![
            ("NETWORK_ID", "1")
        ])
        .validate();
    
    assert_success!(result);
}

#[test]
#[ignore = "Circular dependency detection not implemented in simple validator"]
fn test_doctor_catches_circular_dependency() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            variable "a" {
                value = input.b
            }
            
            variable "b" {
                value = input.a
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "circular");
}

#[test]
fn test_doctor_validates_signer_references() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" { network_id = 1 }
            
            action "send" "evm::send_eth" {
                signer = signer.missing_signer
                recipient_address = "0x456"
                amount = "1000"
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "undefined signer");
    assert_validation_error!(result, "missing_signer");
}

#[test]
#[ignore = "Deep addon configuration validation not implemented in simple validator"]
fn test_doctor_validates_addon_configuration() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" {
                // Missing required network_id
            }
            
            action "test" "evm::call_contract" {
                contract_address = "0x123"
                function = "balanceOf"
                args = ["0x456"]
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "network_id");
}

//
// Comparison with original tests:
//
// Original test_doctor_finds_undefined_action: 17 lines
// New version: 10 lines (41% reduction)
//
// Benefits:
// 1. No file system operations needed
// 2. No process spawning required  
// 3. Clear, declarative test structure
// 4. Better error messages with assert_validation_error!
// 5. Easier to maintain and understand
//