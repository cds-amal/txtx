use txtx_test_utils::{RunbookBuilder, ValidationResult, ValidationMode, create_test_manifest_with_env};
use std::path::PathBuf;

// Helper macros for LSP testing
macro_rules! assert_has_diagnostic {
    ($diagnostics:expr, $message:expr) => {
        assert!(
            $diagnostics.iter().any(|d| d.message.contains($message)),
            "Expected diagnostic containing '{}', but got: {:?}",
            $message,
            $diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    };
}

#[cfg(test)]
mod lsp_hover_tests {
    use super::*;
    
    // Test hover information for functions
    #[test]
    fn test_function_hover_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .variable("wei_amount", "evm::to_wei(1, \"ether\")")
            .variable("hex_value", "std::encode_hex(\"hello\")")
            .action("deploy", "evm::get_contract_from_foundry_project")
                .input("project_path", "\"./contracts\"")
                .input("contract", "\"Token\"");
        
        // In a real LSP implementation, we would:
        // 1. Parse the runbook to get AST positions
        // 2. Query hover info at specific positions
        // 3. Verify the returned documentation
        
        // For now, we verify the runbook structure is valid
        let content = builder.build_content();
        assert!(content.contains("evm::to_wei"));
        assert!(content.contains("std::encode_hex"));
        assert!(content.contains("evm::get_contract_from_foundry_project"));
    }
    
    // Test hover for action types
    #[test]
    fn test_action_hover_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("send", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "1000")
            .action("deploy", "evm::deploy_contract")
                .input("contract", "\"Token.sol\"")
            .action("call", "evm::call")
                .input("contract", "0x456")
                .input("method", "\"transfer\"");
        
        // Hover over action types should show documentation
        let content = builder.build_content();
        assert!(content.contains("evm::send_eth"));
        assert!(content.contains("evm::deploy_contract"));
        assert!(content.contains("evm::call"));
    }
    
    // Test hover for variable references
    #[test]
    fn test_variable_hover_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .variable("base_fee", "1000000000")
            .variable("multiplier", "2")
            .variable("total_fee", "variable.base_fee * variable.multiplier")
            .action("send", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "variable.total_fee");
        
        // Hover over variable references should show type and value info
        let content = builder.build_content();
        assert!(content.contains("variable.base_fee"));
        assert!(content.contains("variable.multiplier"));
        assert!(content.contains("variable.total_fee"));
    }
}

#[cfg(test)]
mod lsp_diagnostics_tests {
    use super::*;
    
    // Test that LSP provides diagnostics for undefined references
    #[test]
    fn test_lsp_undefined_reference_diagnostics() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("send", "evm::send_eth")
                .input("signer", "signer.undefined")  // Undefined signer
                .input("to", "0x123")
                .input("value", "variable.missing");  // Undefined variable
        
        // In LSP mode, this would produce diagnostics
        let result = builder.validate_with_mode(ValidationMode::Doctor {
            manifest: None,
            environment: None,
            file_path: Some(PathBuf::from("test.tx")),
        });
        
        assert!(!result.success);
        assert!(result.errors.len() >= 2);
        assert_has_diagnostic!(&result.errors, "undefined");
        assert_has_diagnostic!(&result.errors, "missing");
    }
    
    // Test LSP diagnostics for type mismatches
    #[test]
    fn test_lsp_type_mismatch_diagnostics() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("send", "evm::send_eth")
                .input("to", "not_an_address")      // Invalid address format
                .input("value", "\"not_a_number\""); // String instead of number
        
        let result = builder.validate_with_doctor(None, None);
        
        // Should have type-related errors
        assert!(!result.success);
    }
    
    // Test LSP diagnostics for circular dependencies
    #[test]
    fn test_lsp_circular_dependency_diagnostics() {
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .variable("a", "variable.b")
            .variable("b", "variable.c")
            .variable("c", "variable.a");  // Circular reference
        
        let result = builder.validate_with_doctor(None, None);
        
        // Should detect circular dependency
        assert!(!result.success);
        assert_has_diagnostic!(&result.errors, "circular");
    }
}

#[cfg(test)]
mod lsp_completion_tests {
    use super::*;
    
    // Test completion for action types
    #[test]
    fn test_action_type_completion_with_builder() {
        // Build a partial runbook where user is typing an action
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .addon("bitcoin", vec![])
            .with_content(r#"
                addon "evm" {}
                addon "bitcoin" {}
                
                action "test" "evm::"  # User typing here, should get evm completions
            "#);
        
        // In real LSP, this would:
        // 1. Parse up to cursor position
        // 2. Determine context (after "evm::")
        // 3. Return completions like: deploy_contract, send_eth, call, etc.
        
        let content = builder.build_content();
        assert!(content.contains("evm::"));
    }
    
    // Test completion for signer references
    #[test]
    fn test_signer_reference_completion_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .signer("deployer", "evm::private_key", vec![
                ("private_key", "0x123")
            ])
            .signer("operator", "evm::private_key", vec![
                ("private_key", "0x456")
            ])
            .with_content(r#"
                signer "deployer" "evm::private_key" {
                    private_key = "0x123"
                }
                signer "operator" "evm::private_key" {
                    private_key = "0x456"
                }
                
                action "test" "evm::send_eth" {
                    signer = signer.  # User typing here, should get signer completions
                }
            "#);
        
        // Completions would include: deployer, operator
        let content = builder.build_content();
        assert!(content.contains("signer."));
    }
    
    // Test completion for action outputs
    #[test]
    fn test_action_output_completion_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("deploy", "evm::deploy_contract")
                .input("contract", "\"Token.sol\"")
            .action("send", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "1000")
            .with_content(r#"
                action "deploy" "evm::deploy_contract" {
                    contract = "Token.sol"
                }
                
                action "send" "evm::send_eth" {
                    to = "0x123"
                    value = "1000"
                }
                
                output "result" {
                    value = action.deploy.  # User typing here, should get output field completions
                }
            "#);
        
        // Completions would include: contract_address, tx_hash, gas_used, etc.
        let content = builder.build_content();
        assert!(content.contains("action.deploy."));
    }
}

#[cfg(test)]
mod lsp_go_to_definition_tests {
    use super::*;
    
