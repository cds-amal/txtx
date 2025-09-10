use std::collections::HashMap;
use std::path::{Path, PathBuf};

use txtx_core::kit::types::commands::{CommandSpecification, PreCommandSpecification};
use txtx_core::manifest::WorkspaceManifest;
use txtx_parser::{parse, Runbook, Expression};

pub(crate) mod parser_validator;
use parser_validator::LocatedInputRef;

#[derive(Debug)]
pub struct DoctorResult {
    pub errors: Vec<DoctorError>,
    pub warnings: Vec<DoctorWarning>,
    pub suggestions: Vec<DoctorSuggestion>,
}

#[derive(Debug)]
pub struct DoctorError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub context: Option<String>,
    pub documentation_link: Option<String>,
}

#[derive(Debug)]
pub struct DoctorWarning {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub suggestion: Option<String>,
}

#[derive(Debug)]
pub struct DoctorSuggestion {
    pub message: String,
    pub example: Option<String>,
}

/// Main entry point for the doctor command
pub fn run_doctor(
    manifest_path: Option<String>,
    runbook_name: Option<String>,
    environment: Option<String>,
    cli_inputs: Vec<(String, String)>,
    format: crate::cli::DoctorOutputFormat,
) -> Result<(), String> {
    // Debug message only in pretty format
    if matches!(format, crate::cli::DoctorOutputFormat::Pretty) {
        // Suppress diagnostic output for non-pretty formats
        if matches!(format, crate::cli::DoctorOutputFormat::Pretty) {
            eprintln!("Doctor command running with runbook: {:?}", runbook_name);
        }
    }
    let manifest_path = manifest_path.unwrap_or_else(|| "./txtx.yml".to_string());
    
    // Auto-detect format if needed
    let format = match format {
        crate::cli::DoctorOutputFormat::Auto => {
            // Check environment variable first
            if let Ok(env_format) = std::env::var("TXTX_DOCTOR_FORMAT") {
                match env_format.to_lowercase().as_str() {
                    "quickfix" => crate::cli::DoctorOutputFormat::Quickfix,
                    "json" => crate::cli::DoctorOutputFormat::Json,
                    "pretty" => crate::cli::DoctorOutputFormat::Pretty,
                    _ => {
                        // Fall back to auto-detection
                        if !atty::is(atty::Stream::Stdout) || std::env::var("CI").is_ok() {
                            crate::cli::DoctorOutputFormat::Quickfix
                        } else {
                            crate::cli::DoctorOutputFormat::Pretty
                        }
                    }
                }
            } else {
                // Check if output is being piped or we're in CI
                if !atty::is(atty::Stream::Stdout) || std::env::var("CI").is_ok() {
                    crate::cli::DoctorOutputFormat::Quickfix
                } else {
                    crate::cli::DoctorOutputFormat::Pretty
                }
            }
        }
        other => other,
    };

    // If a specific runbook is specified, try to find it
    if let Some(runbook_name) = runbook_name {
        // First try as a direct file path
        let path = PathBuf::from(&runbook_name);
        if path.exists() && path.extension().map_or(false, |ext| ext == "tx") {
            // When analyzing a direct file without manifest context
            analyze_runbook_file_with_context(&path, None, environment.as_ref(), &cli_inputs, &format)?;
        } else {
            // Try to load from manifest
            let manifest = match crate::cli::runbooks::load_workspace_manifest_from_manifest_path(
                &manifest_path,
            ) {
                Ok(m) => m,
                Err(_) => {
                    // If no manifest, treat as file path error
                    return Err(format!("File '{}' not found or is not a .tx file", runbook_name));
                }
            };

            // Look for the runbook in the manifest
            let mut found = false;
            for metadata in &manifest.runbooks {
                if metadata.name == runbook_name {
                    // Found the runbook, analyze its location
                    let base_path = std::path::Path::new(&manifest_path)
                        .parent()
                        .unwrap_or(std::path::Path::new("."));
                    let runbook_path = base_path.join(&metadata.location);

                    // Check if it's a directory or a file
                    if runbook_path.is_dir() {
                        // Look for main.tx in the directory
                        let main_file = runbook_path.join("main.tx");
                        if main_file.exists() {
                            analyze_runbook_file_with_context(
                                &main_file,
                                Some(&manifest),
                                environment.as_ref(),
                                &cli_inputs,
                                &format,
                            )?;
                            found = true;
                        } else {
                            return Err(format!(
                                "No main.tx found in runbook directory: {}",
                                runbook_path.display()
                            ));
                        }
                    } else if runbook_path.exists() {
                        analyze_runbook_file_with_context(
                            &runbook_path,
                            Some(&manifest),
                            environment.as_ref(),
                            &cli_inputs,
                            &format,
                        )?;
                        found = true;
                    } else {
                        // Try with .tx extension
                        let runbook_path_tx = runbook_path.with_extension("tx");
                        if runbook_path_tx.exists() {
                            analyze_runbook_file_with_context(
                                &runbook_path_tx,
                                Some(&manifest),
                                environment.as_ref(),
                                &cli_inputs,
                                &format,
                            )?;
                            found = true;
                        }
                    }
                    break;
                }
            }

            if !found {
                return Err(format!(
                    "Runbook '{}' not found in manifest or as a file",
                    runbook_name
                ));
            }
        }
    } else {
        // No specific runbook, check all runbooks in manifest if it exists
        if let Ok(manifest) =
            crate::cli::runbooks::load_workspace_manifest_from_manifest_path(&manifest_path)
        {
            if manifest.runbooks.is_empty() {
                if matches!(format, crate::cli::DoctorOutputFormat::Pretty) {
                    println!("No runbooks found in manifest.");
                }
            } else {
                let mut any_errors = false;
                for metadata in &manifest.runbooks {
                    if matches!(format, crate::cli::DoctorOutputFormat::Pretty) {
                        println!("Checking runbook '{}'...", metadata.name);
                    }

                    let base_path = std::path::Path::new(&manifest_path)
                        .parent()
                        .unwrap_or(std::path::Path::new("."));
                    let runbook_path = base_path.join(&metadata.location);

                    // Check if it's a directory or a file
                    let file_to_check = if runbook_path.is_dir() {
                        runbook_path.join("main.tx")
                    } else if runbook_path.exists() {
                        runbook_path
                    } else {
                        runbook_path.with_extension("tx")
                    };

                    if file_to_check.exists() {
                        if analyze_runbook_file_with_context(
                            &file_to_check,
                            Some(&manifest),
                            environment.as_ref(),
                            &cli_inputs,
                            &format,
                        )
                        .is_err()
                        {
                            any_errors = true;
                        }
                    } else {
                        eprintln!(
                            "Warning: Runbook '{}' file not found at {}",
                            metadata.name,
                            file_to_check.display()
                        );
                    }
                    if matches!(format, crate::cli::DoctorOutputFormat::Pretty) {
                        println!();
                    }
                }

                if any_errors {
                    return Err("Doctor found errors in one or more runbooks".to_string());
                }
            }
        } else {
            // No manifest, try to find runbooks in current directory
            let current_dir = std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?;

            let possible_files = vec!["main.tx", "txtx.tx", "runbook.tx"];
            let mut found = false;

            for file in possible_files {
                let path = current_dir.join(file);
                if path.exists() {
                    analyze_runbook_file(&path, &format)?;
                    found = true;
                    break;
                }
            }

            if !found {
                // Look for any .tx files in current directory
                let entries = std::fs::read_dir(&current_dir)
                    .map_err(|e| format!("Failed to read directory: {}", e))?;

                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "tx") {
                    analyze_runbook_file(&path, &format)?;
                            found = true;
                            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to validate fixture content
    fn validate_fixture(content: &str) -> DoctorResult {
        let runbook = parse(content).expect("Failed to parse test runbook");
        let mut result = DoctorResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        let _ = parser_validator::validate_with_visitor(&runbook, &mut result, "test.tx");
        result
    }

    #[test]
    fn test_problematic_transfer() {
        let content = include_str!("../../../../../addons/evm/fixtures/doctor_demo/runbooks/problematic_transfer.tx");
        let result = validate_fixture(content);

        // This fixture has 4 errors: from, to, value, gas_used
        assert_eq!(result.errors.len(), 4, "Expected 4 errors in problematic_transfer.tx");

        // Check specific errors
        let error_messages: Vec<_> = result.errors.iter()
            .map(|e| &e.message)
            .collect();

        assert!(error_messages.iter().any(|m| m.contains("from")));
        assert!(error_messages.iter().any(|m| m.contains("to")));
        assert!(error_messages.iter().any(|m| m.contains("value")));
        assert!(error_messages.iter().any(|m| m.contains("gas_used")));

        // Should have suggestions for accessing transaction details
        assert!(!result.suggestions.is_empty());
    }

    #[test]
    fn test_correct_transfer() {
        let content = include_str!("../../../../../addons/evm/fixtures/doctor_demo/runbooks/correct_transfer.tx");
        let result = validate_fixture(content);

        // This fixture should have no errors
        assert_eq!(result.errors.len(), 0, "Expected no errors in correct_transfer.tx");
        assert_eq!(result.warnings.len(), 0, "Expected no warnings in correct_transfer.tx");
    }

    #[test]
    fn test_undefined_action() {
        // Take a valid fixture and break it by referencing undefined action
        let valid = include_str!("../../../../../addons/evm/fixtures/doctor_demo/runbooks/correct_transfer.tx");
        let broken = valid.replace("action.transfer.tx_hash", "action.nonexistent.tx_hash");

        let result = validate_fixture(&broken);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("undefined action"));
        assert!(result.errors[0].context.is_some());
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
        assert!(result.errors[0].message.contains("only outputs: tx_hash"));
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
        use txtx_core::manifest::WorkspaceManifest;
        use txtx_core::indexmap::IndexMap;
        
        let runbook_content = r#"
            addon "evm" {
                network_id = input.CHAIN_ID
                rpc_api_url = input.RPC_URL
            }
            
            action "send" "evm::send_eth" {
                value = input.AMOUNT
            }
        "#;
        
        let runbook = parse(runbook_content).expect("Failed to parse");
        
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
        
        let mut result = DoctorResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        // Test with dev environment - should inherit RPC_URL from global
        validate_inputs_against_manifest(
            &runbook,
            &manifest,
            Some(&"dev".to_string()),
            &mut result,
            Path::new("test.tx"),
            &[],
        );
        
        assert_eq!(result.errors.len(), 0, "All inputs should be found through inheritance");
        assert!(result.suggestions.iter().any(|s| 
            s.message.contains("inherits from 'global'")),
            "Should mention environment inheritance"
        );
    }

    #[test]
    fn test_missing_input_in_environment() {
        use txtx_core::manifest::WorkspaceManifest;
        use txtx_core::indexmap::IndexMap;
        
        let runbook_content = r#"
            output "test" {
                value = input.MISSING_VAR
            }
        "#;
        
        let runbook = parse(runbook_content).expect("Failed to parse");
        
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
        
        let mut result = DoctorResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        validate_inputs_against_manifest(
            &runbook,
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
        use txtx_core::manifest::WorkspaceManifest;
        use txtx_core::indexmap::IndexMap;
        
        let runbook_content = r#"
            output "test" {
                value = input.MY_VAR
            }
        "#;
        
        let runbook = parse(runbook_content).expect("Failed to parse");
        
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
        
        let mut result = DoctorResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        validate_inputs_against_manifest(
            &runbook,
            &manifest,
            None,
            &mut result,
            Path::new("test.tx"),
            &[],
        );
        
        assert!(result.suggestions.iter().any(|s| 
            s.message.contains("CLI take precedence")),
            "Should mention CLI precedence"
        );
    }

    #[test]
    fn test_cli_inputs_override_environment() {
        use txtx_core::manifest::WorkspaceManifest;
        use txtx_core::indexmap::IndexMap;
        
        let runbook_content = r#"
            output "test" {
                value = input.MY_VAR
            }
            output "test2" {
                value = input.ANOTHER_VAR
            }
        "#;
        
        let runbook = parse(runbook_content).expect("Failed to parse");
        
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
        
        let mut result = DoctorResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        // Provide both values via CLI
        let cli_inputs = vec![
            ("MY_VAR".to_string(), "cli_override".to_string()),
            ("ANOTHER_VAR".to_string(), "cli_provided".to_string()),
        ];
        
        validate_inputs_against_manifest(
            &runbook,
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

                if !found {
                    return Err(
                        "No manifest found and no .tx runbook files found in current directory."
                            .to_string(),
                    );
                }
            }
        }
    }

    Ok(())
}

/// Analyze a single runbook file
fn analyze_runbook_file(path: &Path, format: &crate::cli::DoctorOutputFormat) -> Result<(), String> {
    analyze_runbook_file_with_context(path, None, None, &[], format)
}

/// Analyze a single runbook file with manifest context
fn analyze_runbook_file_with_context(
    path: &Path,
    manifest: Option<&WorkspaceManifest>,
    environment: Option<&String>,
    cli_inputs: &[(String, String)],
    format: &crate::cli::DoctorOutputFormat,
) -> Result<(), String> {
    // Read the runbook file
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read runbook file: {}", e))?;

    // Only print diagnostic messages for pretty format
    let is_pretty_format = matches!(format, crate::cli::DoctorOutputFormat::Pretty);
    if is_pretty_format {
        println!("Analyzing runbook: {}", path.display());
        if let Some(env) = environment {
            println!("Using environment: {}", env);
        }
        println!();
    }

    // Run the analysis
    let result = analyze_runbook_with_context(path, &content, manifest, environment, cli_inputs);

    // Display the results
    display_results(&result, format);

    // Return error if there were any errors found
    if !result.errors.is_empty() {
        Err("Doctor found errors in your runbook".to_string())
    } else {
        Ok(())
    }
}

/// Run diagnostic analysis on a txtx runbook with manifest context
pub fn analyze_runbook_with_context(
    file_path: &Path,
    content: &str,
    manifest: Option<&WorkspaceManifest>,
    environment: Option<&String>,
    cli_inputs: &[(String, String)],
) -> DoctorResult {
    let mut result =
        DoctorResult { errors: Vec::new(), warnings: Vec::new(), suggestions: Vec::new() };

    // Parse the runbook
    match parse(content) {
        Ok(runbook) => {
            // Use visitor pattern for validation and collect input refs
            let input_refs = parser_validator::validate_with_visitor(&runbook, &mut result, &file_path.to_string_lossy());

            // If we have manifest context, validate inputs with location info
            if let Some(manifest) = manifest {
                validate_inputs_against_manifest_with_locations(
                    &input_refs,
                    content,
                    manifest,
                    environment,
                    &mut result,
                    file_path,
                    cli_inputs,
                );
            }
        }
        Err(e) => {
            result.errors.push(DoctorError {
                message: format!("Failed to parse runbook: {}", e),
                file: file_path.to_string_lossy().to_string(),
                line: None,
                column: None,
                context: None,
                documentation_link: None,
            });
        }
    }

    result
}

/// Get specifications for all available addons
pub(crate) fn get_addon_specifications() -> HashMap<String, Vec<(String, CommandSpecification)>> {
    let mut specs = HashMap::new();

    // Use the same get_available_addons from main.rs
    let addons = crate::get_available_addons();

    for addon in addons {
        let namespace = addon.get_namespace();
        let actions = addon.get_actions();

        let mut action_specs = Vec::new();
        for action in actions {
            if let PreCommandSpecification::Atomic(spec) = action {
                action_specs.push((spec.matcher.clone(), spec));
            }
        }

        if !action_specs.is_empty() {
            specs.insert(namespace.to_string(), action_specs);
        }
    }

    specs
}

/// Analyze an action block
/// Get documentation link for an action
fn get_action_doc_link(action_type: &str) -> String {
    let parts: Vec<&str> = action_type.split("::").collect();
    if parts.len() == 2 {
        let addon = parts[0];
        let action = parts[1].replace("_", "-");
        format!("https://docs.txtx.sh/addons/{}/actions#{}", addon, action)
    } else {
        "https://docs.txtx.sh/addons".to_string()
    }
}

/// Helper function to find the line and column of an input reference in the source
fn find_input_location(content: &str, input_name: &str) -> Option<(usize, usize)> {
    for (line_idx, line) in content.lines().enumerate() {
        if let Some(col_idx) = line.find(input_name) {
            return Some((line_idx + 1, col_idx + 1));
        }
    }
    None
}

/// Validate input references against manifest environment with location information
fn validate_inputs_against_manifest_with_locations(
    input_refs: &[parser_validator::LocatedInputRef],
    content: &str,
    manifest: &WorkspaceManifest,
    environment: Option<&String>,
    result: &mut DoctorResult,
    file_path: &Path,
    cli_inputs: &[(String, String)],
) {
    // Determine which environment to use
    let env_name = if let Some(env) = environment {
        // Use the explicitly specified environment
        Some(env.as_str())
    } else if manifest.environments.contains_key("global") {
        // Default to global if no environment specified
        Some("global")
    } else {
        // Fall back to first available environment
        manifest.environments.keys().next().map(|s| s.as_str())
    };

    // Build the effective environment by merging global with specific environment
    let mut effective_inputs = HashMap::new();
    
    // First, add all inputs from global environment
    if let Some(global_inputs) = manifest.environments.get("global") {
        for (key, value) in global_inputs {
            effective_inputs.insert(key.clone(), value.clone());
        }
    }
    
    // Then, overlay the specific environment (if different from global)
    if let Some(env_name) = env_name {
        if env_name != "global" {
            if let Some(env_inputs) = manifest.environments.get(env_name) {
                for (key, value) in env_inputs {
                    effective_inputs.insert(key.clone(), value.clone());
                }
            }
        }
    }

    // Apply CLI input overrides
    for (key, value) in cli_inputs {
        effective_inputs.insert(key.clone(), value.clone());
    }

    // Check each input reference
    for input_ref in input_refs {
        let input_name = if input_ref.name.starts_with("input.") {
            &input_ref.name[6..]
        } else {
            &input_ref.name
        };
        
        if !effective_inputs.contains_key(input_name) {
            // Find the actual location in the source file
            let (line, column) = if let Some((l, c)) = find_input_location(content, &input_ref.name) {
                (Some(l), Some(c))
            } else {
                (None, None)
            };
            
            let mut context_msg = format!(
                "Add '{}' to your txtx.yml file",
                input_name
            );
            
            if env_name.is_some() && env_name != Some("global") {
                context_msg.push_str(" (consider adding to 'global' if used across environments)");
            }
            
            result.errors.push(DoctorError {
                message: format!(
                    "Input '{}' is not defined in environment '{}' (including inherited values)",
                    input_ref.name,
                    env_name.unwrap_or("default")
                ),
                file: file_path.to_string_lossy().to_string(),
                line,
                column,
                context: Some(context_msg),
                documentation_link: Some(
                    "https://docs.txtx.sh/concepts/manifest#environments".to_string(),
                ),
            });

            // Add a suggestion with example
            result.suggestions.push(DoctorSuggestion {
                message: "Add the missing input to your environment".to_string(),
                example: Some(format!(
                    "environments:\n  {}:\n    {}: \"<value>\"{}",
                    env_name.unwrap_or("default"),
                    input_name,
                    if env_name != Some("global") { 
                        format!("\n\n# Or add to global for all environments:\nenvironments:\n  global:\n    {}: \"<value>\"", input_name) 
                    } else { 
                        "".to_string() 
                    }
                )),
            });
        }
    }
}

/// Keep the old function for backward compatibility
fn validate_inputs_against_manifest(
    runbook: &Runbook,
    manifest: &WorkspaceManifest,
    environment: Option<&String>,
    result: &mut DoctorResult,
    file_path: &Path,
    cli_inputs: &[(String, String)],
) {
    // Determine which environment to use
    let env_name = if let Some(env) = environment {
        // Use the explicitly specified environment
        Some(env.as_str())
    } else if manifest.environments.contains_key("global") {
        // Default to global if no environment specified
        Some("global")
    } else {
        // Fall back to first available environment
        manifest.environments.keys().next().map(|s| s.as_str())
    };

    // Build the effective environment by merging global with specific environment
    let mut effective_inputs = HashMap::new();
    
    // First, add all inputs from global environment
    if let Some(global_inputs) = manifest.environments.get("global") {
        for (key, value) in global_inputs {
            effective_inputs.insert(key.clone(), value.clone());
        }
    }
    
    // Then, overlay the specific environment (if different from global)
    if let Some(env_name) = env_name {
        if env_name != "global" {
            if let Some(env_inputs) = manifest.environments.get(env_name) {
                for (key, value) in env_inputs {
                    effective_inputs.insert(key.clone(), value.clone());
                }
            } else {
                // Environment specified but not found
                result.errors.push(DoctorError {
                    message: format!("Environment '{}' is not defined in the manifest", env_name),
                    file: file_path.to_string_lossy().to_string(),
                    line: None,
                    column: None,
                    context: Some(format!(
                        "Available environments: {}",
                        manifest.environments.keys().cloned().collect::<Vec<_>>().join(", ")
                    )),
                    documentation_link: Some(
                        "https://docs.txtx.sh/concepts/manifest#environments".to_string(),
                    ),
                });
            }
        }
    }

    // Apply CLI input overrides
    for (key, value) in cli_inputs {
        effective_inputs.insert(key.clone(), value.clone());
    }

    // Add info about environment inheritance
    if env_name.is_some() && env_name != Some("global") && manifest.environments.contains_key("global") {
        result.suggestions.push(DoctorSuggestion {
            message: format!(
                "Environment '{}' inherits from 'global'. Values in '{}' override those in 'global'.",
                env_name.unwrap(),
                env_name.unwrap()
            ),
            example: None,
        });
    }
    
    // Add info about CLI inputs if any were provided
    if !cli_inputs.is_empty() {
        result.suggestions.push(DoctorSuggestion {
            message: format!(
                "CLI inputs take precedence over environment values. {} CLI input{} provided.",
                cli_inputs.len(),
                if cli_inputs.len() > 1 { "s" } else { "" }
            ),
            example: None,
        });
    }

    // Collect all input references from the runbook
    let mut input_refs = std::collections::HashSet::new();
    collect_input_references(runbook, &mut input_refs);

    // Check each input reference
    for input_ref in &input_refs {
        let input_name = &input_ref[6..]; // Skip "input." prefix

        if !effective_inputs.is_empty() {
            if !effective_inputs.contains_key(input_name) {
                // Input not found in effective environment
                let mut context_msg = format!(
                    "Add '{}' to your txtx.yml file",
                    input_name
                );
                
                // Provide specific guidance on where to add it
                if manifest.environments.contains_key("global") {
                    context_msg.push_str(" (consider adding to 'global' if used across environments)");
                }
                
                result.errors.push(DoctorError {
                    message: format!(
                        "Input '{}' is not defined in environment '{}' (including inherited values)",
                        input_ref,
                        env_name.unwrap_or("default")
                    ),
                    file: file_path.to_string_lossy().to_string(),
                    line: None,
                    column: None,
                    context: Some(context_msg),
                    documentation_link: Some(
                        "https://docs.txtx.sh/concepts/manifest#environments".to_string(),
                    ),
                });

                // Add a suggestion with example
                result.suggestions.push(DoctorSuggestion {
                    message: "Add the missing input to your environment".to_string(),
                    example: Some(format!(
                        "environments:\n  {}:\n    {}: \"<value>\"\n\n# Or add to global for all environments:\nenvironments:\n  global:\n    {}: \"<value>\"",
                        env_name.unwrap_or("global"),
                        input_name,
                        input_name
                    )),
                });
            }
        } else if !input_refs.is_empty() {
            // No environment defined but inputs are used
            result.warnings.push(DoctorWarning {
                message: "Runbook uses inputs but no environment is defined in the manifest"
                    .to_string(),
                file: file_path.to_string_lossy().to_string(),
                line: None,
                column: None,
                suggestion: Some("Add an 'environments' section to your txtx.yml file".to_string()),
            });
            break; // Only warn once
        }
    }

    // Check for unused environment variables
    if !effective_inputs.is_empty() {
        for (input_name, _) in &effective_inputs {
            let input_ref = format!("input.{}", input_name);
            if !input_refs.contains(&input_ref) {
                // Determine if it's from global or specific environment
                let source = if manifest.environments.get("global").map_or(false, |g| g.contains_key(input_name)) 
                    && env_name != Some("global") {
                    "inherited from global"
                } else {
                    "defined"
                };
                
                result.warnings.push(DoctorWarning {
                    message: format!(
                        "Environment variable '{}' is {} but not used in this runbook",
                        input_name,
                        source
                    ),
                    file: file_path.to_string_lossy().to_string(),
                    line: None,
                    column: None,
                    suggestion: Some(format!(
                        "Consider removing '{}' if it's not needed",
                        input_name
                    )),
                });
            }
        }
    }
    
    // Add note about CLI precedence
    if !input_refs.is_empty() {
        result.suggestions.push(DoctorSuggestion {
            message: "Note: Values passed via --input on CLI take precedence over environment values".to_string(),
            example: Some("txtx run myrunbook --input MY_VAR=override_value".to_string()),
        });
    }
}

/// Collect all input references from a runbook
fn collect_input_references(runbook: &Runbook, refs: &mut std::collections::HashSet<String>) {
    // Check addon blocks
    for addon in &runbook.addons {
        for (_, expr) in &addon.attributes {
            collect_input_refs_from_expression(expr, refs);
        }
    }

    // Check signer blocks
    for signer in &runbook.signers {
        for (_, expr) in &signer.attributes {
            collect_input_refs_from_expression(expr, refs);
        }
    }

    // Check action blocks
    for action in &runbook.actions {
        for (_, expr) in &action.attributes {
            collect_input_refs_from_expression(expr, refs);
        }
    }

    // Check variable declarations
    for var in &runbook.variables {
        for (_, expr) in &var.attributes {
            collect_input_refs_from_expression(expr, refs);
        }
    }
}

/// Collect input references from an expression
fn collect_input_refs_from_expression(
    expr: &Expression,
    refs: &mut std::collections::HashSet<String>,
) {
    match expr {
        Expression::Reference(parts) => {
            if parts.len() >= 2 && parts[0] == "input" {
                refs.insert(parts.join("."));
            }
        }
        Expression::Object(fields) => {
            for (_, value) in fields {
                collect_input_refs_from_expression(value, refs);
            }
        }
        Expression::Array(items) => {
            for item in items {
                collect_input_refs_from_expression(item, refs);
            }
        }
        Expression::FunctionCall { args, .. } => {
            for arg in args {
                collect_input_refs_from_expression(arg, refs);
            }
        }
        _ => {} // Other expression types don't contain references
    }
}

/// Format and display the doctor results
pub fn display_results(result: &DoctorResult, format: &crate::cli::DoctorOutputFormat) {
    match format {
        crate::cli::DoctorOutputFormat::Quickfix => display_results_quickfix(result),
        crate::cli::DoctorOutputFormat::Json => display_results_json(result),
        crate::cli::DoctorOutputFormat::Pretty => display_results_pretty(result),
        crate::cli::DoctorOutputFormat::Auto => display_results_pretty(result), // Should not reach here after auto-detection
    }
}

/// Display results in quickfix format (single line per issue)
fn display_results_quickfix(result: &DoctorResult) {
    // Errors in quickfix format
    for error in &result.errors {
        let location = if let (Some(line), Some(column)) = (error.line, error.column) {
            format!("{}:{}:{}: ", error.file, line + 1, column + 1)
        } else {
            // When we don't have specific location, default to line 1 for navigation
            format!("{}:1: ", error.file)
        };
        
        let mut message = format!("error: {}", error.message);
        if let Some(link) = &error.documentation_link {
            message.push_str(&format!(" (see: {})", link));
        }
        
        println!("{}{}", location, message);
    }

    // Warnings in quickfix format
    for warning in &result.warnings {
        let location = if let (Some(line), Some(column)) = (warning.line, warning.column) {
            format!("{}:{}:{}: ", warning.file, line + 1, column + 1)
        } else {
            // When we don't have specific location, default to line 1 for navigation
            format!("{}:1: ", warning.file)
        };
        
        let mut message = format!("warning: {}", warning.message);
        if let Some(suggestion) = &warning.suggestion {
            message.push_str(&format!(" (hint: {})", suggestion));
        }
        
        println!("{}{}", location, message);
    }
}

/// Display results in JSON format
fn display_results_json(result: &DoctorResult) {
    use serde_json::json;
    
    let output = json!({
        "errors": result.errors.iter().map(|e| {
            json!({
                "file": e.file,
                "line": e.line,
                "column": e.column,
                "level": "error",
                "message": e.message,
                "context": e.context,
                "documentation": e.documentation_link,
            })
        }).collect::<Vec<_>>(),
        "warnings": result.warnings.iter().map(|w| {
            json!({
                "file": w.file,
                "line": w.line,
                "column": w.column,
                "level": "warning",
                "message": w.message,
                "suggestion": w.suggestion,
            })
        }).collect::<Vec<_>>(),
        "suggestions": result.suggestions.iter().map(|s| {
            json!({
                "message": s.message,
                "example": s.example,
            })
        }).collect::<Vec<_>>(),
    });
    
    println!("{}", serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string()));
}

/// Display results in pretty format (current implementation)
fn display_results_pretty(result: &DoctorResult) {
    use ansi_term::Colour::{Blue, Red, Yellow};

    let total_issues = result.errors.len() + result.warnings.len();

    if total_issues == 0 {
        println!("{} No issues found!", Blue.paint("✓"));
        return;
    }

    println!("{}", Red.bold().paint(format!("Found {} issue(s):", total_issues)));
    println!();

    // Display errors
    for (i, error) in result.errors.iter().enumerate() {
        // Format: file:line:column: error message
        // This format is recognized by most IDEs and can be clicked
        let location = if let (Some(line), Some(column)) = (error.line, error.column) {
            format!("{}:{}:{}: ", error.file, line + 1, column + 1)
        } else {
            format!("{}: ", error.file)
        };
        
        println!("{}{} {}", 
            location,
            Red.bold().paint(format!("error[{}]:", i + 1)), 
            Red.paint(&error.message)
        );

        if let Some(context) = &error.context {
            println!("   {}", context);
        }

        if let Some(link) = &error.documentation_link {
            println!("   {} {}", Blue.paint("Documentation:"), link);
        }

        println!();
    }

    // Display warnings
    for warning in &result.warnings {
        // Format: file:line:column: warning message
        let location = if let (Some(line), Some(column)) = (warning.line, warning.column) {
            format!("{}:{}:{}: ", warning.file, line + 1, column + 1)
        } else {
            format!("{}: ", warning.file)
        };
        
        println!("{}{} {}", 
            location,
            Yellow.paint("warning:"), 
            warning.message
        );
        
        if let Some(suggestion) = &warning.suggestion {
            println!("   {} {}", Blue.paint("Suggestion:"), suggestion);
        }
        println!();
    }

    // Display suggestions
    if !result.suggestions.is_empty() {
        println!("{}", Blue.bold().paint("Suggestions:"));
        for suggestion in &result.suggestions {
            println!("  • {}", suggestion.message);
            if let Some(example) = &suggestion.example {
                println!("    {}", example);
            }
        }
    }
}
