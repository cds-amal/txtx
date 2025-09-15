//! Common context types for processors

use std::collections::HashMap;
use std::path::PathBuf;
use crate::manifest::WorkspaceManifest;
use indexmap::IndexMap;

/// General processing context that most processors will use
#[derive(Debug, Clone)]
pub struct ProcessingContext {
    /// Optional workspace manifest
    pub manifest: Option<WorkspaceManifest>,
    
    /// Selected environment name
    pub environment: Option<String>,
    
    /// CLI input overrides
    pub cli_inputs: HashMap<String, String>,
    
    /// Working directory
    pub working_dir: PathBuf,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ProcessingContext {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            manifest: None,
            environment: None,
            cli_inputs: HashMap::new(),
            working_dir,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_manifest(mut self, manifest: WorkspaceManifest) -> Self {
        self.manifest = Some(manifest);
        self
    }
    
    pub fn with_environment(mut self, environment: String) -> Self {
        self.environment = Some(environment);
        self
    }
    
    pub fn with_cli_inputs(mut self, inputs: HashMap<String, String>) -> Self {
        self.cli_inputs = inputs;
        self
    }
    
    /// Get environment variables for the selected environment
    pub fn get_env_vars(&self) -> Option<&IndexMap<String, String>> {
        self.manifest.as_ref()
            .and_then(|m| self.environment.as_ref()
                .and_then(|env| m.environments.get(env)))
    }
}

/// Specialized context for validation processors
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Base processing context
    pub base: ProcessingContext,
    
    /// Whether to perform strict validation
    pub strict: bool,
    
    /// Additional validation rules to apply
    pub extra_rules: Vec<String>,
}

impl ValidationContext {
    pub fn new(base: ProcessingContext) -> Self {
        Self {
            base,
            strict: false,
            extra_rules: Vec::new(),
        }
    }
    
    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }
    
    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.extra_rules = rules;
        self
    }
}

impl From<ProcessingContext> for ValidationContext {
    fn from(base: ProcessingContext) -> Self {
        Self::new(base)
    }
}