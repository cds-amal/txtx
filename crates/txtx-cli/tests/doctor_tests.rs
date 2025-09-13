// Integration tests for the doctor command
// These tests run the actual txtx binary

use std::process::Command;
use std::path::Path;
use std::fs;
use std::env;

/// Helper to run the doctor command on a file
fn run_doctor_on_file(content: &str) -> (String, bool) {
    // Create a temporary file with a unique name using timestamp and random number
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let test_file = format!("test_doctor_{}_{}.tx", std::process::id(), timestamp);
    let file_path = Path::new(&test_file);
    
    // Write test content
    fs::write(&file_path, content).expect("Failed to write test file");
    
    // Run doctor command with empty stdin to avoid hanging
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
    
    let success = output.status.success();
    
    (full_output, success)
}

/// Helper to run the doctor command with JSON output
fn run_doctor_json(content: &str) -> (serde_json::Value, bool) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let test_file = format!("test_doctor_{}_{}.tx", std::process::id(), timestamp);
    let file_path = Path::new(&test_file);
    
    fs::write(&file_path, content).expect("Failed to write test file");
    
    let output = Command::new(env!("CARGO_BIN_EXE_txtx"))
        .arg("doctor")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("Failed to run doctor command");
    
    let _ = fs::remove_file(&file_path);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| {
            eprintln!("Failed to parse JSON: {}", e);
            eprintln!("Output was: {}", stdout);
            serde_json::json!({})
        });
    
    (json, output.status.success())
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

#[test]
fn test_doctor_finds_invalid_send_eth_field() {
    let runbook = r#"
        addon "evm" { network_id = 1 }
        
        action "send" "evm::send_eth" {
            from = "0x123"
            to = "0x456"
            value = "1000"
        }
        
        output "bad" {
            value = action.send.from
        }
    "#;
    
    let (json, success) = run_doctor_json(runbook);
    assert!(!success, "Doctor should fail on invalid field access");
    
    let errors = json["errors"].as_array().expect("Expected errors array");
    assert_eq!(errors.len(), 1, "Expected exactly one error");
    
    let error = &errors[0];
    assert!(error["message"].as_str().unwrap().contains("Field 'from' does not exist"));
    assert!(error["message"].as_str().unwrap().contains("send_eth"));
}

#[test]
fn test_doctor_passes_valid_runbook() {
    let runbook = r#"
        addon "evm" { network_id = 1 }
        
        action "send" "evm::send_eth" {
            from = "0x123"
            to = "0x456"
            value = "1000"
        }
        
        output "tx" {
            value = action.send.tx_hash
        }
    "#;
    
    let (json, success) = run_doctor_json(runbook);
    assert!(success, "Doctor should pass on valid runbook");
    
    // When successful, JSON output should still be valid with empty arrays
    if let Some(errors) = json["errors"].as_array() {
        assert_eq!(errors.len(), 0, "Expected no errors");
    }
    if let Some(warnings) = json["warnings"].as_array() {
        assert_eq!(warnings.len(), 0, "Expected no warnings");
    }
}

#[test]
fn test_doctor_validates_action_types() {
    let runbook = r#"
        addon "evm" { network_id = 1 }
        
        action "deploy" "evm::deploy" {
            signer = "0x123"
            fee = 1000
        }
        
        output "result" {
            value = action.deploy.contract_address
        }
    "#;
    
    let (json, success) = run_doctor_json(runbook);
    assert!(!success, "Doctor should fail on unknown action type");
    
    let errors = json["errors"].as_array().expect("Expected errors array");
    assert!(errors.len() >= 1, "Expected at least one error");
    
    // Check that we got an error about the unknown action type
    let has_unknown_action_error = errors.iter().any(|e| {
        let msg = e["message"].as_str().unwrap_or("");
        msg.contains("Unknown action type") && msg.contains("evm::deploy")
    });
    assert!(has_unknown_action_error, "Expected error about unknown action type 'evm::deploy'");
    
    // Check for suggestion
    let has_deploy_contract_suggestion = errors.iter().any(|e| {
        if let Some(ctx) = e["context"].as_str() {
            ctx.contains("deploy_contract")
        } else {
            false
        }
    });
    assert!(has_deploy_contract_suggestion, "Expected suggestion to use 'deploy_contract'");
}

#[test]
fn test_doctor_validates_unknown_namespace() {
    let runbook = r#"
        action "test" "unknown::action" {
            value = 123
        }
    "#;
    
    let (json, success) = run_doctor_json(runbook);
    assert!(!success, "Doctor should fail on unknown namespace");
    
    let errors = json["errors"].as_array().expect("Expected errors array");
    assert!(errors.len() >= 1, "Expected at least one error");
    
    let error = &errors[0];
    assert!(error["message"].as_str().unwrap().contains("Unknown addon namespace"));
    assert!(error["message"].as_str().unwrap().contains("unknown"));
}

#[test] 
fn test_doctor_with_problematic_transfer_fixture() {
    // Test with the actual fixture file
    let fixture_path = Path::new("../../addons/evm/fixtures/doctor_demo/runbooks/problematic_transfer.tx");
    
    if fixture_path.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_txtx"))
            .arg("doctor")
            .arg(fixture_path)
            .arg("--format")
            .arg("json")
            .output()
            .expect("Failed to run doctor command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Failed to parse JSON output");
        
        assert!(!output.status.success(), "Doctor should fail on problematic_transfer.tx");
        
        // Should find multiple errors
        let errors = json["errors"].as_array().expect("Expected errors array");
        assert!(errors.len() > 0, "Expected errors in problematic_transfer.tx");
    }
}