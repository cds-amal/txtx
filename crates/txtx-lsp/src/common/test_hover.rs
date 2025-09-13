#[cfg(test)]
mod tests {
    use super::super::hover::generate_hover_info;
    use super::super::specifications::{get_function_spec, get_function_specifications};
    use lsp_types::Position;

    #[test]
    fn test_hover_on_evm_function() {
        // First check what functions are available
        let all_specs = get_function_specifications();
        println!("Available namespaces: {:?}", all_specs.keys().collect::<Vec<_>>());
        
        if let Some(evm_functions) = all_specs.get("evm") {
            println!("EVM functions ({}):", evm_functions.len());
            for func in evm_functions {
                println!("  - {}", func.name);
            }
        }
        
        // Test looking up a specific function
        let test_cases = vec![
            "evm::get_contract_from_foundry_project",
            "evm::deploy_contract",
            "evm::send_eth",
        ];
        
        for name in test_cases {
            if let Some(spec) = get_function_spec(name) {
                println!("✓ Found spec for {}", name);
            } else {
                println!("✗ No spec for {}", name);
            }
        }
        
        // Test hover generation
        let test_content = r#"addon "evm" {
    rpc_api_url = "https://eth.example.com"
}

variable "contract" {
    value = evm::get_contract_from_foundry_project("MyContract")
}"#;

        // Test hovering at different positions on line 5
        let line = 5;
        let test_positions = vec![12, 15, 20, 26, 30, 35, 40];
        for char_pos in test_positions {
            let position = Position { line, character: char_pos };
            if let Some(hover) = generate_hover_info(test_content, &position) {
                println!("✓ Found hover at line {}, char {}", line, char_pos);
                if let lsp_types::HoverContents::Markup(markup) = hover.contents {
                    println!("  First line of content: {}", markup.value.lines().next().unwrap_or(""));
                }
            } else {
                println!("✗ No hover at line {}, char {}", line, char_pos);
            }
        }
        
        // Test the exact scenario from the bug report
        let bug_content = r#"// Contract ABIs
variable "din_registry_coordinator_contract" {
    description = "The DINRegistryCoordinator contract and its metadata"
    value = evm::get_contract_from_foundry_project("DINRegistryCoordinator")
}"#;
        
        // Line 3 (0-based) is where the function call is, character 26
        let bug_position = Position { line: 3, character: 26 };
        println!("\nTesting bug scenario at position {:?}", bug_position);
        
        if let Some(hover) = generate_hover_info(bug_content, &bug_position) {
            println!("✓ Bug fixed! Found hover for evm::get_contract_from_foundry_project");
            if let lsp_types::HoverContents::Markup(markup) = hover.contents {
                println!("  Content starts with: {}", markup.value.lines().next().unwrap_or(""));
            }
        } else {
            println!("✗ Bug still present - no hover found at position");
        }
    }
}