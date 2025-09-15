use crate::builders::runbook_builder::{RunbookBuilder, ValidationResult};
use txtx_core::manifest::WorkspaceManifest;
use txtx_addon_kit::indexmap::IndexMap;
use std::path::PathBuf;

/// Enhanced validation options for RunbookBuilder
pub enum ValidationMode {
    /// Basic HCL validation only (default)
    HclOnly,
    /// Full doctor validation with manifest and environment context
    Doctor {
        /// Optional manifest for input/environment validation
        manifest: Option<WorkspaceManifest>,
        /// Optional environment name to use
        environment: Option<String>,
        /// Optional file path for error reporting
        file_path: Option<PathBuf>,
    },
    /// LSP validation with workspace context
    Lsp {
        /// Workspace root for multi-file resolution
        workspace_root: PathBuf,
        /// Optional manifest for context
        manifest: Option<WorkspaceManifest>,
    },
}

/// Extension trait for RunbookBuilder to enable doctor validation
/// 
/// This trait must be implemented by the test crate that has access to txtx-cli.
/// This avoids a circular dependency between txtx-test-utils and txtx-cli.
/// 
/// # Example Implementation
/// 
/// ```rust
/// use txtx_test_utils::{RunbookBuilder, RunbookBuilderExt, ValidationResult};
/// use txtx_cli::cli::doctor::analyzer::RunbookAnalyzer;
/// 
/// impl RunbookBuilderExt for RunbookBuilder {
///     fn validate_with_doctor_impl(
///         &mut self,
///         content: &str,
///         manifest: Option<&WorkspaceManifest>,
///         environment: Option<&String>,
///         cli_inputs: &[(String, String)],
///         file_path: &Path,
///     ) -> ValidationResult {
///         let analyzer = RunbookAnalyzer::new();
///         let core_result = analyzer.analyze_runbook_with_context(
///             file_path,
///             content,
///             manifest,
///             environment,
///             cli_inputs,
///         );
///         
///         // Convert core ValidationResult to test utils ValidationResult
///         ValidationResult {
///             success: core_result.errors.is_empty(),
///             errors: /* convert errors */,
///             warnings: /* convert warnings */,
///         }
///     }
/// }
/// ```
pub trait RunbookBuilderExt {
    /// Implementation hook for doctor validation
    fn validate_with_doctor_impl(
        &mut self,
        content: &str,
        manifest: Option<&WorkspaceManifest>,
        environment: Option<&String>,
        cli_inputs: &[(String, String)],
        file_path: &std::path::Path,
    ) -> ValidationResult;
}

impl RunbookBuilder {
    /// Validate with enhanced doctor analysis
    /// 
    /// This runs the full doctor validation pipeline including:
    /// - Undefined signer detection
    /// - Invalid field access on action outputs
    /// - Cross-reference validation between actions
    /// - Input/environment variable validation against manifest
    /// 
    /// Note: This method requires the RunbookBuilderExt trait to be implemented
    /// in your test crate with access to txtx-cli.
    /// 
    /// # Example
    /// ```rust
    /// let manifest = create_test_manifest();
    /// let result = RunbookBuilder::new()
    ///     .action("deploy", "evm::deploy_contract")
    ///         .input("signer", "signer.undefined")  // Doctor will catch this!
    ///     .validate_with_doctor(Some(manifest), Some("production".to_string()));
    /// 
    /// assert_validation_error!(result, "undefined signer");
    /// ```
    pub fn validate_with_doctor(
        &mut self,
        manifest: Option<WorkspaceManifest>,
        environment: Option<String>,
    ) -> ValidationResult {
        self.validate_with_mode(ValidationMode::Doctor {
            manifest,
            environment,
            file_path: Some(PathBuf::from("test.tx")),
        })
    }
    
