// Simple test to verify hover functionality
use txtx_lsp::common::hover::generate_hover_info;
use lsp_types::Position;

fn main() {
    let test_content = r#"addon "evm" {
    rpc_api_url = "https://eth.example.com"
}

// Test hover on this function
variable "contract" {
    value = evm::get_contract_from_foundry_project("MyContract")
}

action "deploy" "evm::deploy_contract" {
    contract = variable.contract
}"#;

    // Test hovering over the function name
    // Line 5 (0-indexed), around position 16 where "evm::get_contract_from_foundry_project" starts
    let position = Position { line: 5, character: 16 };
    
    if let Some(hover) = generate_hover_info(test_content, &position) {
        println!("Hover info found!");
        if let lsp_types::HoverContents::Markup(markup) = hover.contents {
            println!("Content:\n{}", markup.value);
        }
    } else {
        println!("No hover info found at position {:?}", position);
        
        // Try different positions to debug
        for char_pos in 10..50 {
            let test_pos = Position { line: 5, character: char_pos };
            if generate_hover_info(test_content, &test_pos).is_some() {
                println!("Found hover at character position {}", char_pos);
                break;
            }
        }
    }
}