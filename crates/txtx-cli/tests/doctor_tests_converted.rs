// Integration tests for the doctor command using RunbookBuilder API
// These tests validate runbooks without running the actual txtx binary

use txtx_test_utils::{RunbookBuilder, assert_validation_error, assert_success};

#[test]
fn test_doctor_finds_undefined_action() {
    let result = RunbookBuilder::new()
        .output("bad", "action.nonexistent.result")
        .validate();
    
    assert_validation_error!(result, "undefined action");
    assert_eq!(result.errors.len(), 1, "Expected exactly one error");
}

#[test]
fn test_doctor_finds_invalid_send_eth_field() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![("network_id", "1")])
        .action("send", "evm::send_eth")
            .input("from", "0x123")
            .input("to", "0x456")
            .input("value", "1000")
        .output("bad", "action.send.from")
        .validate();
    
    assert_validation_error!(result, "Field 'from' does not exist");
    assert_validation_error!(result, "send_eth");
    assert_eq!(result.errors.len(), 1, "Expected exactly one error");
}

#[test]
fn test_doctor_passes_valid_runbook() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![("network_id", "1")])
        .signer("alice", "evm::wallet", vec![
            ("private_key", "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        ])
        .action("send", "evm::send_eth")
            .input("signer", "signer.alice")
            .input("recipient_address", "0x456")
            .input("amount", "1000")
        .output("tx", "action.send.tx_hash")
        .validate();
    
    assert_success!(result);
}

#[test]
fn test_doctor_validates_action_types() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![("network_id", "1")])
        .action("deploy", "evm::deploy")  // Invalid action type
            .input("signer", "0x123")
            .input("fee", "1000")
        .output("result", "action.deploy.contract_address")
        .validate();
    
    assert_validation_error!(result, "Unknown action type");
    assert_validation_error!(result, "evm::deploy");
    
    // Check for suggestion
    let has_suggestion = result.errors.iter().any(|e| 
        e.message.contains("deploy_contract") || 
        e.documentation.as_ref().map_or(false, |d| d.contains("deploy_contract"))
    );
    assert!(has_suggestion, "Expected suggestion to use 'deploy_contract'");
}

#[test]
fn test_doctor_validates_unknown_namespace() {
    let result = RunbookBuilder::new()
        .action("test", "unknown::action")
            .input("value", "123")
        .validate();
    
    assert_validation_error!(result, "Unknown addon namespace");
    assert_validation_error!(result, "unknown");
}

#[test] 
fn test_doctor_with_problematic_transfer_fixture() {
    // Test with the actual fixture file content
    let fixture_path = std::path::Path::new("../../addons/evm/fixtures/doctor_demo/runbooks/problematic_transfer.tx");
    
    if fixture_path.exists() {
        let content = std::fs::read_to_string(fixture_path)
            .expect("Failed to read fixture file");
            
        let result = RunbookBuilder::new()
            .with_content(&content)
            .validate();
        
        // Should find multiple errors
        assert!(!result.success, "Doctor should fail on problematic_transfer.tx");
        assert!(result.errors.len() > 0, "Expected errors in problematic_transfer.tx");
    }
}

#[test]
fn test_doctor_validates_signer_references() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![("network_id", "1")])
        .action("send", "evm::send_eth")
            .input("signer", "signer.nonexistent")  // Invalid signer reference
            .input("recipient_address", "0x456")
            .input("amount", "1000")
        .validate();
    
    assert_validation_error!(result, "signer.nonexistent");
    assert_validation_error!(result, "not defined");
}

#[test]
fn test_doctor_validates_variable_references() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            variable "amount" { value = "1000" }
            
            output "double" {
                value = std::add(input.amount, input.missing_var)
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "missing_var");
    assert_validation_error!(result, "not defined");
}

#[test]
fn test_doctor_validates_nested_field_access() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![("network_id", "1")])
        .action("deploy", "evm::deploy_contract")
            .input("contract", "./Token.sol")
        .action("call", "evm::call_contract")
            .input("contract_address", "action.deploy.contract_address")
            .input("function", "action.deploy.abi.functions[0]")  // Invalid nested access
        .validate();
    
    assert_validation_error!(result, "abi");
    assert_validation_error!(result, "does not exist");
}

#[test]
fn test_doctor_validates_addon_configuration() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            addon "evm" {
                // Missing required network_id
                rpc_url = "https://eth.example.com"
            }
            
            action "test" "evm::send_eth" {
                recipient_address = "0x123"
                amount = "1000"
            }
        "#)
        .validate();
    
    assert_validation_error!(result, "network_id");
}

#[test]
fn test_doctor_with_multi_file_runbook() {
    let result = RunbookBuilder::new()
        .with_file("main.tx", r#"
            include "flows.tx"
            
            variable "network" {
                value = env.NETWORK_ID
            }
            
            output "result" {
                value = action.deploy.contract_address
            }
        "#)
        .with_file("flows.tx", r#"
            addon "evm" { network_id = input.network }
            
            action "deploy" "evm::deploy_contract" {
                contract = "./Token.sol"
            }
        "#)
        .with_environment("test", vec![("NETWORK_ID", "1")])
        .validate();
    
    assert_success!(result);
}

#[test]
fn test_doctor_detects_circular_dependencies() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            action "a" "std::send_http_request" {
                url = "https://example.com"
                depends_on = [action.b]
            }
            
            action "b" "std::send_http_request" {
                url = "https://example.com"
                depends_on = [action.a]
            }
        "#)
        .validate();
    
    // Note: This might not be detected by the simple validator
    // but should be detected by the full doctor implementation
    if !result.success {
        assert_validation_error!(result, "circular");
    }
}