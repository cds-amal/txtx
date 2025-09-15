//! Basic HCL validation processor

use crate::processing::{RunbookProcessor, ValidationResult, ProcessorError, ValidationContext};
use crate::runbook::Runbook;
use error_stack::{Result, Report};

/// Basic validator that performs HCL syntax and structure validation
pub struct BasicValidator {
    // Could add configuration options here
}

impl BasicValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl RunbookProcessor for BasicValidator {
    type Output = ValidationResult;
    type Error = ProcessorError;
    type Context = ValidationContext;
    
    fn process(
        &self,
        runbook: &Runbook,
        context: Self::Context,
    ) -> Result<Self::Output, Self::Error> {
        let mut result = ValidationResult::new();
        
        // For now, we'll use the existing validation logic
        // In a real implementation, we'd move the validation logic here
        match crate::validation::hcl_validator::validate_runbook(runbook, None, &context.base.cli_inputs) {
            Ok(validation_result) => {
                // Convert the existing validation result to our new format
                for error in validation_result.errors {
                    result.add_error(error.message);
                }
                for warning in validation_result.warnings {
                    result.add_warning(warning.message);
                }
                Ok(result)
            }
            Err(e) => {
                result.add_error(format!("Validation failed: {:?}", e));
                Ok(result)
            }
        }
    }
    
    fn name(&self) -> &str {
        "BasicValidator"
    }
}