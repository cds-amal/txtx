//! Manifest parsing and management for the LSP workspace
//! 
//! This module handles parsing and indexing of Txtx.toml manifest files,
//! including tracking runbook references and environment configurations.

use lsp_types::Url;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Represents a parsed txtx manifest
#[derive(Debug, Clone)]
pub struct Manifest {
    pub uri: Url,
    pub runbooks: Vec<RunbookRef>,
    pub environments: HashMap<String, HashMap<String, String>>,
}

/// Reference to a runbook from a manifest
#[derive(Debug, Clone)]
pub struct RunbookRef {
    pub name: String,
    pub location: String,
    pub absolute_uri: Option<Url>,
}

impl Manifest {
    /// Parse a manifest from content
    pub fn parse(uri: Url, content: &str) -> Result<Self, String> {
        // Parse TOML content
        let toml_value: toml::Value = toml::from_str(content)
            .map_err(|e| format!("Failed to parse TOML: {}", e))?;
        
        let toml_table = toml_value.as_table()
            .ok_or("Expected TOML table at root")?;
        
        // Extract runbooks
        let mut runbooks = Vec::new();
        if let Some(runbooks_section) = toml_table.get("runbooks").and_then(|v| v.as_table()) {
            for (name, value) in runbooks_section {
                if let Some(location) = value.as_str() {
                    let absolute_uri = resolve_runbook_uri(&uri, location).ok();
                    runbooks.push(RunbookRef {
                        name: name.clone(),
                        location: location.to_string(),
                        absolute_uri,
                    });
                }
            }
        }
        
        // Extract environments
        let mut environments = HashMap::new();
        if let Some(envs_section) = toml_table.get("environments").and_then(|v| v.as_table()) {
            for (env_name, env_value) in envs_section {
                if let Some(env_table) = env_value.as_table() {
                    let mut env_vars = HashMap::new();
                    for (key, value) in env_table {
                        if let Some(str_value) = value.as_str() {
                            env_vars.insert(key.clone(), str_value.to_string());
                        }
                    }
                    environments.insert(env_name.clone(), env_vars);
                }
            }
        }
        
        Ok(Manifest {
            uri,
            runbooks,
            environments,
        })
    }
    
    /// Find a runbook by name
    pub fn find_runbook(&self, name: &str) -> Option<&RunbookRef> {
        self.runbooks.iter().find(|r| r.name == name)
    }
    
    /// Get environment variables for a specific environment
    pub fn get_environment(&self, name: &str) -> Option<&HashMap<String, String>> {
        self.environments.get(name)
    }
}

/// Resolve a runbook location relative to a manifest URI
fn resolve_runbook_uri(manifest_uri: &Url, location: &str) -> Result<Url, String> {
    let manifest_path = manifest_uri.to_file_path()
        .map_err(|_| "Failed to convert manifest URI to path")?;
    
    let manifest_dir = manifest_path.parent()
        .ok_or("Manifest has no parent directory")?;
    
    let runbook_path = manifest_dir.join(location);
    
    Url::from_file_path(&runbook_path)
        .map_err(|_| format!("Failed to convert path to URI: {:?}", runbook_path))
}

/// Find the manifest file for a given runbook
pub fn find_manifest_for_runbook(runbook_uri: &Url) -> Option<Url> {
    let runbook_path = runbook_uri.to_file_path().ok()?;
    let mut current_dir = runbook_path.parent()?;
    
    // Walk up the directory tree looking for Txtx.toml
    loop {
        let manifest_path = current_dir.join("Txtx.toml");
        if manifest_path.exists() {
            return Url::from_file_path(&manifest_path).ok();
        }
        
        let alt_manifest_path = current_dir.join("txtx.toml");
        if alt_manifest_path.exists() {
            return Url::from_file_path(&alt_manifest_path).ok();
        }
        
        current_dir = current_dir.parent()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_manifest_parsing() {
        let content = r#"
            [runbooks]
            deploy = "runbooks/deploy.tx"
            test = "runbooks/test.tx"
            
            [environments.prod]
            api_key = "prod_key"
            url = "https://prod.example.com"
            
            [environments.dev]
            api_key = "dev_key"
            url = "https://dev.example.com"
        "#;
        
        let uri = Url::parse("file:///project/Txtx.toml").unwrap();
        let manifest = Manifest::parse(uri, content).unwrap();
        
        assert_eq!(manifest.runbooks.len(), 2);
        assert_eq!(manifest.environments.len(), 2);
        
        let deploy_runbook = manifest.find_runbook("deploy").unwrap();
        assert_eq!(deploy_runbook.location, "runbooks/deploy.tx");
        
        let prod_env = manifest.get_environment("prod").unwrap();
        assert_eq!(prod_env.get("api_key").unwrap(), "prod_key");
    }
}