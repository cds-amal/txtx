//! Shared validation module for runbook files
//! 
//! This module provides validation functionality that is shared between
//! the doctor command (CLI) and the LSP for real-time error detection.

pub mod addon_specifications;
pub mod hcl_validator;
pub mod hcl_diagnostics;
pub mod manifest_validator;
pub mod types;
pub mod validator;

pub use types::{
    LocatedInputRef, ValidationError, ValidationResult, ValidationSuggestion, ValidationWarning,
};
pub use validator::{validate_runbook, ValidatorConfig};
pub use manifest_validator::{
    validate_inputs_against_manifest, ManifestValidationConfig, ManifestValidationRule,
    ManifestValidationContext, ValidationOutcome,
};