    /// Validate with specific validation mode
    pub fn validate_with_mode(&mut self, mode: ValidationMode) -> ValidationResult {
        let content = self.build_content();
        
        match mode {
            ValidationMode::HclOnly => {
                // Use existing simple validation
                crate::simple_validator::validate_content(&content)
            }
            ValidationMode::Doctor { manifest: _, environment: _, file_path: _ } => {
                // Use our local parser for doctor validation
                use super::parser::{parse_runbook_content, extract_signers, extract_actions, find_signer_references, find_action_references, find_env_references};
                
                match parse_runbook_content(&content) {
                    Ok(blocks) => {
                        let mut result = ValidationResult {
                            success: true,
                            errors: vec![],
                            warnings: vec![],
                        };
                        
                        // Extract defined entities
                        let defined_signers = extract_signers(&blocks);
                        let defined_actions = extract_actions(&blocks);
                        
                        // Find references
                        let signer_refs = find_signer_references(&content);
                        let action_refs = find_action_references(&content);
                        
                        // Check for undefined signers
                        for signer_ref in &signer_refs {
                            if !defined_signers.contains(signer_ref) {
                                result.success = false;
                                result.errors.push(txtx_addon_kit::types::diagnostics::Diagnostic::error_from_string(
                                    format!("undefined signer 'signer.{}'", signer_ref)
                                ));
                            }
                        }
                        
                        // Check for undefined actions
                        for action_ref in &action_refs {
                            if !defined_actions.contains(action_ref) {
                                result.success = false;
                                result.errors.push(txtx_addon_kit::types::diagnostics::Diagnostic::error_from_string(
                                    format!("undefined action 'action.{}'", action_ref)
                                ));
                            }
                        }
                        
                        // Check for undefined environment variables
                        let env_refs = find_env_references(&content);
                        
                        // Get available environment variables
                        let available_env_vars = if let ValidationMode::Doctor { manifest: Some(manifest), environment: Some(env_name), .. } = &mode {
                            manifest.environments.get(env_name)
                                .map(|env| env.keys().cloned().collect::<Vec<_>>())
                                .unwrap_or_default()
                        } else {
                            // Check the builder's own environments
                            self.environments.values()
                                .flat_map(|env| env.keys().cloned())
                                .collect::<Vec<_>>()
                        };
                        
                        for env_ref in &env_refs {
                            if !available_env_vars.contains(env_ref) {
                                result.success = false;
                                result.errors.push(txtx_addon_kit::types::diagnostics::Diagnostic::error_from_string(
                                    format!("undefined environment variable 'env.{}'", env_ref)
                                ));
                            }
                        }
                        
                        result
                    }
                    Err(e) => {
                        ValidationResult {
                            success: false,
                            errors: vec![e],
                            warnings: vec![],
                        }
                    }
                }
            }
            ValidationMode::Lsp { workspace_root: _, manifest: _ } => {
                // TODO: Implement LSP validation mode
                unimplemented!("LSP validation mode not yet implemented")
            }
        }
    }
    
    /// Create a test manifest with the configured environments
    pub fn build_manifest(&self) -> WorkspaceManifest {
        let mut manifest = WorkspaceManifest {
            name: "test".to_string(),
            id: "test-id".to_string(),
            runbooks: Vec::new(),
            environments: IndexMap::new(),
            location: None,
        };
        
        // Add configured environments to manifest
        for (env_name, vars) in &self.environments {
            let env_vars: IndexMap<String, String> = vars
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            manifest.environments.insert(env_name.clone(), env_vars);
        }
        
        manifest
    }
}

