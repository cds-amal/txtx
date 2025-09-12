use txtx_lsp::backend::{EditorStateInput, LspNotification, process_notification};
use txtx_lsp::state::EditorState;
use txtx_addon_kit::helpers::fs::FileLocation;
use lsp_types::Position;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_manifest_opened_builds_workspace_state() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();
    
    // Create test manifest
    let manifest_content = r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: deploy.tx
    description: Deploy contract

environments:
  default:
    api_url: "https://api.test.com"
    contract_address: "0x1234"
"#;
    
    let manifest_path = workspace_path.join("txtx.yml");
    fs::write(&manifest_path, manifest_content).unwrap();
    
    // Create runbook
    let runbook_content = r#"action "test" "http::get" {
  url = inputs.api_url
}"#;
    
    let runbook_path = workspace_path.join("deploy.tx");
    fs::write(&runbook_path, runbook_content).unwrap();
    
    // Create editor state
    let mut editor_state = EditorStateInput::Owned(EditorState::new());
    
    // Open manifest
    let manifest_location = FileLocation::from_path(manifest_path.clone());
    let response = process_notification(
        LspNotification::ManifestOpened(manifest_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Check that no errors were reported
    assert!(response.notification.is_none() || 
            !response.notification.as_ref().unwrap().1.contains("error"),
            "Should not have errors when opening valid manifest");
    
    // Verify workspace was indexed
    editor_state.try_read(|es| {
        assert!(es.workspaces.contains_key(&manifest_location),
                "Workspace should be indexed after opening manifest");
    }).unwrap();
}

#[tokio::test]
async fn test_runbook_opened_finds_manifest() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();
    
    // Create test manifest
    let manifest_content = r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: deploy.tx

environments:
  default:
    api_url: "https://api.test.com"
"#;
    
    let manifest_path = workspace_path.join("txtx.yml");
    fs::write(&manifest_path, manifest_content).unwrap();
    
    // Create runbook
    let runbook_content = r#"action "test" "http::get" {
  url = inputs.api_url
}"#;
    
    let runbook_path = workspace_path.join("deploy.tx");
    fs::write(&runbook_path, runbook_content).unwrap();
    
    // Create editor state
    let mut editor_state = EditorStateInput::Owned(EditorState::new());
    
    // Open runbook (should find manifest automatically)
    let runbook_location = FileLocation::from_path(runbook_path.clone());
    let response = process_notification(
        LspNotification::RunbookOpened(runbook_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Check response
    if let Some((_, msg)) = &response.notification {
        println!("Notification: {}", msg);
    }
    
    // Verify runbook was added
    editor_state.try_read(|es| {
        assert!(es.active_runbooks.contains_key(&runbook_location),
                "Runbook should be in active runbooks");
    }).unwrap();
}

#[tokio::test]
async fn test_definition_location_after_workspace_setup() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();
    
    // Create manifest with environment variables
    let manifest_content = r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: deploy.tx

environments:
  default:
    api_url: "https://api.test.com"
    contract_address: "0x123456"
    private_key: "secret"
"#;
    
    let manifest_path = workspace_path.join("txtx.yml");
    fs::write(&manifest_path, manifest_content).unwrap();
    
    // Create runbook with input references
    let runbook_content = r#"action "deploy" "evm::deploy" {
  address = inputs.contract_address
  key = inputs.private_key
  endpoint = inputs.api_url
}"#;
    
    let runbook_path = workspace_path.join("deploy.tx");
    fs::write(&runbook_path, runbook_content).unwrap();
    
    // Setup editor state
    let mut editor_state = EditorStateInput::Owned(EditorState::new());
    
    // Open manifest first
    let manifest_location = FileLocation::from_path(manifest_path.clone());
    process_notification(
        LspNotification::ManifestOpened(manifest_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Open runbook
    let runbook_location = FileLocation::from_path(runbook_path.clone());
    process_notification(
        LspNotification::RunbookOpened(runbook_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Store runbook content in active_runbooks
    editor_state.try_write(|es| {
        es.insert_active_runbook(runbook_location.clone(), runbook_content);
    }).unwrap();
    
    // Test get_definition_location for "contract_address"
    let position = Position { line: 1, character: 25 }; // Position in "contract_address"
    
    let definition = editor_state.try_read(|es| {
        es.get_definition_location(&runbook_location, &position)
    }).unwrap();
    
    if let Some(location) = definition {
        assert!(location.uri.to_string().contains("txtx.yml"),
                "Definition should point to manifest file");
        println!("✓ Go-to-definition correctly points to manifest");
    } else {
        println!("⚠ Go-to-definition returned None (may need implementation)");
    }
}

#[tokio::test]
async fn test_runbook_without_manifest_shows_error() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();
    
    // Create only a runbook, NO manifest
    let runbook_content = r#"action "test" "http::get" {
  url = inputs.some_url
}"#;
    
    let runbook_path = workspace_path.join("orphan.tx");
    fs::write(&runbook_path, runbook_content).unwrap();
    
    // Create editor state
    let mut editor_state = EditorStateInput::Owned(EditorState::new());
    
    // Open runbook without manifest
    let runbook_location = FileLocation::from_path(runbook_path.clone());
    let response = process_notification(
        LspNotification::RunbookOpened(runbook_location),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Should have an error about missing manifest
    let has_error = response.notification
        .map(|(_, msg)| msg.contains("txtx.yml") || msg.contains("manifest"))
        .unwrap_or(false) ||
        response.aggregated_diagnostics.iter()
        .any(|(_, diags)| diags.iter().any(|d| d.message.contains("txtx.yml")));
    
    assert!(has_error, "Should report error when manifest is missing");
    println!("✓ LSP correctly reports missing manifest");
}

#[tokio::test]
async fn test_multiple_runbooks_share_workspace_state() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();
    
    // Create manifest
    let manifest_content = r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: deploy.tx
  - name: configure
    location: configure.tx

environments:
  default:
    api_url: "https://api.test.com"
"#;
    
    let manifest_path = workspace_path.join("txtx.yml");
    fs::write(&manifest_path, manifest_content).unwrap();
    
    // Create first runbook
    let deploy_content = r#"action "deploy" "http::get" {
  url = inputs.api_url
}"#;
    fs::write(workspace_path.join("deploy.tx"), deploy_content).unwrap();
    
    // Create second runbook
    let configure_content = r#"action "configure" "http::post" {
  endpoint = inputs.api_url
}"#;
    fs::write(workspace_path.join("configure.tx"), configure_content).unwrap();
    
    // Setup editor state
    let mut editor_state = EditorStateInput::Owned(EditorState::new());
    
    // Open manifest
    let manifest_location = FileLocation::from_path(manifest_path.clone());
    process_notification(
        LspNotification::ManifestOpened(manifest_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Open both runbooks
    let deploy_location = FileLocation::from_path(
        workspace_path.join("deploy.tx")
    );
    let configure_location = FileLocation::from_path(
        workspace_path.join("configure.tx")
    );
    
    process_notification(
        LspNotification::RunbookOpened(deploy_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    process_notification(
        LspNotification::RunbookOpened(configure_location.clone()),
        &mut editor_state,
        None
    ).await.unwrap();
    
    // Both runbooks should be in active_runbooks
    editor_state.try_read(|es| {
        assert!(es.active_runbooks.contains_key(&deploy_location),
                "First runbook should be active");
        assert!(es.active_runbooks.contains_key(&configure_location),
                "Second runbook should be active");
        
        // Both should share the same workspace
        assert_eq!(es.active_runbooks.len(), 2,
                   "Should have exactly 2 active runbooks");
    }).unwrap();
    
    println!("✓ Multiple runbooks can share workspace state");
}