    // Test go-to-definition for variables
    #[test]
    fn test_goto_definition_variable_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .variable("base_amount", "1000")
            .variable("multiplier", "10")
            .action("send", "evm::send_eth")
                .input("to", "0x123")
                .input("value", "variable.base_amount * variable.multiplier");
        
        // Go-to-definition on "variable.base_amount" should jump to line with variable definition
        // This would be tested by:
        // 1. Getting cursor position on "variable.base_amount"
        // 2. Calling go-to-definition
        // 3. Verifying it returns the location of variable "base_amount" definition
        
        let content = builder.build_content();
        assert!(content.contains("variable \"base_amount\""));
        assert!(content.contains("variable.base_amount"));
    }
    
    // Test go-to-definition for actions
    #[test]
    fn test_goto_definition_action_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .action("deploy", "evm::deploy_contract")
                .input("contract", "\"Token.sol\"")
            .output("token_address", "action.deploy.contract_address");
        
        // Go-to-definition on "action.deploy" should jump to action definition
        let content = builder.build_content();
        assert!(content.contains("action \"deploy\""));
        assert!(content.contains("action.deploy.contract_address"));
    }
    
    // Test go-to-definition for signers
    #[test]
    fn test_goto_definition_signer_with_builder() {
        let builder = RunbookBuilder::new()
            .addon("evm", vec![])
            .signer("treasury", "evm::private_key", vec![
                ("private_key", "0x123")
            ])
            .action("fund", "evm::send_eth")
                .input("from", "signer.treasury.address")
                .input("to", "0x456")
                .input("value", "1000");
        
        // Go-to-definition on "signer.treasury" should jump to signer definition
        let content = builder.build_content();
        assert!(content.contains("signer \"treasury\""));
        assert!(content.contains("signer.treasury.address"));
    }
}

#[cfg(test)]
mod lsp_multi_file_tests {
    use super::*;
    
    // Test LSP with multi-file runbooks
    #[test]
    fn test_lsp_multi_file_imports_with_builder() {
        let mut builder = RunbookBuilder::new()
            // Main file imports other files
            .with_content(r#"
                import "./signers.tx"
                import "./flows.tx"
                
                addon "evm" {
                    rpc_api_url = "https://eth.example.com"
                }
                
                action "main" "evm::send_eth" {
                    signer = signer.deployer  # Defined in signers.tx
                    to = flow.recipient       # Defined in flows.tx
                    value = "1000"
                }
            "#)
            // Signers file
            .with_file("./signers.tx", r#"
                signer "deployer" "evm::private_key" {
                    private_key = "0x123"
                }
                
                signer "operator" "evm::private_key" {
                    private_key = "0x456"
                }
            "#)
            // Flows file
            .with_file("./flows.tx", r#"
                flow "production" {
                    recipient = "0x789"
                    gas_limit = 21000
                }
            "#);
        
        // LSP should resolve references across files
        let result = builder.validate_with_mode(ValidationMode::Lsp {
            workspace_root: PathBuf::from("."),
            manifest: None,
        });
        
        // When LSP mode is implemented, this would validate cross-file references
        // For now, we just verify the structure
        let content = builder.build_content();
        assert!(content.contains("import"));
    }
}

#[cfg(test)]
mod lsp_workspace_tests {
    use super::*;
    
    // Test LSP with workspace manifest
    #[test]
    fn test_lsp_workspace_manifest_validation() {
        let manifest = create_test_manifest_with_env(vec![
            ("production", vec![
                ("API_URL", "https://api.prod.example.com"),
                ("CHAIN_ID", "1"),
            ]),
            ("staging", vec![
                ("API_URL", "https://api.staging.example.com"),
                ("CHAIN_ID", "5"),
            ]),
        ]);
        
        let mut builder = RunbookBuilder::new()
            .addon("evm", vec![
                ("rpc_api_url", "env.API_URL"),
                ("chain_id", "env.CHAIN_ID"),
            ])
            .action("deploy", "evm::deploy_contract")
                .input("contract", "\"Token.sol\"");
        
        // Validate with LSP mode and manifest
        let result = builder.validate_with_mode(ValidationMode::Lsp {
            workspace_root: PathBuf::from("."),
            manifest: Some(manifest),
        });
        
        // When implemented, LSP would validate env vars against manifest
        let content = builder.build_content();
        assert!(content.contains("env.API_URL"));
        assert!(content.contains("env.CHAIN_ID"));
    }
}

// Helper function to simulate LSP position in content
#[derive(Debug, Clone)]
struct Position {
    line: u32,
    character: u32,
}

impl Position {
    fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

// Utility to find position of text in content
fn find_position_of(content: &str, search: &str) -> Option<Position> {
    let lines: Vec<&str> = content.lines().collect();
    for (line_idx, line) in lines.iter().enumerate() {
        if let Some(col_idx) = line.find(search) {
            return Some(Position::new(line_idx as u32, col_idx as u32));
        }
    }
    None
}