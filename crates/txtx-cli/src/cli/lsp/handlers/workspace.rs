//! Workspace-related handlers for environment management
//!
//! This module provides custom LSP handlers for workspace operations,
//! specifically for environment selection and management.

use super::SharedWorkspaceState;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetEnvironmentParams {
    pub environment: String,
}

/// Handler for workspace-related requests
pub struct WorkspaceHandler {
    workspace_state: SharedWorkspaceState,
    current_environment: std::sync::RwLock<Option<String>>,
}

impl WorkspaceHandler {
    pub fn new(workspace_state: SharedWorkspaceState) -> Self {
        Self { workspace_state, current_environment: std::sync::RwLock::new(None) }
    }

    /// Get the workspace state
    pub fn workspace_state(&self) -> &SharedWorkspaceState {
        &self.workspace_state
    }

    /// Get the list of available environments in the workspace
    pub fn get_environments(&self) -> Vec<String> {
        eprintln!("[DEBUG] Getting available environments");

        let mut environments = HashSet::new();
        environments.insert("global".to_string());

        // Read workspace state to find all .tx files
        let workspace = self.workspace_state.read();

        // Check if we have any documents open
        for (uri, _doc) in workspace.documents() {
            if let Ok(path) = uri.to_file_path() {
                if let Some(env) = extract_environment_from_path(&path) {
                    environments.insert(env);
                }
            }
        }

        // Also check manifest for defined environments
        if let Some((_uri, manifest)) = workspace
            .documents()
            .iter()
            .find(|(uri, _)| uri.path().ends_with("txtx.yml") || uri.path().ends_with("txtx.yaml"))
            .and_then(|(uri, _)| workspace.get_manifest_for_document(uri).map(|m| (uri, m)))
        {
            for env_name in manifest.environments.keys() {
                environments.insert(env_name.clone());
            }
        }

        // If we don't have many environments from open files, scan the workspace
        if environments.len() <= 2 {
            if let Some(workspace_root) = self.find_workspace_root() {
                eprintln!("[DEBUG] Scanning workspace root: {:?}", workspace_root);
                if let Ok(tx_files) = find_tx_files(&workspace_root) {
                    for file in tx_files {
                        if let Some(env) = extract_environment_from_path(&file) {
                            environments.insert(env);
                        }
                    }
                }
            }
        }

        let mut env_list: Vec<String> = environments.into_iter().collect();
        env_list.sort();

        eprintln!("[DEBUG] Found environments: {:?}", env_list);
        env_list
    }

    /// Set the current environment for validation
    pub fn set_environment(&self, environment: String) {
        eprintln!("[DEBUG] Setting environment to: {}", environment);
        *self.current_environment.write().unwrap() = Some(environment.clone());
        // Also update in the workspace state
        self.workspace_state.write().set_current_environment(Some(environment));
    }

    /// Get the current environment
    pub fn get_current_environment(&self) -> Option<String> {
        // Get from workspace state instead of local field
        self.workspace_state.read().get_current_environment()
    }

    /// Find the workspace root by looking for txtx.yml
    fn find_workspace_root(&self) -> Option<PathBuf> {
        let workspace = self.workspace_state.read();

        // Try to find from open documents
        for (uri, _) in workspace.documents() {
            if let Ok(path) = uri.to_file_path() {
                if let Some(root) = find_txtx_yml_root(&path) {
                    return Some(root);
                }
            }
        }

        None
    }
}

/// Extract environment from file path
/// For "config.aws.prod.tx", returns Some("prod")
/// For "main.tx", returns None
fn extract_environment_from_path(path: &PathBuf) -> Option<String> {
    let file_name = path.file_name()?.to_str()?;

    // Must end with .tx
    if !file_name.ends_with(".tx") {
        return None;
    }

    // Remove .tx extension
    let without_ext = &file_name[..file_name.len() - 3];

    // Split by dots
    let parts: Vec<&str> = without_ext.split('.').collect();

    // If there are at least 2 parts, the last one is the environment
    if parts.len() >= 2 {
        Some(parts[parts.len() - 1].to_string())
    } else {
        None
    }
}

/// Find the root directory containing txtx.yml
fn find_txtx_yml_root(start_path: &PathBuf) -> Option<PathBuf> {
    let mut current =
        if start_path.is_file() { start_path.parent()? } else { start_path.as_path() };

    loop {
        for name in &["txtx.yml", "txtx.yaml"] {
            if current.join(name).exists() {
                return Some(current.to_path_buf());
            }
        }

        current = current.parent()?;
    }
}

/// Find all .tx files in a directory recursively
fn find_tx_files(root: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut tx_files = Vec::new();
    find_tx_files_recursive(root, &mut tx_files)?;
    Ok(tx_files)
}

fn find_tx_files_recursive(dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip hidden directories and common build directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if !name_str.starts_with('.')
                    && !["target", "node_modules", "dist"].contains(&name_str.as_ref())
                {
                    find_tx_files_recursive(&path, files)?;
                }
            }
        } else if let Some(ext) = path.extension() {
            if ext == "tx" {
                files.push(path);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_environment_from_path() {
        assert_eq!(
            extract_environment_from_path(&PathBuf::from("deploy.prod.tx")),
            Some("prod".to_string())
        );
        assert_eq!(
            extract_environment_from_path(&PathBuf::from("config.aws.staging.tx")),
            Some("staging".to_string())
        );
        assert_eq!(extract_environment_from_path(&PathBuf::from("main.tx")), None);
        assert_eq!(extract_environment_from_path(&PathBuf::from("test.yml")), None);
    }
}
