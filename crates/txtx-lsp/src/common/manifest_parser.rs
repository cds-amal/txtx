use lsp_types::{Position, Range};
use std::collections::HashMap;

/// Information about where an input is defined in the manifest
#[derive(Debug, Clone)]
pub struct InputDefinition {
    pub name: String,
    pub range: Range,
    pub environment: String,
}

/// Parse a manifest file (txtx.yml) and extract input definitions with their locations
pub fn parse_manifest_inputs(manifest_content: &str) -> HashMap<String, Vec<InputDefinition>> {
    let mut inputs = HashMap::new();
    let lines: Vec<&str> = manifest_content.lines().collect();
    
    let mut current_environment: Option<String> = None;
    let mut in_environments = false;
    let mut indent_level = 0;
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        // Count leading spaces for indentation
        let current_indent = line.len() - line.trim_start().len();
        
        // Check if we're entering the environments section
        if trimmed == "environments:" {
            in_environments = true;
            indent_level = current_indent;
            continue;
        }
        
        if !in_environments {
            continue;
        }
        
        // Check if we've exited the environments section
        if current_indent <= indent_level && !trimmed.ends_with(':') {
            in_environments = false;
            continue;
        }
        
        // Check for environment name (one level deeper than "environments:")
        if current_indent == indent_level + 2 && trimmed.ends_with(':') {
            current_environment = Some(trimmed.trim_end_matches(':').to_string());
            continue;
        }
        
        // Check for input definitions (two levels deeper than "environments:")
        if current_indent == indent_level + 4 && current_environment.is_some() {
            // Parse input definition line (format: "input_name: value")
            if let Some(colon_pos) = trimmed.find(':') {
                let input_name = trimmed[..colon_pos].trim();
                
                // Calculate the actual character positions in the original line
                let name_start = line.find(input_name).unwrap_or(current_indent);
                let name_end = name_start + input_name.len();
                
                let definition = InputDefinition {
                    name: input_name.to_string(),
                    range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: name_start as u32,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: name_end as u32,
                        },
                    },
                    environment: current_environment.as_ref().unwrap().clone(),
                };
                
                inputs.entry(input_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(definition);
            }
        }
    }
    
    inputs
}

/// Find the definition location for a specific input name
pub fn find_input_definition(
    manifest_content: &str,
    input_name: &str,
    preferred_environment: Option<&str>,
) -> Option<Range> {
    let inputs = parse_manifest_inputs(manifest_content);
    
    if let Some(definitions) = inputs.get(input_name) {
        // If we have a preferred environment, try to find it
        if let Some(env) = preferred_environment {
            for def in definitions {
                if def.environment == env {
                    return Some(def.range);
                }
            }
        }
        
        // Otherwise, prefer "default" environment
        for def in definitions {
            if def.environment == "default" {
                return Some(def.range);
            }
        }
        
        // If no default, return the first one
        definitions.first().map(|d| d.range)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_manifest_inputs() {
        let manifest = r#"name: test-workspace
id: test-workspace

runbooks:
  - name: deploy
    location: deploy.tx

environments:
  default:
    contract_address: "0x1234"
    private_key: "key1"
  testnet:
    contract_address: "0x5678"
    api_url: "https://test.com"
"#;

        let inputs = parse_manifest_inputs(manifest);
        
        assert!(inputs.contains_key("contract_address"));
        assert!(inputs.contains_key("private_key"));
        assert!(inputs.contains_key("api_url"));
        
        let contract_defs = &inputs["contract_address"];
        assert_eq!(contract_defs.len(), 2);
        
        // Check that we found both environments
        let envs: Vec<String> = contract_defs.iter().map(|d| d.environment.clone()).collect();
        assert!(envs.contains(&"default".to_string()));
        assert!(envs.contains(&"testnet".to_string()));
    }
    
    #[test]
    fn test_find_input_definition() {
        let manifest = r#"name: test
environments:
  default:
    my_input: "value1"
  prod:
    my_input: "value2"
"#;

        // Should prefer default environment
        let range = find_input_definition(manifest, "my_input", None);
        assert!(range.is_some());
        
        let range = range.unwrap();
        assert_eq!(range.start.line, 3); // Line 3 (0-indexed) is where default's my_input is
        
        // Should find specific environment when requested
        let range = find_input_definition(manifest, "my_input", Some("prod"));
        assert!(range.is_some());
        
        let range = range.unwrap();
        assert_eq!(range.start.line, 5); // Line 5 is where prod's my_input is
    }
}