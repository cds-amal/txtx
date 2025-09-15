use std::collections::HashMap;
use txtx_addon_kit::types::diagnostics::Diagnostic;
use txtx_addon_kit::serde_json;

/// Validation result for a runbook
#[derive(Debug)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
}

/// Parse result for a runbook
#[derive(Debug)]
pub struct ParseResult {
    pub runbook: Option<String>,
    pub errors: Vec<String>,
}



/// Execution result for a runbook
pub struct ExecutionResult {
    pub success: bool,
    pub outputs: HashMap<String, String>,
    pub errors: Vec<String>,
}

/// Builder for creating and testing runbooks
pub struct RunbookBuilder {
    /// The main runbook content
    content: String,
    /// Additional files for multi-file runbooks
    files: HashMap<String, String>,
    /// Environment variables by environment name
    environments: HashMap<String, HashMap<String, String>>,
    /// Mock blockchain configurations
    mocks: HashMap<String, MockConfig>,
    /// CLI inputs
    cli_inputs: HashMap<String, String>,
    /// Current building state for fluent API
    building_content: Vec<String>,
    /// Current action being built
    current_action: Option<String>,
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
            cli_inputs: HashMap::new(),
            building_content: Vec::new(),
            current_action: None,
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
    
    /// Add CLI input
    pub fn with_cli_input(mut self, key: &str, value: &str) -> Self {
        self.cli_inputs.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Add a mock blockchain
    pub fn with_mock(mut self, name: &str, config: MockConfig) -> Self {
        self.mocks.insert(name.to_string(), config);
        self
    }
    
    /// Add an addon
    pub fn addon(mut self, name: &str, config: Vec<(&str, &str)>) -> Self {
        let config_str = config
            .into_iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        self.building_content.push(format!(r#"addon "{}" {{ {} }}"#, name, config_str));
        self
    }
    
    /// Add a variable
    pub fn variable(mut self, name: &str, value: &str) -> Self {
        self.building_content.push(format!(r#"
variable "{}" {{
    value = {}
}}"#, name, 
            if value.starts_with("env.") || value.starts_with("input.") || value.starts_with("action.") {
                value.to_string()
            } else {
                format!(r#""{}""#, value)
            }
        ));
        self
    }
    
    /// Add an action
    pub fn action(mut self, name: &str, action_type: &str) -> Self {
        // Close any previous action
        if self.current_action.is_some() {
            self.building_content.push("}".to_string());
        }
        self.current_action = Some(name.to_string());
        self.building_content.push(format!(r#"
action "{}" "{}" {{"#, name, action_type));
        self
    }
    
    /// Add an input to the current action
    pub fn input(mut self, name: &str, value: &str) -> Self {
        if self.current_action.is_some() {
            self.building_content.push(format!("    {} = {}", name, 
                if value.starts_with("signer.") || value.starts_with("input.") || value.starts_with("action.") || value.parse::<i64>().is_ok() {
                    value.to_string()
                } else {
                    format!(r#""{}""#, value)
                }
            ));
        }
        self
    }
    
    /// Add an output
    pub fn output(mut self, name: &str, value: &str) -> Self {
        // Close any open action
        if self.current_action.is_some() {
            self.building_content.push("}".to_string());
            self.current_action = None;
        }
        self.building_content.push(format!(r#"
output "{}" {{
    value = {}
}}"#, name, value));
        self
    }
    
    /// Add a signer
    pub fn signer(mut self, name: &str, signer_type: &str, config: Vec<(&str, &str)>) -> Self {
        // Close any open action
        if self.current_action.is_some() {
            self.building_content.push("}".to_string());
            self.current_action = None;
        }
        
        let config_lines = config
            .into_iter()
            .map(|(k, v)| format!("    {} = \"{}\"", k, v))
            .collect::<Vec<_>>()
            .join("\n");
            
        self.building_content.push(format!(r#"
signer "{}" "{}" {{
{}
}}"#, name, signer_type, config_lines));
        self
    }
    
    /// Build the final content
    fn build_content(&mut self) -> String {
        // Close any open action
        if self.current_action.is_some() {
            self.building_content.push("}".to_string());
            self.current_action = None;
        }
        
        if !self.content.is_empty() {
            self.content.clone()
        } else {
            self.building_content.join("\n")
        }
    }
    
    /// Parse the runbook without validation
    pub fn parse(&self) -> ParseResult {
        // TODO: Implement actual parsing
        // For now, return a placeholder
        ParseResult {
            runbook: None,
            errors: vec![],
        }
    }
    
    /// Validate the runbook without execution
    pub fn validate(&mut self) -> ValidationResult {
        let content = self.build_content();
        // Use the simple validator
        crate::simple_validator::validate_content(&content)
    }
    
    /// Execute the runbook
    pub async fn execute(&self) -> ExecutionResult {
        // TODO: Implement actual execution
        // For now, return a placeholder
        ExecutionResult {
            success: true,
            outputs: HashMap::new(),
            errors: vec![],
        }
    }
    

}