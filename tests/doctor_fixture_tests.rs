// Integration tests for doctor command using fixtures
use std::process::Command;
use std::path::Path;
use std::fs;

#[test]
fn test_doctor_fixtures() {
    let fixtures_dir = Path::new("tests/fixtures/doctor");
    
    // Define expected outcomes for each fixture
    let test_cases = vec![
        ("test_doctor_valid.tx", true, 0, "Valid runbook should pass"),
        ("test_doctor_simple.tx", false, 2, "Should find undefined signer and invalid field"),
        ("test_doctor_two_pass.tx", false, 1, "Should find undefined action reference"),
        ("test_doctor_bad_flow_detection.tx", false, 1, "Should find unknown action type"),
        ("test_doctor_errors.tx", false, -1, "Should find multiple errors"), // -1 means any number
    ];
    
    for (fixture, should_pass, expected_errors, description) in test_cases {
        let fixture_path = fixtures_dir.join(fixture);
        if !fixture_path.exists() {
            eprintln!("Skipping missing fixture: {}", fixture);
            continue;
        }
        
        let output = Command::new(env!("CARGO_BIN_EXE_txtx"))
            .arg("doctor")
            .arg(&fixture_path)
            .arg("--format")
            .arg("json")
            .stdin(std::process::Stdio::piped())
            .output()
            .expect("Failed to run doctor command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let success = output.status.success();
        
        // Parse JSON output
        let json: serde_json::Value = if stdout.trim().is_empty() {
            serde_json::json!({"errors": [], "warnings": []})
        } else {
            serde_json::from_str(&stdout).unwrap_or_else(|e| {
                panic!("Failed to parse JSON for {}: {}\nOutput: {}", fixture, e, stdout);
            })
        };
        
        let errors = json["errors"].as_array().map(|a| a.len()).unwrap_or(0);
        
        // Check pass/fail status
        assert_eq!(
            success, should_pass,
            "{} - {}: expected {}, got {}",
            fixture, description,
            if should_pass { "pass" } else { "fail" },
            if success { "pass" } else { "fail" }
        );
        
        // Check error count if specified
        if expected_errors >= 0 {
            assert_eq!(
                errors, expected_errors as usize,
                "{} - {}: expected {} errors, got {}",
                fixture, description, expected_errors, errors
            );
        }
    }
}

#[test]
fn test_doctor_fixture_specific_errors() {
    // Test specific error messages for key fixtures
    let fixtures_dir = Path::new("tests/fixtures/doctor");
    
    // Test bad flow detection
    let bad_flow_path = fixtures_dir.join("test_doctor_bad_flow_detection.tx");
    if bad_flow_path.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_txtx"))
            .arg("doctor")
            .arg(&bad_flow_path)
            .arg("--format")
            .arg("json")
            .output()
            .expect("Failed to run doctor");
        
        let json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))
            .expect("Failed to parse JSON");
        
        let errors = json["errors"].as_array().expect("Expected errors array");
        
        // Should detect evm::deploy doesn't exist
        let has_deploy_error = errors.iter().any(|e| {
            let msg = e["message"].as_str().unwrap_or("");
            msg.contains("Unknown action type") && msg.contains("evm::deploy")
        });
        assert!(has_deploy_error, "Should detect unknown action type 'evm::deploy'");
    }
}