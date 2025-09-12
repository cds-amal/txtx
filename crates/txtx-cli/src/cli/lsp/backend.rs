//! Synchronous LSP backend implementation for txtx.
//!
//! This module implements the Language Server Protocol without async/await,
//! following the rust-analyzer pattern using lsp-server.

use lsp_types::{
    DidOpenTextDocumentParams, DidChangeTextDocumentParams, DidSaveTextDocumentParams,
    DidCloseTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, 
    HoverParams, Hover, CompletionParams, CompletionResponse, CompletionItem,
    Location, Position, Range, Url, MarkupContent, MarkupKind, CompletionItemKind,

};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Represents the state of a single document
#[derive(Debug, Clone)]
struct Document {
    _uri: Url, // Kept for future use (e.g., diagnostics)
    content: String,
    _version: i32, // Kept for incremental sync
}

/// Represents a parsed txtx manifest
#[derive(Debug, Clone)]
struct Manifest {
    uri: Url,
    runbooks: Vec<RunbookRef>,
    environments: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
struct RunbookRef {
    _name: String, // Kept for future use (e.g., runbook listing)
    location: String,
}

/// The workspace state containing all documents and parsed information
struct WorkspaceState {
    /// All open documents indexed by URI
    documents: HashMap<Url, Document>,
    /// Parsed manifests indexed by their URI
    manifests: HashMap<Url, Manifest>,
    /// Map from runbook URI to its manifest URI
    runbook_to_manifest: HashMap<Url, Url>,
    /// Cached environment variables for quick lookup
    environment_vars: HashMap<String, HashMap<String, String>>,
}

impl WorkspaceState {
    fn new() -> Self {
        Self {
            documents: HashMap::new(),
            manifests: HashMap::new(),
            runbook_to_manifest: HashMap::new(),
            environment_vars: HashMap::new(),
        }
    }
    
    fn open_document(&mut self, uri: Url, content: String) {
        self.documents.insert(uri.clone(), Document {
            _uri: uri,
            content,
            _version: 1,
        });
    }
    
    fn update_document(&mut self, uri: Url, content: String) {
        if let Some(doc) = self.documents.get_mut(&uri) {
            doc.content = content;
            doc._version += 1;
        }
    }
    
    fn close_document(&mut self, uri: Url) {
        self.documents.remove(&uri);
    }
    
    fn get_document(&self, uri: &Url) -> Option<&String> {
        self.documents.get(uri).map(|d| &d.content)
    }
    
    fn index_manifest(&mut self, uri: &Url, content: &str) {
        // Parse the YAML manifest
        if let Ok(manifest) = parse_manifest(uri, content) {
            // Store environment variables
            for (env_name, vars) in &manifest.environments {
                self.environment_vars.insert(env_name.clone(), vars.clone());
            }
            
            // Map runbooks to this manifest
            for runbook in &manifest.runbooks {
                if let Ok(runbook_uri) = resolve_relative_uri(uri, &runbook.location) {
                    self.runbook_to_manifest.insert(runbook_uri, uri.clone());
                }
            }
            
            self.manifests.insert(uri.clone(), manifest);
        }
    }
    
    fn index_runbook(&mut self, uri: &Url, _content: &str) {
        // Try to find the manifest for this runbook
        if !self.runbook_to_manifest.contains_key(uri) {
            // Search for txtx.yml in parent directories
            if let Ok(manifest_uri) = find_manifest_for_runbook(uri) {
                eprintln!("  Found manifest at: {}", manifest_uri);
                
                // Load and index the manifest if we haven't already
                if !self.manifests.contains_key(&manifest_uri) {
                    if let Ok(manifest_content) = std::fs::read_to_string(manifest_uri.path()) {
                        // Store the document content
                        self.open_document(manifest_uri.clone(), manifest_content.clone());
                        // Index it
                        self.index_manifest(&manifest_uri, &manifest_content);
                    }
                }
                
                // Map this runbook to the manifest
                self.runbook_to_manifest.insert(uri.clone(), manifest_uri);
            }
        }
    }
    
