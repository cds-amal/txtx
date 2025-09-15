//! Enhanced go-to-definition handler with multi-file support
//!
//! This handler supports:
//! - input references to manifest environments
//! - flow references to flows.tx
//! - var references within the same file
//! - action references within the same file

use crate::cli::lsp::multi_file::{get_runbook_name_for_file, load_multi_file_runbook};
use crate::cli::lsp::workspace::SharedWorkspaceState;
use lsp_types::*;
use regex::Regex;
use std::path::PathBuf;

pub struct EnhancedDefinitionHandler {
    workspace: SharedWorkspaceState,
}

impl EnhancedDefinitionHandler {
    pub fn new(workspace: SharedWorkspaceState) -> Self {
        Self { workspace }
    }

    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        eprintln!("[Definition] Request for {:?} at {}:{}", uri, position.line, position.character);

        let workspace = self.workspace.read();
        let document = workspace.get_document(uri)?;
        let content = document.content();

        // Extract the reference at cursor position
        let reference = extract_reference_at_position(content, &position)?;
        eprintln!("[Definition] Found reference: {:?}", reference);

        match reference {
            Reference::Input(var_name) => {
                // Look for input in manifest environments
                if let Some(manifest) = workspace.get_manifest_for_document(uri) {
                    if let Some(location) = find_input_in_manifest(&manifest.uri, &var_name) {
                        eprintln!("[Definition] Found input '{}' in manifest", var_name);
                        return Some(GotoDefinitionResponse::Scalar(location));
                    }
                }
            }
            Reference::Flow(flow_name) => {
                // Look for flow definition in flows.tx
                if let Some(location) = find_flow_definition(uri, &flow_name) {
                    eprintln!("[Definition] Found flow '{}' definition", flow_name);
                    return Some(GotoDefinitionResponse::Scalar(location));
                }
            }
            Reference::Variable(var_name) => {
                // Look for variable definition in current file
                if let Some(location) = find_variable_definition(uri, content, &var_name) {
                    eprintln!("[Definition] Found variable '{}' definition", var_name);
                    return Some(GotoDefinitionResponse::Scalar(location));
                }
            }
            Reference::Action(action_name) => {
                // Look for action definition in current file
                if let Some(location) = find_action_definition(uri, content, &action_name) {
                    eprintln!("[Definition] Found action '{}' definition", action_name);
                    return Some(GotoDefinitionResponse::Scalar(location));
                }
            }
        }

        eprintln!("[Definition] No definition found");
        None
    }
}

#[derive(Debug)]
enum Reference {
    Input(String),    // input.name
    Flow(String),     // flow.name
    Variable(String), // var.name
    Action(String),   // action.name
}

fn extract_reference_at_position(content: &str, position: &Position) -> Option<Reference> {
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Check various reference patterns
    let patterns = [
        (r"input\.(\w+)", |name: &str| Reference::Input(name.to_string())),
        (r"flow\.(\w+)", |name: &str| Reference::Flow(name.to_string())),
        (r"var\.(\w+)", |name: &str| Reference::Variable(name.to_string())),
        (r"action\.(\w+)", |name: &str| Reference::Action(name.to_string())),
    ];

    for (pattern, constructor) in patterns {
        let re = Regex::new(pattern).ok()?;

        for capture in re.captures_iter(line) {
            if let Some(name_match) = capture.get(1) {
                let full_match = capture.get(0)?;
                let start = full_match.start() as u32;
                let end = full_match.end() as u32;

                if position.character >= start && position.character <= end {
                    return Some(constructor(name_match.as_str()));
                }
            }
        }
    }

    None
}

fn find_input_in_manifest(manifest_uri: &Url, var_name: &str) -> Option<Location> {
    if let Ok(content) = std::fs::read_to_string(manifest_uri.path()) {
        for (line_num, line) in content.lines().enumerate() {
            // Look for the variable in environments section
            if line.trim_start().starts_with(&format!("{}:", var_name)) {
                return Some(Location {
                    uri: manifest_uri.clone(),
                    range: Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                });
            }
        }
    }
    None
}

fn find_flow_definition(current_uri: &Url, flow_name: &str) -> Option<Location> {
    // Construct path to flows.tx in the same directory
    let current_path = PathBuf::from(current_uri.path());
    if let Some(dir) = current_path.parent() {
        let flows_path = dir.join("flows.tx");

        if flows_path.exists() {
            if let Ok(flows_uri) = Url::from_file_path(&flows_path) {
                if let Ok(content) = std::fs::read_to_string(&flows_path) {
                    // Look for flow definition
                    let pattern = format!(r#"flow\s+"{}"\s*\{{"#, flow_name);
                    if let Ok(re) = Regex::new(&pattern) {
                        for (line_num, line) in content.lines().enumerate() {
                            if re.is_match(line) {
                                return Some(Location {
                                    uri: flows_uri,
                                    range: Range {
                                        start: Position { line: line_num as u32, character: 0 },
                                        end: Position {
                                            line: line_num as u32,
                                            character: line.len() as u32,
                                        },
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn find_variable_definition(uri: &Url, content: &str, var_name: &str) -> Option<Location> {
    // Look for variable definition pattern
    let pattern = format!(r#"variable\s+"{}"\s*\{{"#, var_name);
    if let Ok(re) = Regex::new(&pattern) {
        for (line_num, line) in content.lines().enumerate() {
            if re.is_match(line) {
                return Some(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                });
            }
        }
    }
    None
}

fn find_action_definition(uri: &Url, content: &str, action_name: &str) -> Option<Location> {
    // Look for action definition pattern
    let pattern = format!(r#"action\s+"{}"\s+"[^"]+"\s*\{{"#, action_name);
    if let Ok(re) = Regex::new(&pattern) {
        for (line_num, line) in content.lines().enumerate() {
            if re.is_match(line) {
                return Some(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                });
            }
        }
    }
    None
}