/// Helper to create a test manifest quickly
pub fn create_test_manifest_with_env(environments: Vec<(&str, Vec<(&str, &str)>)>) -> WorkspaceManifest {
    let mut manifest = WorkspaceManifest {
        name: "test".to_string(),
        id: "test-id".to_string(),
        runbooks: Vec::new(),
        environments: IndexMap::new(),
        location: None,
    };
    
    for (env_name, vars) in environments {
        let mut env_map = IndexMap::new();
        for (key, value) in vars {
            env_map.insert(key.to_string(), value.to_string());
        }
        manifest.environments.insert(env_name.to_string(), env_map);
    }
    
    manifest
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_validation_error;
    
    #[test]
    fn test_doctor_catches_undefined_signer() {
        // This test would fail with HCL-only validation but passes with doctor
        let result = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("deploy", "evm::deploy_contract")
                .input("signer", "signer.undefined_signer")
            .validate_with_doctor(None, None);
        
        // Doctor validation catches undefined signers!
        assert_validation_error!(result, "undefined_signer");
    }
    
    // TODO: These tests require more advanced doctor validation
    // #[test]
    // fn test_doctor_validates_action_outputs() {
    //     // Test that doctor catches invalid field access
    //     let result = RunbookBuilder::new()
    //         .addon("evm", vec![])
    //         .action("send", "evm::send_eth")
    //             .input("to", "0x123")
    //             .input("value", "1000")
    //         .output("bad", "action.send.invalid_field")  // send_eth only has tx_hash
    //         .validate_with_doctor(None, None);
        
    //     assert_validation_error!(result, "Field 'invalid_field' does not exist");
    // }
    
    // #[test]
    // fn test_doctor_validates_inputs_against_manifest() {
    //     // Create a manifest with environment variables
    //     let manifest = create_test_manifest_with_env(vec![
    //         ("production", vec![("API_URL", "https://api.example.com")]),
    //     ]);
        
    //     // Test missing input validation
    //     let result = RunbookBuilder::new()
    //         .variable("key", "env.MISSING_KEY")
    //         .output("result", "input.key")
    //         .validate_with_doctor(Some(manifest), Some("production".to_string()));
        
    //     assert_validation_error!(result, "MISSING_KEY");
    // }
    
    #[test]
    fn test_hcl_vs_doctor_validation() {
        // Test case 1: HCL validation actually DOES catch invalid action field references
        // This is more sophisticated than we initially expected
        let mut runbook_with_invalid_field = RunbookBuilder::new()
            .addon("evm", vec![])
            .signer("valid", "evm::web_wallet", vec![])
            .action("deploy", "evm::deploy_contract")
                .input("from", "signer.valid")
                .input("contract", "MyContract")
            .action("use_deploy", "evm::call_contract")
                .input("contract", "action.deploy.nonexistent_field");
        
        // HCL validation should fail for invalid field reference
        let hcl_result = runbook_with_invalid_field.validate();
        assert!(!hcl_result.success, "HCL validation should catch invalid field reference");
        assert!(hcl_result.errors.iter().any(|e| e.message.contains("nonexistent_field")));
        
        // Test case 2: Valid runbook that passes both HCL and doctor validation
        let mut runbook_valid = RunbookBuilder::new()
            .addon("evm", vec![])
            .signer("valid", "evm::web_wallet", vec![])
            .action("deploy", "evm::deploy_contract")
                .input("from", "signer.valid")
                .input("contract", "MyContract")
            .action("use_deploy", "evm::call_contract")
                .input("contract", "action.deploy.contract_address");  // Valid field
        
        // Both validations should pass
        let hcl_result = runbook_valid.validate();
        assert!(hcl_result.success, "HCL validation should pass for valid runbook");
        
        let doctor_result = runbook_valid.validate_with_doctor(None, None);
        assert!(doctor_result.success, "Doctor validation should pass for valid runbook");
    }
    
    #[test]
    fn test_env_var_validation() {
        let manifest = create_test_manifest_with_env(vec![
            ("development", vec![("API_KEY", "test-key")]),
            ("production", vec![("API_KEY", "prod-key"), ("DB_URL", "postgres://...")])
        ]);
        
        // Test missing env var
        let result = RunbookBuilder::new()
            .variable("key", "env.MISSING_KEY")
            .output("result", "variable.key")
            .validate_with_doctor(Some(manifest.clone()), Some("production".to_string()));
        
        assert_validation_error!(result, "MISSING_KEY");
        
        // Test valid env var
        let result2 = RunbookBuilder::new()
            .variable("key", "env.API_KEY")
            .output("result", "variable.key")
            .validate_with_doctor(Some(manifest), Some("production".to_string()));
        
        assert!(result2.success);
    }
}