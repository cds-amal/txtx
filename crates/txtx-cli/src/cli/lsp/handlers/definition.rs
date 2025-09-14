//! Go-to-definition handler

use lsp_types::*;
use crate::cli::lsp::workspace::SharedWorkspaceState;
use super::{Handler, TextDocumentHandler};

pub struct DefinitionHandler {
    workspace: SharedWorkspaceState,
}

impl DefinitionHandler {
    pub fn new(workspace: SharedWorkspaceState) -> Self {
        Self { workspace }
    }
    
    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let (uri, content, position) = self.get_document_at_position(&params.text_document_position_params)?;
        
        // Extract what's at the cursor position
        if let Some(var_ref) = extract_input_reference(&content, &position) {
            let workspace = self.workspace.read();
            // Find the manifest for this runbook
            let manifest = workspace.get_manifest_for_runbook(&uri)?;
            
            // Look for variable definition in manifest
            if let Some(line) = find_variable_line(&manifest.uri, &var_ref) {
                return Some(GotoDefinitionResponse::Scalar(Location {
                    uri: manifest.uri.clone(),
                    range: Range {
                        start: Position { line, character: 0 },
                        end: Position { line, character: 100 },
                    },
                }));
            }
        }
        
        None
    }
}

impl Handler for DefinitionHandler {
    fn workspace(&self) -> &SharedWorkspaceState {
        &self.workspace
    }
}

impl TextDocumentHandler for DefinitionHandler {}

fn extract_input_reference(content: &str, position: &Position) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;
    
    // Look for input.variable_name pattern
    let re = regex::Regex::new(r"input\.(\w+)").ok()?;
    
    for capture in re.captures_iter(line) {
        if let Some(var_match) = capture.get(1) {
            let full_match = capture.get(0)?;
            let start = full_match.start() as u32;
            let end = full_match.end() as u32;
            
            if position.character >= start && position.character <= end {
                return Some(var_match.as_str().to_string());
            }
        }
    }
    
    None
}

fn find_variable_line(manifest_uri: &Url, var_name: &str) -> Option<u32> {
    // This is a simplified version - in a real implementation,
    // you'd parse the manifest and track line numbers
    if let Ok(content) = std::fs::read_to_string(manifest_uri.path()) {
        for (line_num, line) in content.lines().enumerate() {
            if line.contains(var_name) && line.contains("=") {
                return Some(line_num as u32);
            }
        }
    }
    None
}