use std::path::Path;
use txtx_core::{
    manifest::WorkspaceManifest,
    validation::{ValidationResult, LocatedInputRef, ManifestValidationConfig, get_doctor_rules, get_strict_doctor_rules},
};

/// Validate input references against manifest environment
/// 
/// This function now delegates to the core validation module with doctor-specific rules
pub fn validate_inputs_against_manifest(
    input_refs: &[LocatedInputRef],
    content: &str,
    manifest: &WorkspaceManifest,
    environment: Option<&String>,
    result: &mut ValidationResult,
    file_path: &Path,
    cli_inputs: &[(String, String)],
) {
    // Create configuration with doctor rules based on environment
    let mut config = if environment == Some(&"production".to_string()) || environment == Some(&"prod".to_string()) {
        let mut cfg = ManifestValidationConfig::strict();
        // Add doctor-specific rules for production
        cfg.custom_rules.extend(get_strict_doctor_rules());
        cfg
    } else {
        let mut cfg = ManifestValidationConfig::default();
        // Add standard doctor rules
        cfg.custom_rules.extend(get_doctor_rules());
        cfg
    };
    
    // Delegate to core validation
    txtx_core::validation::validate_inputs_against_manifest(
        input_refs,
        content,
        manifest,
        environment,
        result,
        &file_path.to_string_lossy(),
        cli_inputs,
        config,
    );
}