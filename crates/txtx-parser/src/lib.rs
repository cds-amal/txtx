//! Tree-sitter based parser for txtx runbook language

use std::collections::HashMap;
use tree_sitter::{Node, Parser};

pub mod ast;
pub mod builder;
pub mod renderer;
pub mod transform;
pub mod visitor;

pub use ast::*;
pub use builder::RunbookBuilder;
pub use renderer::RunbookRenderer;
pub use transform::RunbookTransform;
pub use visitor::RunbookVisitor;

use tree_sitter_txtx;

/// Source location information for AST nodes
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// A source map that tracks locations of AST nodes
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// Maps construct type and name to its source location
    /// e.g., ("action", "send_eth") -> SourceLocation
    pub construct_locations: HashMap<(String, String), SourceLocation>,
    /// Maps expression paths to their locations
    /// e.g., "action.send_eth.tx_hash" -> SourceLocation
    pub expression_locations: HashMap<String, SourceLocation>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            construct_locations: HashMap::new(),
            expression_locations: HashMap::new(),
        }
    }
}

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

/// Parse a txtx runbook from source text with source location tracking
pub fn parse_with_locations(source: &str) -> Result<(Runbook, SourceMap), ParseError> {
    let mut parser = Parser::new();

    // Set the language
    parser.set_language(tree_sitter_txtx::language()).map_err(|_| ParseError::LanguageError)?;

    // Parse the source
    let tree = parser.parse(source, None).ok_or(ParseError::ParseFailed)?;

    let root = tree.root_node();

    // Convert tree-sitter AST to our AST with location tracking
    convert_node_with_locations(&root, source)
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

fn node_to_location(node: &Node) -> SourceLocation {
    let start = node.start_position();
    let end = node.end_position();
    SourceLocation {
        start_line: start.row + 1,  // Tree-sitter uses 0-based line numbers
        start_column: start.column + 1,  // Convert to 1-based for display
        end_line: end.row + 1,
        end_column: end.column + 1,
    }
}

fn convert_node_with_locations(node: &Node, source: &str) -> Result<(Runbook, SourceMap), ParseError> {
    let mut runbook = Runbook::new();
    let mut source_map = SourceMap::new();

    let cursor = &mut node.walk();
    for child in node.children(cursor) {
        match child.kind() {
            "addon_block" => {
                if let Ok(addon) = convert_addon(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("addon".to_string(), addon.network.clone()),
                        location,
                    );
                    runbook.addons.push(addon);
                }
            }
            "signer_block" => {
                if let Ok(signer) = convert_signer(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("signer".to_string(), signer.name.clone()),
                        location,
                    );
                    runbook.signers.push(signer);
                }
            }
            "action_block" => {
                if let Ok(action) = convert_action(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("action".to_string(), action.name.clone()),
                        location,
                    );
                    // Also track attribute locations
                    track_attribute_locations(&child, source, &format!("action.{}", action.name), &mut source_map);
                    runbook.actions.push(action);
                }
            }
            "output_block" => {
                if let Ok(output) = convert_output(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("output".to_string(), output.name.clone()),
                        location,
                    );
                    // Also track attribute locations
                    track_attribute_locations(&child, source, &format!("output.{}", output.name), &mut source_map);
                    runbook.outputs.push(output);
                }
            }
            "variable_declaration" => {
                if let Ok(var) = convert_variable(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("variable".to_string(), var.name.clone()),
                        location,
                    );
                    runbook.variables.push(var);
                }
            }
            "flow_block" => {
                if let Ok(flow) = convert_flow(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("flow".to_string(), flow.name.clone()),
                        location,
                    );
                    runbook.flows.push(flow);
                }
            }
            "module_block" => {
                if let Ok(module) = convert_module(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("module".to_string(), module.name.clone()),
                        location,
                    );
                    runbook.modules.push(module);
                }
            }
            "runbook_block" => {
                if let Ok(runbook_block) = convert_runbook_block(&child, source) {
                    let location = node_to_location(&child);
                    source_map.construct_locations.insert(
                        ("runbook".to_string(), runbook_block.name.clone()),
                        location,
                    );
                    runbook.runbook_blocks.push(runbook_block);
                }
            }
            _ => {} // Ignore other nodes like comments
        }
    }

    Ok((runbook, source_map))
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
    let source_location = Some(ast::SourceLocation {
        line: node.start_position().row,
        column: node.start_position().column,
    });

    Ok(SignerBlock { name, signer_type, attributes, source_location })
}

fn convert_action(node: &Node, source: &str) -> Result<ActionBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let action_type = get_string_field(node, "type", source)?;
    let attributes = convert_attributes(node, source)?;
    let source_location = Some(ast::SourceLocation {
        line: node.start_position().row,
        column: node.start_position().column,
    });

    Ok(ActionBlock { name, action_type, attributes, source_location })
}

fn convert_output(node: &Node, source: &str) -> Result<OutputBlock, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;
    let source_location = Some(ast::SourceLocation {
        line: node.start_position().row,
        column: node.start_position().column,
    });

    Ok(OutputBlock { name, attributes, source_location })
}

fn convert_variable(node: &Node, source: &str) -> Result<VariableDeclaration, ParseError> {
    let name = get_string_field(node, "name", source)?;
    let attributes = convert_attributes(node, source)?;
    let source_location = Some(ast::SourceLocation {
        line: node.start_position().row,
        column: node.start_position().column,
    });

    Ok(VariableDeclaration { name, attributes, source_location })
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

/// Track locations of attributes and expressions within a block
fn track_attribute_locations(node: &Node, source: &str, prefix: &str, source_map: &mut SourceMap) {
    // Find the config block child node
    let cursor = &mut node.walk();
    for child in node.children(cursor) {
        if child.kind() == "block" {
            track_block_attribute_locations(&child, source, prefix, source_map);
        }
    }
}

fn track_block_attribute_locations(block_node: &Node, source: &str, prefix: &str, source_map: &mut SourceMap) {
    let cursor = &mut block_node.walk();
    for child in block_node.children(cursor) {
        if child.kind() == "attribute" {
            // Get attribute name and value nodes
            let mut attr_cursor = child.walk();
            let mut key_node = None;
            let mut value_node = None;
            
            for attr_child in child.children(&mut attr_cursor) {
                match attr_child.kind() {
                    "identifier" if key_node.is_none() => key_node = Some(attr_child),
                    _ if attr_child.kind().contains("expression") || attr_child.kind() == "reference" => {
                        value_node = Some(attr_child);
                    }
                    _ => {}
                }
            }
            
            if let (Some(key), Some(value)) = (key_node, value_node) {
                let key_text = &source[key.byte_range()];
                let expression_path = format!("{}.{}", prefix, key_text);
                let location = node_to_location(&value);
                source_map.expression_locations.insert(expression_path, location);
                
                // If it's a reference, also track the reference itself
                if value.kind() == "reference" {
                    track_reference_location(&value, source, source_map);
                }
            }
        }
    }
}

fn track_reference_location(ref_node: &Node, source: &str, source_map: &mut SourceMap) {
    let ref_text = &source[ref_node.byte_range()];
    let location = node_to_location(ref_node);
    source_map.expression_locations.insert(ref_text.to_string(), location);
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
