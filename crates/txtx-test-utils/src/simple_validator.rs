//! Simple validation wrapper for tests
//! 
//! This provides a minimal interface to the existing validation logic
//! 
//! ## Known Limitations
//! 
//! 1. Circular dependency detection between actions is not implemented
//! 2. Deep addon configuration validation only checks for presence of fields

use txtx_addon_kit::types::diagnostics::Diagnostic;
use txtx_core::validation::{ValidationResult as CoreResult, hcl_validator};
use crate::builders::ValidationResult;
use crate::addon_registry::{get_all_addons, extract_addon_specifications};

/// Validate runbook content using the existing validation infrastructure
pub fn validate_content(content: &str) -> ValidationResult {
    // Create core validation result
    let mut core_result = CoreResult {
        errors: Vec::new(),
        warnings: Vec::new(),
        suggestions: Vec::new(),
    };
    
    // Get addon specifications  
    let addons = get_all_addons();
    let addon_specs = extract_addon_specifications(&addons);
    
    // Run validation
    let _ = hcl_validator::validate_with_hcl_and_addons(
        content, 
        &mut core_result, 
        "test.tx", 
        addon_specs
    );
    
    // Convert errors to our type
    let errors: Vec<Diagnostic> = core_result.errors.into_iter()
        .map(|e| Diagnostic::error_from_string(e.message.clone()))
        .collect();
    
    ValidationResult {
        success: errors.is_empty(),
        errors,
    }
}