use lsp_types::Position;

/// Represents a reference to an input variable in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct InputReference {
    pub name: String,
    pub start_pos: Position,
    pub end_pos: Position,
}

/// Parse the text at a given position to identify if it's an input reference
/// Returns Some(InputReference) if the position is within an input reference like "inputs.variable_name"
pub fn parse_input_reference(source: &str, position: &Position) -> Option<InputReference> {
    let lines: Vec<&str> = source.lines().collect();
    
    // Check if position is valid
    if position.line as usize >= lines.len() {
        return None;
    }
    
    let line = lines[position.line as usize];
    let char_pos = position.character as usize;
    
    // Check if character position is valid
    if char_pos > line.len() {
        return None;
    }
    
    // Look for "inputs." pattern before the cursor position
    // We need to find the start of the identifier
    let mut start = char_pos;
    
    // Move backwards to find the start of the identifier
    while start > 0 {
        let ch = line.chars().nth(start - 1)?;
        if !ch.is_alphanumeric() && ch != '_' && ch != '.' {
            break;
        }
        start -= 1;
    }
    
    // Move forward to find the end of the identifier
    let mut end = char_pos;
    while end < line.len() {
        let ch = line.chars().nth(end)?;
        if !ch.is_alphanumeric() && ch != '_' && ch != '.' {
            break;
        }
        end += 1;
    }
    
    // Extract the full identifier
    let identifier = &line[start..end];
    
    // Check if it's an input reference (starts with "inputs.")
    if !identifier.starts_with("inputs.") {
        return None;
    }
    
    // Extract the input name (everything after "inputs.")
    let input_name = identifier.strip_prefix("inputs.")?;
    
    // Don't return empty names
    if input_name.is_empty() {
        return None;
    }
    
    Some(InputReference {
        name: input_name.to_string(),
        start_pos: Position {
            line: position.line,
            character: (start + 7) as u32, // Start after "inputs."
        },
        end_pos: Position {
            line: position.line,
            character: end as u32,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_reference() {
        let source = r#"action "deploy" "evm::deploy_contract" {
  address = inputs.contract_address
  key = inputs.private_key
}"#;

        // Test position within "contract_address"
        let pos = Position { line: 1, character: 25 };
        let result = parse_input_reference(source, &pos);
        assert!(result.is_some());
        
        let input_ref = result.unwrap();
        assert_eq!(input_ref.name, "contract_address");
        
        // Test position within "private_key"
        let pos = Position { line: 2, character: 18 };
        let result = parse_input_reference(source, &pos);
        assert!(result.is_some());
        
        let input_ref = result.unwrap();
        assert_eq!(input_ref.name, "private_key");
        
        // Test position not on an input reference
        let pos = Position { line: 0, character: 10 };
        let result = parse_input_reference(source, &pos);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_input_at_boundary() {
        let source = "value = inputs.test_var";
        
        // At the start of "inputs"
        let pos = Position { line: 0, character: 8 };
        let result = parse_input_reference(source, &pos);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "test_var");
        
        // At the dot
        let pos = Position { line: 0, character: 14 };
        let result = parse_input_reference(source, &pos);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "test_var");
        
        // At the end
        let pos = Position { line: 0, character: 23 };
        let result = parse_input_reference(source, &pos);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "test_var");
    }
}