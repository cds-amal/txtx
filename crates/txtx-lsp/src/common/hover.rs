use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position};
use txtx_addon_kit::types::functions::FunctionSpecification;
use txtx_addon_kit::types::commands::CommandSpecification;
use crate::common::specifications::{get_function_spec, get_action_spec};

/// Extract the identifier at the given position in the text
fn get_identifier_at_position(text: &str, position: &Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;
    
    #[cfg(debug_assertions)]
    eprintln!("LSP Hover: Line {}: '{}'", position.line, line);
    #[cfg(debug_assertions)]
    eprintln!("LSP Hover: Character position: {}", position.character);
    
    // Find word boundaries around the cursor position
    let mut start = position.character as usize;
    let mut end = position.character as usize;
    
    let chars: Vec<char> = line.chars().collect();
    
    // Ensure we're within bounds
    if position.character as usize >= chars.len() {
        #[cfg(debug_assertions)]
        eprintln!("LSP Hover: Character position {} is beyond line length {}", position.character, chars.len());
        return None;
    }
    
    // Find start of identifier (including namespace::function format)
    // We need to handle the :: as a unit, not individual characters
    while start > 0 {
        let prev_char = chars.get(start - 1)?;
        if prev_char.is_alphanumeric() || *prev_char == '_' {
            start -= 1;
        } else if start >= 2 && *prev_char == ':' && *chars.get(start - 2)? == ':' {
            // Skip over :: as a unit
            start -= 2;
        } else {
            break;
        }
    }
    
    // Find end of identifier
    while end < chars.len() {
        let curr_char = chars.get(end)?;
        if curr_char.is_alphanumeric() || *curr_char == '_' {
            end += 1;
        } else if end + 1 < chars.len() && *curr_char == ':' && *chars.get(end + 1)? == ':' {
            // Skip over :: as a unit
            end += 2;
        } else {
            break;
        }
    }
    
    if start >= end {
        #[cfg(debug_assertions)]
        eprintln!("LSP Hover: No identifier found (start={}, end={})", start, end);
        return None;
    }
    
    let identifier: String = chars[start..end].iter().collect();
    #[cfg(debug_assertions)]
    eprintln!("LSP Hover: Extracted identifier '{}' from positions {}..{}", identifier, start, end);
    Some(identifier)
}

/// Identify the type of construct at the position (e.g., function call, action, variable reference)
fn identify_construct_type(text: &str, identifier: &str, position: &Position) -> ConstructType {
    // Check if it's a namespace::function or namespace::action pattern
    if identifier.contains("::") {
        // Look at context to determine if it's a function or action
        let lines: Vec<&str> = text.lines().collect();
        if let Some(line) = lines.get(position.line as usize) {
            // Check if we're in an action block declaration
            if line.contains("action") && line.contains(&format!("\"{}\"", identifier)) {
                return ConstructType::ActionDeclaration(identifier.to_string());
            }
            // Otherwise it's likely a function call
            return ConstructType::FunctionCall(identifier.to_string());
        }
    }
    
    // Check for variable references
    if identifier.starts_with("variable.") || identifier.starts_with("input.") 
        || identifier.starts_with("action.") || identifier.starts_with("signer.") {
        return ConstructType::VariableReference(identifier.to_string());
    }
    
    ConstructType::Unknown
}

#[derive(Debug)]
enum ConstructType {
    FunctionCall(String),
    ActionDeclaration(String),
    VariableReference(String),
    Unknown,
}

/// Generate hover content for a function specification
fn function_spec_to_hover(spec: &FunctionSpecification) -> Hover {
    let mut content = String::new();
    
    // Function signature
    content.push_str(&format!("### `{}`\n\n", spec.name));
    
    // Documentation
    content.push_str(&spec.documentation);
    content.push_str("\n\n");
    
    // Parameters
    if !spec.inputs.is_empty() {
        content.push_str("**Parameters:**\n");
        for input in &spec.inputs {
            let optional = if input.optional { " (optional)" } else { "" };
            content.push_str(&format!("- `{}`: {}{}\n", 
                input.name, 
                input.documentation, 
                optional
            ));
        }
        content.push_str("\n");
    }
    
    // Return type
    content.push_str(&format!("**Returns:** {}\n\n", spec.output.documentation));
    
    // Example
    if !spec.example.is_empty() {
        content.push_str("**Example:**\n```hcl\n");
        content.push_str(&spec.example);
        content.push_str("\n```\n");
    }
    
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    }
}

/// Generate hover content for an action specification
fn action_spec_to_hover(spec: &CommandSpecification) -> Hover {
    let mut content = String::new();
    
    // Action name
    content.push_str(&format!("### Action: `{}`\n\n", spec.matcher));
    
    // Documentation
    content.push_str(&spec.documentation);
    content.push_str("\n\n");
    
    // Inputs
    if !spec.inputs.is_empty() {
        content.push_str("**Inputs:**\n");
        for input in &spec.inputs {
            let optional = if input.optional { " (optional)" } else { "" };
            content.push_str(&format!("- `{}`: {}{}\n", 
                input.name, 
                input.documentation, 
                optional
            ));
        }
        content.push_str("\n");
    }
    
    // Outputs
    if !spec.outputs.is_empty() {
        content.push_str("**Outputs:**\n");
        for output in &spec.outputs {
            content.push_str(&format!("- `{}`: {}\n", 
                output.name, 
                output.documentation
            ));
        }
        content.push_str("\n");
    }
    
    // Example
    if !spec.example.is_empty() {
        content.push_str("**Example:**\n```hcl\n");
        content.push_str(&spec.example);
        content.push_str("\n```\n");
    }
    
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    }
}

/// Generate hover information for the position in the document
pub fn generate_hover_info(text: &str, position: &Position) -> Option<Hover> {
    // Extract the identifier at the cursor position
    let identifier = get_identifier_at_position(text, position)?;
    
    #[cfg(debug_assertions)]
    eprintln!("LSP Hover: Found identifier '{}' at position {:?}", identifier, position);
    
    // Identify what type of construct it is
    let construct_type = identify_construct_type(text, &identifier, position);
    
    #[cfg(debug_assertions)]
    eprintln!("LSP Hover: Construct type: {:?}", construct_type);
    
    match construct_type {
        ConstructType::FunctionCall(name) => {
            // Look up function specification
            if let Some(spec) = get_function_spec(&name) {
                #[cfg(debug_assertions)]
                eprintln!("LSP Hover: Found spec for function '{}'", name);
                return Some(function_spec_to_hover(&spec));
            } else {
                #[cfg(debug_assertions)]
                eprintln!("LSP Hover: No spec found for function '{}'", name);
            }
        }
        ConstructType::ActionDeclaration(name) => {
            // Look up action specification
            if let Some(spec) = get_action_spec(&name) {
                return Some(action_spec_to_hover(&spec));
            }
        }
        ConstructType::VariableReference(name) => {
            // For now, just show the variable name
            // In the future, we could look up the variable's type and value
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("**Variable Reference:** `{}`", name),
                }),
                range: None,
            });
        }
        ConstructType::Unknown => {}
    }
    
    None
}