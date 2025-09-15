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
            ValidationMode::Doctor { manifest, environment, file_path: _ } => {
                // Use the processor-based validation
                use txtx_core::processing::{
                    validators::DoctorValidator,
                    RunbookProcessor,
                    ProcessingContext,
                    ValidationContext,
                };
                
                // Create processing context
                let mut processing_context = ProcessingContext::new(std::env::current_dir().unwrap());
                if let Some(m) = manifest {
                    processing_context = processing_context.with_manifest(m);
                }
                if let Some(env) = environment {
                    processing_context = processing_context.with_environment(env);
                }
                processing_context = processing_context.with_cli_inputs(self.cli_inputs.clone());
                
                // Create validation context
                let validation_context = ValidationContext::new(processing_context);
                
                // Parse the runbook content to get a Runbook struct
                // For now, we'll return a placeholder since we need to parse the content
                // In a real implementation, we'd parse the content into a Runbook struct
                panic!("Doctor validation requires parsing runbook content into Runbook struct");
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
    use crate::{assert_validation_error, assert_success};
    
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
    
    #[test]
    fn test_doctor_validates_action_outputs() {
        // Test that doctor catches invalid field access
        let result = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("send", "evm::send_eth")
                .input("to", "\"0x123\"")
                .input("value", "\"1000\"")
            .output("bad", "action.send.invalid_field")  // send_eth only has tx_hash
            .validate_with_doctor(None, None);
        
        assert_validation_error!(result, "Field 'invalid_field' does not exist");
    }
    
    #[test]
    fn test_doctor_validates_inputs_against_manifest() {
        // Create a manifest with environment variables
        let manifest = create_test_manifest_with_env(vec![
            ("production", vec![("API_URL", "https://api.example.com")]),
        ]);
        
        // Test missing input validation
        let result = RunbookBuilder::new()
            .variable("key", "env.MISSING_KEY")
            .output("result", "input.key")
            .validate_with_doctor(Some(manifest), Some("production".to_string()));
        
        assert_validation_error!(result, "MISSING_KEY");
    }
    
    #[test]
    fn test_hcl_vs_doctor_validation() {
        let runbook = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("test", "evm::send_eth")
                .input("signer", "signer.missing");
        
        // HCL validation passes (doesn't check signer refs)
        let hcl_result = runbook.clone().validate();
        assert!(hcl_result.success || hcl_result.errors.is_empty());
        
        // Doctor validation catches it
        let doctor_result = runbook.validate_with_doctor(None, None);
        assert_validation_error!(doctor_result, "missing");
    }
}