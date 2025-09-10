use std::collections::HashMap;
use std::path::{Path, PathBuf};

use txtx_core::kit::types::commands::{CommandSpecification, PreCommandSpecification};
use txtx_core::manifest::WorkspaceManifest;
use txtx_parser::{parse, Runbook, Expression};

pub(crate) mod parser_validator;

#[derive(Debug)]
pub struct DoctorResult {
    pub errors: Vec<DoctorError>,
    pub warnings: Vec<DoctorWarning>,
    pub suggestions: Vec<DoctorSuggestion>,
}

#[derive(Debug)]
pub struct DoctorError {
    pub message: String,
    pub _file: String,
    pub _line: Option<usize>,
    pub _column: Option<usize>,
    pub context: Option<String>,
    pub documentation_link: Option<String>,
}

#[derive(Debug)]
pub struct DoctorWarning {
    pub message: String,
    pub _file: String,
    pub _line: Option<usize>,
    pub _column: Option<usize>,
    pub suggestion: Option<String>,
}

#[derive(Debug)]
pub struct DoctorSuggestion {
    pub message: String,
    pub example: Option<String>,
}

/// Main entry point for the doctor command
pub async fn run_doctor(
    manifest_path: Option<String>,
    runbook_name: Option<String>,
    environment: Option<String>,
) -> Result<(), String> {
    let manifest_path = manifest_path.unwrap_or_else(|| "./txtx.yml".to_string());

    // If a specific runbook is specified, try to find it
    if let Some(runbook_name) = runbook_name {
        // First try as a direct file path
        let path = PathBuf::from(&runbook_name);
        if path.exists() && path.extension().map_or(false, |ext| ext == "tx") {
            analyze_runbook_file(&path)?;
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
                println!("No runbooks found in manifest.");
            } else {
                let mut any_errors = false;
                for metadata in &manifest.runbooks {
                    println!("Checking runbook '{}'...", metadata.name);

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
                    println!();
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
                    analyze_runbook_file(&path)?;
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
                            analyze_runbook_file(&path)?;
                            found = true;
                            break;
        }
    }
}

/// Test helper to validate a runbook string and return the result
#[cfg(test)]
pub fn validate_runbook_content(content: &str) -> DoctorResult {
    let runbook = parse(content).expect("Failed to parse test runbook");
    let mut result = DoctorResult {
        errors: Vec::new(),
        warnings: Vec::new(),
        suggestions: Vec::new(),
    };
    parser_validator::validate_with_visitor(&runbook, &mut result, "test.tx");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use the shared test helper
    fn validate_fixture(content: &str) -> DoctorResult {
        validate_runbook_content(content)
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
fn analyze_runbook_file(path: &Path) -> Result<(), String> {
    analyze_runbook_file_with_context(path, None, None)
}

/// Analyze a single runbook file with manifest context
fn analyze_runbook_file_with_context(
    path: &Path,
    manifest: Option<&WorkspaceManifest>,
    environment: Option<&String>,
) -> Result<(), String> {
    // Read the runbook file
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read runbook file: {}", e))?;

    println!("Analyzing runbook: {}", path.display());
    if let Some(env) = environment {
        println!("Using environment: {}", env);
    }
    println!();

    // Run the analysis
    let result = analyze_runbook_with_context(path, &content, manifest, environment);

    // Display the results
    display_results(&result);

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
) -> DoctorResult {
    let mut result =
        DoctorResult { errors: Vec::new(), warnings: Vec::new(), suggestions: Vec::new() };

    // Parse the runbook
    match parse(content) {
        Ok(runbook) => {
            // Use visitor pattern for validation
            parser_validator::validate_with_visitor(&runbook, &mut result, &file_path.to_string_lossy());

            // If we have manifest context, validate inputs
            if let Some(manifest) = manifest {
                validate_inputs_against_manifest(
                    &runbook,
                    manifest,
                    environment,
                    &mut result,
                    file_path,
                );
            }
        }
        Err(e) => {
            result.errors.push(DoctorError {
                message: format!("Failed to parse runbook: {}", e),
                _file: file_path.to_string_lossy().to_string(),
                _line: None,
                _column: None,
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

/// Validate input references against manifest environment
fn validate_inputs_against_manifest(
    runbook: &Runbook,
    manifest: &WorkspaceManifest,
    environment: Option<&String>,
    result: &mut DoctorResult,
    file_path: &Path,
) {
    // Get the environment to use
    let env_name = environment.or_else(|| manifest.environments.keys().next()).map(|s| s.as_str());

    // Get available inputs from the environment
    let available_inputs =
        if let Some(env_name) = env_name { manifest.environments.get(env_name) } else { None };

    // Collect all input references from the runbook
    let mut input_refs = std::collections::HashSet::new();
    collect_input_references(runbook, &mut input_refs);

    // Check each input reference
    for input_ref in &input_refs {
        let input_name = &input_ref[6..]; // Skip "input." prefix

        if let Some(inputs) = available_inputs {
            if !inputs.contains_key(input_name) {
                // Input not found in environment
                result.errors.push(DoctorError {
                    message: format!(
                        "Input '{}' is not defined in environment '{}'",
                        input_ref,
                        env_name.unwrap_or("default")
                    ),
                    _file: file_path.to_string_lossy().to_string(),
                    _line: None,
                    _column: None,
                    context: Some(format!(
                        "Add '{}' to the '{}' environment in your txtx.yml file",
                        input_name,
                        env_name.unwrap_or("default")
                    )),
                    documentation_link: Some(
                        "https://docs.txtx.sh/concepts/manifest#environments".to_string(),
                    ),
                });

                // Add a suggestion with example
                result.suggestions.push(DoctorSuggestion {
                    message: "Add the missing input to your txtx.yml environment section"
                        .to_string(),
                    example: Some(format!(
                        "environments:\n  {}:\n    {}: \"<value>\"",
                        env_name.unwrap_or("default"),
                        input_name
                    )),
                });
            }
        } else if !input_refs.is_empty() {
            // No environment defined but inputs are used
            result.warnings.push(DoctorWarning {
                message: "Runbook uses inputs but no environment is defined in the manifest"
                    .to_string(),
                _file: file_path.to_string_lossy().to_string(),
                _line: None,
                _column: None,
                suggestion: Some("Add an 'environments' section to your txtx.yml file".to_string()),
            });
            break; // Only warn once
        }
    }

    // Also check for unused environment variables
    if let Some(inputs) = available_inputs {
        for (input_name, _) in inputs {
            let input_ref = format!("input.{}", input_name);
            if !input_refs.contains(&input_ref) {
                result.warnings.push(DoctorWarning {
                    message: format!(
                        "Environment variable '{}' is defined but not used in this runbook",
                        input_name
                    ),
                    _file: file_path.to_string_lossy().to_string(),
                    _line: None,
                    _column: None,
                    suggestion: Some(format!(
                        "Consider removing '{}' from the '{}' environment if it's not needed",
                        input_name,
                        env_name.unwrap_or("default")
                    )),
                });
            }
        }
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
pub fn display_results(result: &DoctorResult) {
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
        println!("{} {}", Red.bold().paint(format!("{}.", i + 1)), Red.paint(&error.message));

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
        println!("{} {}", Yellow.paint("Warning:"), warning.message);
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
