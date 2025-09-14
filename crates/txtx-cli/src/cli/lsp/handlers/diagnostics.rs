//! Diagnostics handler

use lsp_types::*;
use crate::cli::lsp::workspace::SharedWorkspaceState;
use crate::cli::lsp::validation::DoctorValidationAdapter;
use super::Handler;

pub struct DiagnosticsHandler {
    workspace: SharedWorkspaceState,
    #[allow(dead_code)]
    validator: DoctorValidationAdapter,
}

impl DiagnosticsHandler {
    pub fn new(workspace: SharedWorkspaceState) -> Self {
        Self { 
            workspace,
            validator: DoctorValidationAdapter::new(),
        }
    }
    
    pub fn validate(&self, uri: &Url) -> Option<PublishDiagnosticsParams> {
        let workspace = self.workspace.read();
        let document = workspace.get_document(uri)?;
        
        // Use doctor validation rules if it's a runbook
        let diagnostics = if document.is_runbook() {
            // Try to find the manifest for this runbook
            let manifest = workspace.get_manifest_for_document(uri);
            
            // Use enhanced validation with doctor rules if we have a manifest
            if let Some(manifest) = manifest {
                crate::cli::lsp::diagnostics_enhanced::validate_runbook_with_doctor_rules(
                    uri,
                    document.content(),
                    Some(manifest),
                    None, // TODO: Get environment from workspace state
                    &[], // TODO: Get CLI inputs from workspace state
                )
            } else {
                // Fall back to basic HCL validation
                crate::cli::lsp::diagnostics::validate_runbook(uri, document.content())
            }
        } else {
            Vec::new()
        };
        
        Some(PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics,
            version: Some(document.version()),
        })
    }
    
    /// Get diagnostics without PublishDiagnosticsParams wrapper
    pub fn get_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let workspace = self.workspace.read();
        
        if let Some(document) = workspace.get_document(uri) {
            if document.is_runbook() {
                // Try to find the manifest for this runbook
                let manifest = workspace.get_manifest_for_document(uri);
                
                // Use enhanced validation with doctor rules if we have a manifest
                if let Some(manifest) = manifest {
                    crate::cli::lsp::diagnostics_enhanced::validate_runbook_with_doctor_rules(
                        uri,
                        document.content(),
                        Some(manifest),
                        None, // TODO: Get environment from workspace state
                        &[], // TODO: Get CLI inputs from workspace state
                    )
                } else {
                    // Fall back to basic HCL validation
                    crate::cli::lsp::diagnostics::validate_runbook(uri, document.content())
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}

impl Handler for DiagnosticsHandler {
    fn workspace(&self) -> &SharedWorkspaceState {
        &self.workspace
    }
}