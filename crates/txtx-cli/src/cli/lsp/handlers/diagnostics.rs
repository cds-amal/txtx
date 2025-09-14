//! Diagnostics handler

use lsp_types::*;
use crate::cli::lsp::workspace::SharedWorkspaceState;
// use crate::cli::lsp::validation::DoctorValidationAdapter;
use super::Handler;

pub struct DiagnosticsHandler {
    workspace: SharedWorkspaceState,
    // validator: DoctorValidationAdapter,
}

impl DiagnosticsHandler {
    pub fn new(workspace: SharedWorkspaceState) -> Self {
        Self { 
            workspace,
            // validator: DoctorValidationAdapter::new(),
        }
    }
    
    pub fn validate(&self, uri: &Url) -> Option<PublishDiagnosticsParams> {
        let workspace = self.workspace.read();
        let document = workspace.get_document(uri)?;
        
        // Use doctor validation rules if it's a runbook
        let diagnostics = if document.is_runbook() {
            // TODO: Integrate doctor validation once we convert between Manifest types
            // For now, return empty diagnostics
            Vec::new()
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
                // TODO: Integrate doctor validation once we convert between Manifest types
                // For now, return empty diagnostics
                Vec::new()
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