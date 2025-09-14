use std::path::Path;
use txtx_core::{
    manifest::WorkspaceManifest,
    validation::{ValidationResult, LocatedInputRef},
};
use super::validator::InputValidator;

/// Validate input references against manifest environment using data-driven approach
pub fn validate_inputs_against_manifest(
    input_refs: &[LocatedInputRef],
    content: &str,
    manifest: &WorkspaceManifest,
    environment: Option<&String>,
    result: &mut ValidationResult,
    file_path: &Path,
    cli_inputs: &[(String, String)],
) {
    // Create validator with appropriate rules based on environment
    let validator = if environment == Some(&"production".to_string()) || environment == Some(&"prod".to_string()) {
        InputValidator::strict()
    } else {
        InputValidator::new()
    };
    
    // Use data-driven validation
    validator.validate_inputs(
        input_refs,
        content,
        manifest,
        environment,
        result,
        file_path,
        cli_inputs,
    );
}