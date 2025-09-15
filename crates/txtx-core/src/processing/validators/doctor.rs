//! Doctor validation processor - enhanced validation with cross-reference checking

use crate::processing::{RunbookProcessor, ValidationResult, ProcessorError, ValidationContext};
use crate::runbook::Runbook;
use error_stack::{Result, Report};
use std::collections::HashMap;

/// Enhanced validator that performs deep validation including:
/// - Cross-reference validation between actions
/// - Signer reference validation  
/// - Action output field validation
/// - Input/environment variable validation
pub struct DoctorValidator {
    /// Additional validation rules to apply
    rules: Vec<Box<dyn ValidationRule>>,
}

/// A validation rule that can be applied to a runbook
pub trait ValidationRule: Send + Sync {
    /// Apply this rule to the runbook
    fn validate(&self, runbook: &Runbook, context: &ValidationContext) -> Vec<ValidationIssue>;
    
    /// Get the name of this rule
    fn name(&self) -> &str;
}

/// An issue found during validation
pub struct ValidationIssue {
    pub level: IssueLevel,
    pub message: String,
    pub location: Option<String>,
}

pub enum IssueLevel {
    Error,
    Warning,
}

impl DoctorValidator {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(SignerReferenceRule),
                Box::new(ActionReferenceRule),
                Box::new(ActionOutputFieldRule),
                Box::new(EnvironmentVariableRule),
            ],
        }
    }
    
    pub fn with_rules(mut self, rules: Vec<Box<dyn ValidationRule>>) -> Self {
        self.rules = rules;
        self
    }
}

impl RunbookProcessor for DoctorValidator {
    type Output = ValidationResult;
    type Error = ProcessorError;
    type Context = ValidationContext;
    
    fn process(
        &self,
        runbook: &Runbook,
        context: Self::Context,
    ) -> Result<Self::Output, Self::Error> {
        let mut result = ValidationResult::new();
        
        // First run basic validation
        let basic_validator = super::BasicValidator::new();
        let basic_result = basic_validator.process(runbook, context.clone())?;
        
        // Copy basic validation results
        for error in basic_result.errors {
            result.errors.push(error);
        }
        for warning in basic_result.warnings {
            result.warnings.push(warning);
        }
        
        // Then apply doctor-specific rules
        for rule in &self.rules {
            let issues = rule.validate(runbook, &context);
            for issue in issues {
                match issue.level {
                    IssueLevel::Error => {
                        result.add_error(issue.message);
                    }
                    IssueLevel::Warning => {
                        result.add_warning(issue.message);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn name(&self) -> &str {
        "DoctorValidator"
    }
}

// Example validation rules

struct SignerReferenceRule;

impl ValidationRule for SignerReferenceRule {
    fn validate(&self, runbook: &Runbook, _context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Collect all defined signers
        let defined_signers: HashMap<String, ()> = runbook.signers.iter()
            .map(|(name, _)| (name.clone(), ()))
            .collect();
        
        // Check all signer references in actions
        for (action_name, action) in &runbook.actions {
            if let Some(signer_ref) = action.inputs.get("signer") {
                if let Some(signer_name) = extract_signer_name(signer_ref) {
                    if !defined_signers.contains_key(&signer_name) {
                        issues.push(ValidationIssue {
                            level: IssueLevel::Error,
                            message: format!(
                                "Action '{}' references undefined signer '{}'",
                                action_name, signer_name
                            ),
                            location: Some(format!("action.{}.signer", action_name)),
                        });
                    }
                }
            }
        }
        
        issues
    }
    
    fn name(&self) -> &str {
        "SignerReferenceRule"
    }
}

struct ActionReferenceRule;

impl ValidationRule for ActionReferenceRule {
    fn validate(&self, runbook: &Runbook, _context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Collect all defined actions
        let defined_actions: HashMap<String, ()> = runbook.actions.iter()
            .map(|(name, _)| (name.clone(), ()))
            .collect();
        
        // Check all action references
        for (source_name, action) in &runbook.actions {
            for (input_name, input_value) in &action.inputs {
                if let Some(referenced_action) = extract_action_reference(input_value) {
                    if !defined_actions.contains_key(&referenced_action) {
                        issues.push(ValidationIssue {
                            level: IssueLevel::Error,
                            message: format!(
                                "Action '{}' references undefined action '{}' in input '{}'",
                                source_name, referenced_action, input_name
                            ),
                            location: Some(format!("action.{}.{}", source_name, input_name)),
                        });
                    }
                }
            }
        }
        
        issues
    }
    
    fn name(&self) -> &str {
        "ActionReferenceRule"
    }
}

struct ActionOutputFieldRule;

impl ValidationRule for ActionOutputFieldRule {
    fn validate(&self, runbook: &Runbook, _context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // This would need access to addon specifications to know valid output fields
        // For now, we'll just demonstrate the pattern
        
        issues
    }
    
    fn name(&self) -> &str {
        "ActionOutputFieldRule"
    }
}

struct EnvironmentVariableRule;

impl ValidationRule for EnvironmentVariableRule {
    fn validate(&self, runbook: &Runbook, context: &ValidationContext) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        // Get environment variables from manifest
        let env_vars = context.base.get_env_vars();
        
        // Check all env references
        for (name, variable) in &runbook.variables {
            if let Some(env_var) = extract_env_reference(&variable.value) {
                if let Some(vars) = env_vars {
                    if !vars.contains_key(&env_var) {
                        issues.push(ValidationIssue {
                            level: IssueLevel::Error,
                            message: format!(
                                "Variable '{}' references undefined environment variable '{}'",
                                name, env_var
                            ),
                            location: Some(format!("variable.{}", name)),
                        });
                    }
                }
            }
        }
        
        issues
    }
    
    fn name(&self) -> &str {
        "EnvironmentVariableRule"
    }
}

// Helper functions to extract references from strings
// These would need to be more sophisticated in a real implementation

fn extract_signer_name(value: &str) -> Option<String> {
    if value.starts_with("signer.") {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() >= 2 {
            return Some(parts[1].to_string());
        }
    }
    None
}

fn extract_action_reference(value: &str) -> Option<String> {
    if value.starts_with("action.") {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() >= 2 {
            return Some(parts[1].to_string());
        }
    }
    None
}

fn extract_env_reference(value: &str) -> Option<String> {
    if value.starts_with("env.") {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() >= 2 {
            return Some(parts[1].to_string());
        }
    }
    None
}