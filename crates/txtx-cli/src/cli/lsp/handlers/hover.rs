//! Hover information handler
//!
//! Provides hover information for functions, actions, and input references

use super::{Handler, TextDocumentHandler};
use crate::cli::lsp::{
    functions::{get_action_hover, get_function_hover},
    workspace::SharedWorkspaceState,
};
use lsp_types::*;

pub struct HoverHandler {
    workspace: SharedWorkspaceState,
}

impl HoverHandler {
    pub fn new(workspace: SharedWorkspaceState) -> Self {
        Self { workspace }
    }

    /// Handle hover request
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let (uri, content, position) =
            self.get_document_at_position(&params.text_document_position_params)?;

        // Try to extract function/action reference
        if let Some(reference) = extract_function_or_action(&content, &position) {
            // Check if it's a function
            if let Some(hover_text) = get_function_hover(&reference) {
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_text,
                    }),
                    range: None,
                });
            }

            // Check if it's an action
            if let Some(hover_text) = get_action_hover(&reference) {
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_text,
                    }),
                    range: None,
                });
            }
        }

        // Try input reference hover
        if let Some(var_ref) = extract_input_reference(&content, &position) {
            let workspace = self.workspace.read();
            let manifest = workspace.get_manifest_for_runbook(&uri)?;

            // Look up in environment variables
            for (env_name, vars) in &manifest.environments {
                if let Some(value) = vars.get(&var_ref) {
                    let hover_text = format!(
                        "**Input**: `{}`\n\n**Value in `{}`**: `{}`",
                        var_ref, env_name, value
                    );
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: hover_text,
                        }),
                        range: None,
                    });
                }
            }
        }

        None
    }
}

impl Handler for HoverHandler {
    fn workspace(&self) -> &SharedWorkspaceState {
        &self.workspace
    }
}

impl TextDocumentHandler for HoverHandler {}

// Helper functions
fn extract_function_or_action(content: &str, position: &Position) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Simple heuristic: look for namespace::name pattern
    let re = regex::Regex::new(r"\b(\w+)::([\w_]+)\b").ok()?;

    for capture in re.captures_iter(line) {
        let full_match = capture.get(0)?;
        let start = full_match.start() as u32;
        let end = full_match.end() as u32;

        if position.character >= start && position.character <= end {
            return Some(full_match.as_str().to_string());
        }
    }

    None
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function_reference() {
        let content = "value = std::encode_hex(data)";
        let position = Position { line: 0, character: 15 };

        let result = extract_function_or_action(content, &position);
        assert_eq!(result, Some("std::encode_hex".to_string()));
    }

    #[test]
    fn test_extract_action_reference() {
        let content = "action \"deploy\" \"evm::deploy_contract\" {";
        let position = Position { line: 0, character: 20 };

        let result = extract_function_or_action(content, &position);
        assert_eq!(result, Some("evm::deploy_contract".to_string()));
    }

    #[test]
    fn test_extract_input_reference() {
        let content = "value = input.api_key";
        let position = Position { line: 0, character: 15 };

        let result = extract_input_reference(content, &position);
        assert_eq!(result, Some("api_key".to_string()));
    }
}
