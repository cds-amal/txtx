//! LSP validation integration with doctor validation rules
//! 
//! This module bridges the doctor validation framework with LSP diagnostics,
//! allowing us to reuse the same validation logic for real-time feedback.

mod converter;
mod adapter;

pub use adapter::DoctorValidationAdapter;
pub use converter::validation_outcome_to_diagnostic;
