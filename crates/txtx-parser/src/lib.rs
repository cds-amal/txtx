//! Tree-sitter based parser for txtx runbook language

use std::collections::HashMap;
use tree_sitter::{Node, Parser};

pub mod ast;
pub mod builder;
pub mod renderer;
pub mod transform;
pub mod visitor;

pub use ast::*;
use ast::{FlowBlock, ModuleBlock, RunbookBlock};
pub use builder::RunbookBuilder;
pub use renderer::RunbookRenderer;
pub use transform::RunbookTransform;
pub use visitor::RunbookVisitor;

use tree_sitter_txtx;

/// Parse a txtx runbook from source text
pub fn parse(source: &str) -> Result<Runbook, ParseError> {
    let mut parser = Parser::new();

    // Set the language
    parser.set_language(tree_sitter_txtx::language()).map_err(|_| ParseError::LanguageError)?;

    // Parse the source
    let tree = parser.parse(source, None).ok_or(ParseError::ParseFailed)?;

    let root = tree.root_node();

    // Convert tree-sitter AST to our AST
    convert_node(&root, source)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Failed to set parser language")]
    LanguageError,

    #[error("Failed to parse source")]
    ParseFailed,

    #[error("Unexpected node type: {0}")]
    UnexpectedNode(String),

    #[error("Invalid syntax at line {line}, column {column}")]
    SyntaxError { line: usize, column: usize },
}

fn convert_node(node: &Node, source: &str) -> Result<Runbook, ParseError> {
    let mut runbook = Runbook::new();

    let cursor = &mut node.walk();
    for child in node.children(cursor) {
        match child.kind() {
            "addon_block" => {
                if let Ok(addon) = convert_addon(&child, source) {
                    runbook.addons.push(addon);
                }
            }
            "signer_block" => {
                if let Ok(signer) = convert_signer(&child, source) {
                    runbook.signers.push(signer);
                }
            }
            "action_block" => {
                if let Ok(action) = convert_action(&child, source) {
                    runbook.actions.push(action);
                }
            }
            "output_block" => {
                if let Ok(output) = convert_output(&child, source) {
                    runbook.outputs.push(output);
                }
            }
            "variable_declaration" => {
                if let Ok(var) = convert_variable(&child, source) {
                    runbook.variables.push(var);
                }
            }
            "flow_block" => {
                if let Ok(flow) = convert_flow(&child, source) {
                    runbook.flows.push(flow);
                }
            }
            "module_block" => {
                if let Ok(module) = convert_module(&child, source) {
                    runbook.modules.push(module);
                }
            }
            "runbook_block" => {
                if let Ok(runbook_block) = convert_runbook_block(&child, source) {
                    runbook.runbook_blocks.push(runbook_block);
                }
            }
            _ => {} // Ignore other nodes like comments
        }
    }

    Ok(runbook)
}

fn convert_addon(node: &Node, source: &str) -> Result<AddonBlock, ParseError> {
    let network = get_string_field(node, "network", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(AddonBlock { network, attributes })
}

fn convert_signer(node: &Node, source: &str) -> Result<SignerBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let signer_type = get_string_field(node, "type", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(SignerBlock { name, signer_type, attributes })
}

fn convert_action(node: &Node, source: &str) -> Result<ActionBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let action_type = get_string_field(node, "type", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(ActionBlock { name, action_type, attributes })
}

fn convert_output(node: &Node, source: &str) -> Result<OutputBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(OutputBlock { name, attributes })
}

fn convert_variable(node: &Node, source: &str) -> Result<VariableDeclaration, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(VariableDeclaration { name, attributes })
}

fn convert_flow(node: &Node, source: &str) -> Result<FlowBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(FlowBlock { name, attributes })
}

fn convert_module(node: &Node, source: &str) -> Result<ModuleBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(ModuleBlock { name, attributes })
}

fn convert_runbook_block(node: &Node, source: &str) -> Result<RunbookBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;

    Ok(RunbookBlock { name, attributes })
}

fn convert_attributes(
    node: &Node,
    source: &str,
) -> Result<HashMap<String, Expression>, ParseError> {
    let mut attributes = HashMap::new();

    // Find the block content - try both "config" and "content" field names
    let block = node.child_by_field_name("config").or_else(|| node.child_by_field_name("content"));

    if let Some(block) = block {
        let cursor = &mut block.walk();
        for child in block.children(cursor) {
            if child.kind() == "attribute" {
                if let Ok((key, value)) = convert_attribute(&child, source) {
                    attributes.insert(key, value);
                }
            }
        }
    }

    Ok(attributes)
}

