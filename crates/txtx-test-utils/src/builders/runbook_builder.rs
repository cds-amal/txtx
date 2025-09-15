//! Simple builder API for creating runbooks in tests
//! 
//! This provides a fluent API for constructing test scenarios without
//! dealing with the complexity of the full runbook execution environment.

use std::collections::HashMap;
use txtx_addon_kit::types::diagnostics::Diagnostic;

/// Result of parsing a runbook
pub struct ParseResult {
    pub success: bool,
    pub errors: Vec<Diagnostic>,
}

/// Result of validating a runbook
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<Diagnostic>,
}

/// Result of executing a runbook
pub struct ExecutionResult {
    pub success: bool,
    pub outputs: HashMap<String, String>,
    pub errors: Vec<Diagnostic>,
}

/// Builder for constructing runbooks in tests
pub struct RunbookBuilder {
    /// Main runbook content
    content: String,
    /// Additional files for multi-file runbooks
    files: HashMap<String, String>,
    /// Environment variables by environment name
    environments: HashMap<String, HashMap<String, String>>,
    /// Mock configurations
    mocks: HashMap<String, MockConfig>,
}

/// Configuration for a mock blockchain
pub struct MockConfig {
    pub chain_type: String,
    pub initial_state: serde_json::Value,
}

impl RunbookBuilder {
    /// Create a new runbook builder
    pub fn new() -> Self {
        Self {
            content: String::new(),
            files: HashMap::new(),
            environments: HashMap::new(),
            mocks: HashMap::new(),
        }
    }
    
    /// Set the main runbook content
    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
    
    /// Add a file for multi-file runbooks
    pub fn with_file(mut self, path: &str, content: &str) -> Self {
        self.files.insert(path.to_string(), content.to_string());
        self
    }
    
    /// Add environment variables
    pub fn with_environment(mut self, env_name: &str, vars: Vec<(&str, &str)>) -> Self {
        let env_vars: HashMap<String, String> = vars
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.environments.insert(env_name.to_string(), env_vars);
        self
    }
    
    /// Add a mock blockchain
    pub fn with_mock(mut self, name: &str, config: MockConfig) -> Self {
        self.mocks.insert(name.to_string(), config);
        self
    }
    
    /// Parse the runbook without validation
    pub fn parse(&self) -> ParseResult {
        // TODO: Implement actual parsing
        // For now, return a placeholder
        ParseResult {
            success: true,
            errors: vec![],
        }
    }
    
    /// Validate the runbook without execution
    pub fn validate(&self) -> ValidationResult {
        // TODO: Implement actual validation
        // For now, return a placeholder
        ValidationResult {
            success: true,
            errors: vec![],
        }
    }
    
    /// Execute the runbook
    pub fn execute(&self) -> ExecutionResult {
        // TODO: Implement actual execution
        // For now, return a placeholder
        ExecutionResult {
            success: true,
            outputs: HashMap::new(),
            errors: vec![],
        }
    }
}

impl Default for RunbookBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_api() {
        let result = RunbookBuilder::new()
            .with_content(r#"
                variable "message" {
                    value = env.MESSAGE
                }
                
                action "print" "core::print" {
                    message = input.message
                }
            "#)
            .with_environment("test", vec![("MESSAGE", "Hello, World!")])
            .validate();
            
        assert!(result.success);
    }
}