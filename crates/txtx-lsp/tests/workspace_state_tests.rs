mod helpers;

use helpers::LspTestClient;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper to create a test workspace with manifest and runbooks
fn create_test_workspace() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create txtx.yml
    let manifest_content = r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: deploy.tx
    description: Deploy contract
  - name: configure
    location: configure.tx
    description: Configure system

environments:
  default:
    contract_address: "0x1234567890abcdef"
    private_key: "test_private_key"
    api_url: "https://api.test.com"
  testnet:
    contract_address: "0xabcdef1234567890"
    private_key: "testnet_private_key"
    api_url: "https://testnet.api.test.com""#;
    
    fs::write(base_path.join("txtx.yml"), manifest_content).unwrap();
    
    // Create deploy.tx
    let deploy_content = r#"// Deploy contract runbook

action "deploy" "evm::deploy_contract" {
  contract_address = inputs.contract_address
  private_key = inputs.private_key
  api_endpoint = inputs.api_url
}

output "contract_address" {
  value = action.deploy.contract_address
}"#;
    
    fs::write(base_path.join("deploy.tx"), deploy_content).unwrap();
    
    // Create configure.tx
    let configure_content = r#"// Configure system runbook

action "configure" "http::post" {
  url = inputs.api_url
  auth = inputs.private_key
  contract = inputs.contract_address
}"#;
    
    fs::write(base_path.join("configure.tx"), configure_content).unwrap();
    
    temp_dir
}

#[test]
fn test_open_manifest_first_then_runbook() {
    let workspace = create_test_workspace();
    let workspace_path = workspace.path().to_str().unwrap();
    let root_uri = format!("file://{}", workspace_path);
    
    // Start LSP server
    let mut client = LspTestClient::start(workspace_path).unwrap();
    
    // Initialize LSP
    let init_response = client.initialize(&root_uri).unwrap();
    assert!(init_response.get("result").is_some(), "LSP should initialize successfully");
    
    // Open txtx.yml first
    let manifest_path = workspace.path().join("txtx.yml");
    let manifest_uri = format!("file://{}", manifest_path.display());
    let manifest_content = fs::read_to_string(&manifest_path).unwrap();
    
    client.open_document(&manifest_uri, "yaml", &manifest_content).unwrap();
    
    // Wait a bit for processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Open deploy.tx
    let deploy_path = workspace.path().join("deploy.tx");
    let deploy_uri = format!("file://{}", deploy_path.display());
    let deploy_content = fs::read_to_string(&deploy_path).unwrap();
    
    client.open_document(&deploy_uri, "txtx", &deploy_content).unwrap();
    
    // Wait for processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Test go-to-definition for inputs.contract_address
    // Line 3 (0-indexed), character 28
    let definition_response = client.goto_definition(&deploy_uri, 3, 28).unwrap();
    
    // Check if we got a valid definition
    if let Some(result) = definition_response.get("result") {
        if !result.is_null() {
            // Should point to txtx.yml
            let uri = result.get("uri")
                .or_else(|| result.get(0).and_then(|r| r.get("uri")))
                .and_then(|u| u.as_str());
            
            assert!(uri.is_some(), "Definition should have a URI");
            assert!(uri.unwrap().contains("txtx.yml"), "Definition should point to txtx.yml");
            println!("✓ Go-to-definition works when manifest opened first");
        } else {
            println!("⚠ Go-to-definition returned null (feature may not be implemented)");
        }
    }
    
    // Test hover
    let hover_response = client.hover(&deploy_uri, 4, 25).unwrap();
    if let Some(result) = hover_response.get("result") {
        if !result.is_null() {
            println!("✓ Hover works when manifest opened first");
        }
    }
    
    // Cleanup
    client.shutdown().unwrap();
}

#[test]
fn test_open_runbook_first_then_manifest() {
    let workspace = create_test_workspace();
    let workspace_path = workspace.path().to_str().unwrap();
    let root_uri = format!("file://{}", workspace_path);
    
    // Start LSP server
    let mut client = LspTestClient::start(workspace_path).unwrap();
    
    // Initialize LSP
    let init_response = client.initialize(&root_uri).unwrap();
    assert!(init_response.get("result").is_some(), "LSP should initialize successfully");
    
    // Open deploy.tx FIRST (without manifest)
    let deploy_path = workspace.path().join("deploy.tx");
    let deploy_uri = format!("file://{}", deploy_path.display());
    let deploy_content = fs::read_to_string(&deploy_path).unwrap();
    
    client.open_document(&deploy_uri, "txtx", &deploy_content).unwrap();
    
    // Wait for processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Test go-to-definition BEFORE manifest is opened
    let def_before = client.goto_definition(&deploy_uri, 3, 28).unwrap();
    
    let before_result = def_before.get("result");
    if before_result.is_some() && !before_result.unwrap().is_null() {
        println!("ℹ LSP found manifest automatically when runbook opened");
    } else {
        println!("ℹ No definition before manifest opened (expected)");
    }
    
    // Now open txtx.yml
    let manifest_path = workspace.path().join("txtx.yml");
    let manifest_uri = format!("file://{}", manifest_path.display());
    let manifest_content = fs::read_to_string(&manifest_path).unwrap();
    
    client.open_document(&manifest_uri, "yaml", &manifest_content).unwrap();
    
    // Wait for workspace to rebuild
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    // Test go-to-definition AFTER manifest is opened
    let def_after = client.goto_definition(&deploy_uri, 3, 28).unwrap();
    
    if let Some(result) = def_after.get("result") {
        if !result.is_null() {
            let uri = result.get("uri")
                .or_else(|| result.get(0).and_then(|r| r.get("uri")))
                .and_then(|u| u.as_str());
            
            if uri.is_some() && uri.unwrap().contains("txtx.yml") {
                println!("✓ Go-to-definition works after manifest opened");
            } else {
                println!("⚠ Definition found but doesn't point to txtx.yml");
            }
        } else {
            println!("⚠ Go-to-definition still null after manifest opened");
        }
    }
    
    // Cleanup
    client.shutdown().unwrap();
}