    fn get_manifest_for_runbook(&self, runbook_uri: &Url) -> Option<&Manifest> {
        self.runbook_to_manifest
            .get(runbook_uri)
            .and_then(|manifest_uri| self.manifests.get(manifest_uri))
    }
}

/// The main synchronous LSP backend
pub struct TxtxLspBackend {
    state: Arc<RwLock<WorkspaceState>>,
}

impl TxtxLspBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(WorkspaceState::new())),
        }
    }
    
    // Document synchronization methods
    
    pub fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        
        eprintln!("[did_open] {}", uri);
        
        let mut state = self.state.write().unwrap();
        state.open_document(uri.clone(), content.clone());
        
        // Index based on file type
        if is_manifest(&uri) {
            state.index_manifest(&uri, &content);
        } else if is_runbook(&uri) {
            state.index_runbook(&uri, &content);
        }
    }
    
    pub fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        
        if let Some(change) = params.content_changes.last() {
            let mut state = self.state.write().unwrap();
            state.update_document(uri.clone(), change.text.clone());
            
            // Re-index
            if is_manifest(&uri) {
                state.index_manifest(&uri, &change.text);
            } else if is_runbook(&uri) {
                state.index_runbook(&uri, &change.text);
            }
        }
    }
    
    pub fn did_save(&self, params: DidSaveTextDocumentParams) {
        eprintln!("[did_save] {}", params.text_document.uri);
    }
    
    pub fn did_close(&self, params: DidCloseTextDocumentParams) {
        eprintln!("[did_close] {}", params.text_document.uri);
        let mut state = self.state.write().unwrap();
        state.close_document(params.text_document.uri);
    }
    
    // Language feature methods
    
    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        eprintln!("[goto_definition] {} at {}:{}", uri, position.line, position.character);
        
        let state = self.state.read().unwrap();
        let content = state.get_document(&uri)?;
        
        // Check if we're on an "inputs.variable_name" pattern
        if let Some(var_name) = extract_input_reference(content, &position) {
            eprintln!("  Found input reference: {}", var_name);
            
            // Get the manifest for this runbook
            if let Some(manifest) = state.get_manifest_for_runbook(&uri) {
                eprintln!("  Found manifest: {}", manifest.uri);
                
                // Check if the variable exists in any environment
                let mut var_exists = false;
                for (_env_name, env) in &manifest.environments {
                    if env.contains_key(&var_name) {
                        var_exists = true;
                        break;
                    }
                }
                
                if var_exists {
                    // Find the line in the manifest where this variable is defined
                    if let Some(manifest_content) = state.get_document(&manifest.uri) {
                        if let Some(line_num) = find_variable_line(manifest_content, &var_name) {
                            return Some(GotoDefinitionResponse::Scalar(Location {
                                uri: manifest.uri.clone(),
                                range: Range {
                                    start: Position { line: line_num, character: 4 },
                                    end: Position { line: line_num, character: 4 + var_name.len() as u32 },
                                },
                            }));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        eprintln!("[hover] {} at {}:{}", uri, position.line, position.character);
        
        let state = self.state.read().unwrap();
        let content = state.get_document(&uri)?;
        
        if let Some(var_name) = extract_input_reference(content, &position) {
            // Look up the variable value
            if let Some(manifest) = state.get_manifest_for_runbook(&uri) {
            // Try to find the variable in any environment
            // First try "default", then "global", then any other
            let env_to_check = ["default", "global"];
            let mut found_value = None;
            
            for env_name in &env_to_check {
                if let Some(env) = manifest.environments.get(*env_name) {
                    if let Some(value) = env.get(&var_name) {
                        found_value = Some((env_name.to_string(), value.clone()));
                        break;
                    }
                }
            }
            
            // If not found in default/global, check all environments
            if found_value.is_none() {
                for (env_name, env) in &manifest.environments {
                    if let Some(value) = env.get(&var_name) {
                        found_value = Some((env_name.clone(), value.clone()));
                        break;
                    }
                }
            }
            
            if let Some((env_name, value)) = found_value {
                let hover_text = format!("**{}**\n\nValue: `{}`\n\nEnvironment: `{}`\n\nDefined in: `{}`", 
                            var_name, value, env_name, manifest.uri.path());
                        
                return Some(Hover {
                    contents: lsp_types::HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_text,
                    }),
                    range: None,
                });
            }
            } // Close the manifest if-let
        }
        
        None
    }
    
    pub fn completion(&self, params: CompletionParams) -> Option<CompletionResponse> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        eprintln!("[completion] {} at {}:{}", uri, position.line, position.character);
        
        let state = self.state.read().unwrap();
        let content = state.get_document(&uri)?;
        
        // Check if we're after "input."
        if is_after_input_dot(content, &position) {
            let mut completions = Vec::new();
            
            // Get available variables from manifest
            if let Some(manifest) = state.get_manifest_for_runbook(&uri) {
                if let Some(default_env) = manifest.environments.get("default") {
                    for (var_name, value) in default_env {
                        completions.push(CompletionItem {
                            label: var_name.clone(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            detail: Some(format!("= {}", value)),
                            documentation: Some(lsp_types::Documentation::String(
                                format!("Input variable from txtx.yml")
                            )),
                            ..Default::default()
                        });
                    }
                }
            }
            
            if !completions.is_empty() {
                return Some(CompletionResponse::Array(completions));
            }
        }
        
        None
    }
}

// Helper functions

fn is_manifest(uri: &Url) -> bool {
    uri.path().ends_with("txtx.yml") || uri.path().ends_with("txtx.yaml")
}

fn is_runbook(uri: &Url) -> bool {
    uri.path().ends_with(".tx")
}

fn parse_manifest(uri: &Url, content: &str) -> Result<Manifest, String> {
    // Simple YAML parsing for txtx manifests
    // In production, use a proper YAML parser
    let mut runbooks = Vec::new();
    let mut environments = HashMap::new();
    
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.starts_with("runbooks:") {
            i += 1;
            while i < lines.len() && lines[i].starts_with("  ") {
                if lines[i].trim().starts_with("- name:") {
                    let name = lines[i].split("name:").nth(1).unwrap_or("").trim().to_string();
                    i += 1;
                    if i < lines.len() && lines[i].trim().starts_with("location:") {
                        let location = lines[i].split("location:").nth(1).unwrap_or("").trim().to_string();
                        runbooks.push(RunbookRef { _name: name, location });
                    }
                }
                i += 1;
            }
        } else if line.starts_with("environments:") {
            i += 1;
            while i < lines.len() && lines[i].starts_with("  ") {
                let env_line = lines[i].trim();
                if !env_line.is_empty() && env_line.ends_with(":") {
                    let env_name = env_line.trim_end_matches(':').to_string();
                    let mut vars = HashMap::new();
                    i += 1;
                    
                    while i < lines.len() && lines[i].starts_with("    ") {
                        let var_line = lines[i].trim();
                        if let Some((key, value)) = var_line.split_once(':') {
                            let key = key.trim().to_string();
                            let value = value.trim().trim_matches('"').to_string();
                            vars.insert(key, value);
                        }
                        i += 1;
                    }
                    
                    environments.insert(env_name, vars);
                    continue;
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    
    Ok(Manifest {
        uri: uri.clone(),
        runbooks,
        environments,
    })
}

fn resolve_relative_uri(base: &Url, relative: &str) -> Result<Url, String> {
    base.join(relative).map_err(|e| e.to_string())
}

fn find_manifest_for_runbook(runbook_uri: &Url) -> Result<Url, String> {
    // Search for txtx.yml in parent directories
    if let Ok(path) = runbook_uri.to_file_path() {
        let mut dir = path.parent();
        
        while let Some(d) = dir {
            let manifest_path = d.join("txtx.yml");
            if manifest_path.exists() {
                return Url::from_file_path(manifest_path)
                    .map_err(|_| "Failed to create URL".to_string());
            }
            
            let manifest_path = d.join("txtx.yaml");
            if manifest_path.exists() {
                return Url::from_file_path(manifest_path)
                    .map_err(|_| "Failed to create URL".to_string());
            }
            
            // Stop at .git directory
            if d.join(".git").exists() {
                break;
            }
            
            dir = d.parent();
        }
    }
    
    Err("No manifest found".to_string())
}

fn extract_input_reference(content: &str, position: &Position) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    
    if position.line as usize >= lines.len() {
        return None;
    }
    
    let line = lines[position.line as usize];
    let char_pos = position.character as usize;
    
    // Look for "input." pattern before cursor (txtx uses singular form only)
    let search_text = &line[..char_pos.min(line.len())];
    
    // Check for "input." (singular - the correct txtx syntax)
    if let Some(input_pos) = search_text.rfind("input.") {
        let start = input_pos + 6; // Length of "input."
        let rest = &line[start..];
        let var_name: String = rest.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        
        if !var_name.is_empty() {
            return Some(var_name);
        }
    }
    
    None
}

fn is_after_input_dot(content: &str, position: &Position) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    
    if position.line as usize >= lines.len() {
        return false;
    }
    
    let line = lines[position.line as usize];
    let char_pos = position.character as usize;
    
    // Check if we're right after "input." (singular - the correct txtx syntax)
    if char_pos >= 6 {
        let before = &line[char_pos.saturating_sub(6)..char_pos.min(line.len())];
        if before == "input." || before.ends_with("input.") {
            return true;
        }
    }
    
    false
}

fn find_variable_line(content: &str, var_name: &str) -> Option<u32> {
    for (i, line) in content.lines().enumerate() {
        if line.contains(&format!("{}:", var_name)) {
            return Some(i as u32);
        }
    }
    None
}