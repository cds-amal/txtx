use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_doctor_multi_file_runbook_with_flow() {
    // Run doctor on a multi-file runbook where flows are defined
    let output = Command::new("cargo")
        .args(&["run", "--bin", "txtx", "--", "doctor", "deploy"])
        .current_dir("tests/fixtures/doctor/separate-flows-file")
        .output()
        .expect("Failed to execute doctor command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should succeed without errors
    assert!(output.status.success(), "Doctor command failed: stdout={}, stderr={}", stdout, stderr);
    assert!(!stdout.contains("error"), "Doctor found unexpected errors: {}", stdout);
}

#[test]
fn test_doctor_multi_file_runbook_missing_flow() {
    // First, create a test fixture without flows
    let test_dir = Path::new("tests/fixtures/doctor/test-missing-flow");
    let deploy_dir = test_dir.join("deploy");
    
    // Clean up from previous runs
    if test_dir.exists() {
        fs::remove_dir_all(test_dir).ok();
    }
    
    // Create the directory structure
    fs::create_dir_all(&deploy_dir).expect("Failed to create test directory");
    
    // Create txtx.yml
    fs::write(
        test_dir.join("txtx.yml"),
        r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: ./deploy
    description: Deploy contract

environments:
  global:
    private_key: "test_private_key"
"#
    ).expect("Failed to write txtx.yml");
    
    // Create main.tx with flow reference but no flow defined
    fs::write(
        deploy_dir.join("main.tx"),
        r#"action "test" "evm::deploy_contract" {
  constructor_args = [
    flow.chain_id
  ]
}
"#
    ).expect("Failed to write main.tx");
    
    // Run doctor
    let output = Command::new("cargo")
        .args(&["run", "--bin", "txtx", "--", "doctor", "deploy"])
        .current_dir(&test_dir)
        .output()
        .expect("Failed to execute doctor command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should fail with specific error about missing flow
    assert!(!output.status.success(), "Doctor should have failed");
    assert!(stdout.contains("Reference to flow.chain_id but no flows are defined"), 
            "Expected error about missing flow, got: {}", stdout);
    
    // Clean up
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_doctor_multi_file_runbook_missing_flow_attribute() {
    // Create a test fixture with flow but missing attribute
    let test_dir = Path::new("tests/fixtures/doctor/test-missing-attr");
    let deploy_dir = test_dir.join("deploy");
    
    // Clean up from previous runs
    if test_dir.exists() {
        fs::remove_dir_all(test_dir).ok();
    }
    
    // Create the directory structure
    fs::create_dir_all(&deploy_dir).expect("Failed to create test directory");
    
    // Create txtx.yml
    fs::write(
        test_dir.join("txtx.yml"),
        r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: ./deploy
    description: Deploy contract

environments:
  global:
    private_key: "test_private_key"
"#
    ).expect("Failed to write txtx.yml");
    
    // Create main.tx with flow.chain_id reference
    fs::write(
        deploy_dir.join("main.tx"),
        r#"action "test" "evm::deploy_contract" {
  constructor_args = [
    flow.chain_id
  ]
}
"#
    ).expect("Failed to write main.tx");
    
    // Create flows.tx with flow but missing chain_id attribute
    fs::write(
        deploy_dir.join("flows.tx"),
        r#"flow "testnet" {
  network_id = 5
}
"#
    ).expect("Failed to write flows.tx");
    
    // Run doctor
    let output = Command::new("cargo")
        .args(&["run", "--bin", "txtx", "--", "doctor", "deploy"])
        .current_dir(&test_dir)
        .output()
        .expect("Failed to execute doctor command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should fail with error about missing attribute
    assert!(!output.status.success(), "Doctor should have failed");
    assert!(stdout.contains("chain_id") && stdout.contains("not defined"), 
            "Expected error about missing chain_id attribute, got: {}", stdout);
    
    // Clean up
    fs::remove_dir_all(test_dir).ok();
}