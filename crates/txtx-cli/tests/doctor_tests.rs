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

#[test]
fn test_doctor_finds_undefined_action() {
    let runbook = r#"
        output "bad" {
            value = action.nonexistent.result
        }
    "#;
    
    let (output, success) = run_doctor_on_file(runbook);
    assert!(!success, "Doctor should fail on undefined action");
    assert!(output.contains("undefined action") || output.contains("Reference to undefined"));
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
    
    let (output, success) = run_doctor_on_file(runbook);
    assert!(!success, "Doctor should fail on invalid field access");
    assert!(output.contains("Field 'from' does not exist"));
    assert!(output.contains("send_eth"));
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
    
    let (output, success) = run_doctor_on_file(runbook);
    assert!(success, "Doctor should pass on valid runbook: {}", output);
    assert!(output.contains("No issues found"));
}

#[test] 
fn test_doctor_with_problematic_transfer_fixture() {
    // Test with the actual fixture file
    let fixture_path = Path::new("../../addons/evm/fixtures/doctor_demo/runbooks/problematic_transfer.tx");
    
    if fixture_path.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_txtx"))
            .arg("doctor")
            .arg(fixture_path)
            .output()
            .expect("Failed to run doctor command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        assert!(!output.status.success(), "Doctor should fail on problematic_transfer.tx");
        
        // Should find multiple errors
        let full_output = format!("{}\n{}", stdout, stderr);
        assert!(full_output.contains("Found") && full_output.contains("issue(s)"));
    }
}