//! Enhanced diagnostics module that integrates doctor validation rules
//!
//! This module extends the basic HCL validation with doctor's semantic validation rules
//! like InputDefinedRule, SensitiveDataRule, etc.

use crate::cli::common::addon_registry;
use crate::cli::doctor::{
    CliInputOverrideRule, InputDefinedRule, InputNamingConventionRule, SensitiveDataRule,
    ValidationContext, ValidationRule,
};
use crate::cli::lsp::validation::validation_outcome_to_diagnostic;
use crate::cli::lsp::workspace::{
    manifest_converter::lsp_manifest_to_workspace_manifest, Manifest,
};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use std::collections::HashMap;
use txtx_core::validation::ValidationResult;

/// Validate a runbook file with both HCL and doctor validation rules
pub fn validate_runbook_with_doctor_rules(
    file_uri: &Url,
    content: &str,
    lsp_manifest: Option<&Manifest>,
    environment: Option<&str>,
    cli_inputs: &[(String, String)],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // First, run HCL validation to get syntax errors and collect input references
    let mut validation_result = ValidationResult::new();
    let file_path = file_uri.path();

    // Load addon specifications
    let addons = addon_registry::get_all_addons();
    let addon_specs = addon_registry::extract_addon_specifications(&addons);

    // Run HCL validation and collect input references
    let input_refs = match txtx_core::validation::hcl_validator::validate_with_hcl_and_addons(
        content,
        &mut validation_result,
        file_path,
        addon_specs,
    ) {
        Ok(refs) => refs,
        Err(_) => Vec::new(),
    };

    // Convert HCL validation errors to diagnostics
    for error in &validation_result.errors {
        let range = Range {
            start: Position {
                line: error.line.unwrap_or(1).saturating_sub(1) as u32,
                character: error.column.unwrap_or(1).saturating_sub(1) as u32,
            },
            end: Position {
                line: error.line.unwrap_or(1).saturating_sub(1) as u32,
                character: (error.column.unwrap_or(1) + 20) as u32,
            },
        };

        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("txtx".to_string()),
            message: error.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });
    }

    // Convert warnings
    for warning in &validation_result.warnings {
        let range = Range {
            start: Position {
                line: warning.line.unwrap_or(1).saturating_sub(1) as u32,
                character: warning.column.unwrap_or(1).saturating_sub(1) as u32,
            },
            end: Position {
                line: warning.line.unwrap_or(1).saturating_sub(1) as u32,
                character: (warning.column.unwrap_or(1) + 20) as u32,
            },
        };

        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::WARNING),
            code: None,
            code_description: None,
            source: Some("txtx".to_string()),
            message: warning.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });
    }

    // Now run doctor validation rules on the collected input references
    if let Some(manifest) = lsp_manifest {
        let workspace_manifest = lsp_manifest_to_workspace_manifest(manifest);

        // Get effective inputs for the environment
        let effective_inputs = get_effective_inputs(&workspace_manifest, environment);

        // Create validation rules
        let rules: Vec<Box<dyn ValidationRule>> = vec![
            Box::new(InputDefinedRule),
            Box::new(InputNamingConventionRule),
            Box::new(CliInputOverrideRule),
            Box::new(SensitiveDataRule),
        ];

        // Run validation rules for each input reference
        for input_ref in &input_refs {
            // Extract just the input name (remove "input." prefix)
            let input_name = input_ref.name.strip_prefix("input.").unwrap_or(&input_ref.name);

            // Create validation context
            let ctx = ValidationContext {
                input_name,
                full_name: &input_ref.name,
                manifest: &workspace_manifest,
                environment,
                effective_inputs: &effective_inputs,
                cli_inputs,
                content,
                file_path,
            };

            // Run each rule
            for rule in &rules {
                let outcome = rule.check(&ctx);

                // Convert outcome to diagnostic at the input reference location
                let range = Range {
                    start: Position {
                        line: (input_ref.line.saturating_sub(1)) as u32,
                        character: input_ref.column as u32,
                    },
                    end: Position {
                        line: (input_ref.line.saturating_sub(1)) as u32,
                        character: (input_ref.column + input_ref.name.len()) as u32,
                    },
                };

                if let Some(diagnostic) = validation_outcome_to_diagnostic(outcome, range) {
                    diagnostics.push(diagnostic);
                }
            }
        }
    }

    diagnostics
}

/// Get effective inputs for an environment, including global inputs
fn get_effective_inputs(
    manifest: &txtx_core::manifest::WorkspaceManifest,
    environment: Option<&str>,
) -> HashMap<String, String> {
    let mut effective_inputs = HashMap::new();

    // First, add global inputs
    if let Some(global_env) = manifest.environments.get("global") {
        for (key, value) in global_env {
            effective_inputs.insert(key.clone(), value.clone());
        }
    }

    // Then, override with environment-specific inputs
    if let Some(env_name) = environment {
        if let Some(env) = manifest.environments.get(env_name) {
            for (key, value) in env {
                effective_inputs.insert(key.clone(), value.clone());
            }
        }
    }

    effective_inputs
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::Url;

    #[test]
    fn test_validate_missing_input() {
        let content = r#"
            addon "evm" {
                network_id = 1
            }
            
            action "deploy" "evm::deploy_contract" {
                private_key = input.deployer_key
            }
        "#;

        let uri = Url::parse("file:///test.tx").unwrap();

        // Create a manifest without the required input
        let manifest = Manifest {
            uri: Url::parse("file:///txtx.yml").unwrap(),
            runbooks: vec![],
            environments: HashMap::new(),
        };

        let diagnostics = validate_runbook_with_doctor_rules(
            &uri,
            content,
            Some(&manifest),
            Some("production"),
            &[],
        );

        // Should have at least one error about missing input
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("deployer_key") && d.message.contains("not defined")));
    }

    #[test]
    fn test_validate_sensitive_input() {
        let content = r#"
            action "deploy" "evm::deploy_contract" {
                private_key = input.wallet_private_key
            }
        "#;

        let uri = Url::parse("file:///test.tx").unwrap();

        // Create a manifest with the sensitive input
        let mut environments = HashMap::new();
        let mut prod_env = HashMap::new();
        prod_env.insert("wallet_private_key".to_string(), "0x123".to_string());
        environments.insert("production".to_string(), prod_env);

        let manifest = Manifest {
            uri: Url::parse("file:///txtx.yml").unwrap(),
            runbooks: vec![],
            environments,
        };

        let diagnostics = validate_runbook_with_doctor_rules(
            &uri,
            content,
            Some(&manifest),
            Some("production"),
            &[],
        );

        // Should have a warning about sensitive data
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("sensitive")
                && d.severity == Some(DiagnosticSeverity::WARNING)));
    }
}