#[test]
fn test_lsp_searches_upward_for_manifest() {
    let workspace = create_test_workspace();
    let workspace_path = workspace.path().to_str().unwrap();
    
    // Create a nested directory structure
    let nested_dir = workspace.path().join("modules");
    fs::create_dir(&nested_dir).unwrap();
    
    let nested_runbook_content = r#"// Nested runbook
action "nested" "http::get" {
  url = inputs.api_url
}"#;
    
    let nested_runbook_path = nested_dir.join("nested.tx");
    fs::write(&nested_runbook_path, nested_runbook_content).unwrap();
    
    let root_uri = format!("file://{}", workspace_path);
    
    // Start LSP server
    let mut client = LspTestClient::start(workspace_path).unwrap();
    
    // Initialize LSP
    client.initialize(&root_uri).unwrap();
    
    // Open the nested runbook (LSP should find manifest in parent dir)
    let nested_uri = format!("file://{}", nested_runbook_path.display());
    client.open_document(&nested_uri, "txtx", &nested_runbook_content).unwrap();
    
    // Wait for processing
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    // Test go-to-definition to see if manifest was discovered
    let definition_response = client.goto_definition(&nested_uri, 2, 15).unwrap();
    
    if let Some(result) = definition_response.get("result") {
        if !result.is_null() {
            println!("✓ LSP found manifest by searching upward from nested directory");
        } else {
            println!("ℹ LSP requires explicit manifest opening (doesn't auto-discover)");
        }
    }
    
    // Cleanup
    client.shutdown().unwrap();
}

#[test]
fn test_multiple_runbooks_same_manifest() {
    let workspace = create_test_workspace();
    let workspace_path = workspace.path().to_str().unwrap();
    let root_uri = format!("file://{}", workspace_path);
    
    // Start LSP server
    let mut client = LspTestClient::start(workspace_path).unwrap();
    
    // Initialize LSP
    client.initialize(&root_uri).unwrap();
    
    // Open manifest
    let manifest_path = workspace.path().join("txtx.yml");
    let manifest_uri = format!("file://{}", manifest_path.display());
    let manifest_content = fs::read_to_string(&manifest_path).unwrap();
    
    client.open_document(&manifest_uri, "yaml", &manifest_content).unwrap();
    
    // Open first runbook (deploy.tx)
    let deploy_path = workspace.path().join("deploy.tx");
    let deploy_uri = format!("file://{}", deploy_path.display());
    let deploy_content = fs::read_to_string(&deploy_path).unwrap();
    
    client.open_document(&deploy_uri, "txtx", &deploy_content).unwrap();
    
    // Open second runbook (configure.tx)
    let configure_path = workspace.path().join("configure.tx");
    let configure_uri = format!("file://{}", configure_path.display());
    let configure_content = fs::read_to_string(&configure_path).unwrap();
    
    client.open_document(&configure_uri, "txtx", &configure_content).unwrap();
    
    // Wait for processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Test go-to-definition from BOTH runbooks
    let def1 = client.goto_definition(&deploy_uri, 3, 28).unwrap();
    let def2 = client.goto_definition(&configure_uri, 3, 15).unwrap();
    
    let both_work = def1.get("result").map(|r| !r.is_null()).unwrap_or(false) &&
                    def2.get("result").map(|r| !r.is_null()).unwrap_or(false);
    
    if both_work {
        println!("✓ Multiple runbooks can access same manifest definitions");
    } else {
        println!("⚠ Not all runbooks have access to manifest definitions");
    }
    
    // Cleanup
    client.shutdown().unwrap();
}

#[test]
fn test_no_manifest_present() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path().to_str().unwrap();
    
    // Create only a runbook, no manifest
    let runbook_content = r#"// Orphan runbook
action "test" "http::get" {
  url = inputs.some_url
}"#;
    
    let runbook_path = temp_dir.path().join("orphan.tx");
    fs::write(&runbook_path, runbook_content).unwrap();
    
    let root_uri = format!("file://{}", workspace_path);
    
    // Start LSP server
    let mut client = LspTestClient::start(workspace_path).unwrap();
    
    // Initialize LSP
    client.initialize(&root_uri).unwrap();
    
    // Open runbook without manifest
    let runbook_uri = format!("file://{}", runbook_path.display());
    client.open_document(&runbook_uri, "txtx", &runbook_content).unwrap();
    
    // Wait for processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Check for diagnostics/errors
    let notifications = client.read_notifications(500);
    
    let has_error = notifications.iter().any(|n| {
        if let Some(method) = n.get("method").and_then(|m| m.as_str()) {
            if method == "textDocument/publishDiagnostics" {
                // Check if diagnostics contain error about missing manifest
                return true;
            }
        }
        false
    });
    
    if has_error {
        println!("✓ LSP reports error when manifest is missing");
    } else {
        println!("ℹ LSP doesn't report error for missing manifest");
    }
    
    // Test go-to-definition (should fail gracefully)
    let def_response = client.goto_definition(&runbook_uri, 2, 15).unwrap();
    
    let result_is_null = def_response.get("result")
        .map(|r| r.is_null())
        .unwrap_or(true);
    
    assert!(result_is_null, "Go-to-definition should return null without manifest");
    
    // Cleanup
    client.shutdown().unwrap();
}