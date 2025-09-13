//! Integration with the shared validation module for real-time diagnostics

use txtx_addon_kit::helpers::fs::FileLocation;
use txtx_addon_kit::types::diagnostics::{Diagnostic as TxtxDiagnostic, DiagnosticLevel, Span};
use txtx_core::validation::{ValidationResult, ValidatorConfig, validate_runbook};

/// Convert validation results to LSP diagnostics
pub fn validation_to_lsp_diagnostics(
    validation_result: ValidationResult,
    file_location: &FileLocation,
) -> Vec<(FileLocation, Vec<TxtxDiagnostic>)> {
    let mut diagnostics = Vec::new();
    
    // Convert errors
    for error in validation_result.errors {
        let mut diag = TxtxDiagnostic::error_from_string(error.message.clone());
        
        // Add context if available
        if let Some(context) = &error.context {
            diag.notes.push(context.clone());
        }
        
        // Add span if we have location info
        if let (Some(line), Some(column)) = (error.line, error.column) {
            diag.span = Some(Span {
                start: 0,
                end: 0,
                span_range: None,
                line_start: line,
                line_stop: line,
                column_start: column,
                column_stop: column + 10,
            });
        }
        
        diagnostics.push(diag);
    }
    
    // Convert warnings
    for warning in validation_result.warnings {
        let mut diag = TxtxDiagnostic::warn_from_string(warning.message.clone());
        
        if let Some(suggestion) = &warning.suggestion {
            diag.hints.push(suggestion.clone());
        }
        
        if let (Some(line), Some(column)) = (warning.line, warning.column) {
            diag.span = Some(Span {
                start: 0,
                end: 0,
                span_range: None,
                line_start: line,
                line_stop: line,
                column_start: column,
                column_stop: column + 10,
            });
        }
        
        diagnostics.push(diag);
    }
    
    vec![(file_location.clone(), diagnostics)]
}

/// Run validation on a runbook file
pub fn validate_runbook_for_lsp(
    file_path: &str,
    source: &str,
    body: &txtx_addon_kit::hcl::structure::Body,
) -> ValidationResult {
    // For now, use default config
    // In the future, this should be populated with actual addon specs
    let config = ValidatorConfig::new();
    
    validate_runbook(file_path, source, body, config)
}