fn convert_attribute(node: &Node, source: &str) -> Result<(String, Expression), ParseError> {
    // Get the key - it's an identifier, not a string
    let key_node = node
        .child_by_field_name("key")
        .ok_or_else(|| ParseError::UnexpectedNode("Missing attribute key".to_string()))?;
    let key = key_node.utf8_text(source.as_bytes()).unwrap().to_string();

    let value = if let Some(value_node) = node.child_by_field_name("value") {
        convert_expression(&value_node, source)?
    } else {
        Expression::Bool(true) // Default for flag attributes
    };

    Ok((key, value))
}

fn convert_expression(node: &Node, source: &str) -> Result<Expression, ParseError> {
    match node.kind() {
        "string" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            // Remove quotes
            let unquoted = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
                &text[1..text.len() - 1]
            } else {
                text
            };
            Ok(Expression::String(unquoted.to_string()))
        }
        "number" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            let num = text.parse::<f64>().map_err(|_| ParseError::SyntaxError {
                line: node.start_position().row,
                column: node.start_position().column,
            })?;
            Ok(Expression::Number(num))
        }
        "boolean" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            Ok(Expression::Bool(text == "true"))
        }
        "reference" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            let parts: Vec<String> = text.split('.').map(|s| s.to_string()).collect();
            Ok(Expression::Reference(parts))
        }
        "array" => {
            let mut items = Vec::new();
            let cursor = &mut node.walk();
            for child in node.children(cursor) {
                if child.kind() != "[" && child.kind() != "]" && child.kind() != "," {
                    if let Ok(expr) = convert_expression(&child, source) {
                        items.push(expr);
                    }
                }
            }
            Ok(Expression::Array(items))
        }
        "object" => {
            let mut fields = HashMap::new();
            let cursor = &mut node.walk();
            for child in node.children(cursor) {
                if child.kind() == "field" {
                    if let Ok((key, value)) = convert_field(&child, source) {
                        fields.insert(key, value);
                    }
                }
            }
            Ok(Expression::Object(fields))
        }
        "function_call" => {
            let name = get_string_field(node, "name", source)?;
            let mut args = Vec::new();

            if let Some(args_node) = node.child_by_field_name("arguments") {
                let cursor = &mut args_node.walk();
                for child in args_node.children(cursor) {
                    if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                        if let Ok(expr) = convert_expression(&child, source) {
                            args.push(expr);
                        }
                    }
                }
            }

            Ok(Expression::FunctionCall { name, args })
        }
        _ => Err(ParseError::UnexpectedNode(node.kind().to_string())),
    }
}

fn convert_field(node: &Node, source: &str) -> Result<(String, Expression), ParseError> {
    let key = get_string_field(node, "key", source)?;
    let value = if let Some(value_node) = node.child_by_field_name("value") {
        convert_expression(&value_node, source)?
    } else {
        Expression::Bool(true)
    };

    Ok((key, value))
}

fn get_string_field(node: &Node, field: &str, source: &str) -> Result<String, ParseError> {
    let field_node = node
        .child_by_field_name(field)
        .ok_or_else(|| ParseError::UnexpectedNode(format!("Missing field: {}", field)))?;

    // If it's a string node, unwrap the quotes
    if field_node.kind() == "string" {
        let text = field_node.utf8_text(source.as_bytes()).unwrap();
        // Remove quotes
        let unquoted = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
            &text[1..text.len() - 1]
        } else {
            text
        };
        Ok(unquoted.to_string())
    } else {
        // Otherwise just return the text
        Ok(field_node.utf8_text(source.as_bytes()).unwrap().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_runbook() {
        let source = r#"
addon "evm" {
    chain_id = 1
    rpc_api_url = "https://mainnet.infura.io"
}

signer "deployer" "evm::secret_key" {
    secret_key = input.private_key
}

action "deploy" "evm::deploy_contract" {
    signer = signer.deployer
    contract = "MyContract"
}

output "address" {
    value = action.deploy.contract_address
}
"#;

        let result = parse(source);
        assert!(result.is_ok());

        let runbook = result.unwrap();
        assert_eq!(runbook.addons.len(), 1);
        assert_eq!(runbook.signers.len(), 1);
        assert_eq!(runbook.actions.len(), 1);
        assert_eq!(runbook.outputs.len(), 1);
    }
}
