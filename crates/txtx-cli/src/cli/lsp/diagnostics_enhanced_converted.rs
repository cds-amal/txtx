// ... (original non-test code would remain the same) ...

#[cfg(test)]
mod tests {
    use super::*;
    use txtx_test_utils::{RunbookBuilder, assert_validation_error, assert_validation_warning};
    
    #[test]
    fn test_validate_missing_input() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("deploy", "evm::deploy_contract")
                .input("private_key", "input.deployer_key")
            .with_environment("production", vec![])  // Empty environment
            .validate();
        
        // Should have at least one error about missing input
        assert_validation_error!(result, "deployer_key");
        assert_validation_error!(result, "not defined");
    }
    
    #[test]
    fn test_validate_sensitive_input() {
        let result = RunbookBuilder::new()
            .action("deploy", "evm::deploy_contract")
                .input("private_key", "input.wallet_private_key")
            .with_environment("production", vec![
                ("wallet_private_key", "0x123")
            ])
            .validate();
        
        // Should have warning about sensitive data
        assert_validation_warning!(result, "private_key");
        assert_validation_warning!(result, "sensitive");
    }
    
    #[test]
    fn test_validate_cli_override() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("deploy", "evm::deploy_contract")
                .input("private_key", "input.deployer_key")
            .with_environment("production", vec![
                ("deployer_key", "env_key")
            ])
            .with_cli_input("deployer_key", "cli_key")
            .validate();
        
        // Should have info about CLI override
        let has_override_info = result.warnings.iter().any(|w| 
            w.message.contains("CLI") && w.message.contains("override")
        );
        assert!(has_override_info, "Expected info about CLI override");
    }
    
    #[test]
    fn test_validate_with_global_environment() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("deploy", "evm::deploy_contract")
                .input("api_url", "input.api_url")
                .input("api_key", "input.api_key")
            .with_environment("global", vec![
                ("api_url", "https://api.example.com")
            ])
            .with_environment("production", vec![
                ("api_key", "prod_key")
            ])
            .validate();
        
        // Should pass - production inherits from global
        assert!(result.success);
    }
    
    #[test]
    fn test_validate_unknown_action_type() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("test", "evm::unknown_action")
            .validate();
        
        assert_validation_error!(result, "Unknown action type");
        assert_validation_error!(result, "evm::unknown_action");
    }
    
    #[test]
    fn test_validate_invalid_field_access() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("send", "evm::send_eth")
                .input("recipient_address", "0x123")
                .input("amount", "1000")
            .output("bad_field", "action.send.invalid_field")
            .validate();
        
        assert_validation_error!(result, "Field 'invalid_field' does not exist");
        assert_validation_error!(result, "Available outputs");
    }
    
    #[test]
    fn test_validate_with_includes() {
        let result = RunbookBuilder::new()
            .with_file("main.tx", r#"
                include "common.tx"
                
                action "deploy" "evm::deploy_contract" {
                    contract = input.contract_path
                }
            "#)
            .with_file("common.tx", r#"
                addon "evm" { network_id = 1 }
                
                variable "contract_path" {
                    value = "./Token.sol"
                }
            "#)
            .validate();
        
        assert!(result.success);
    }
    
    #[test]
    fn test_validate_unused_variable() {
        let result = RunbookBuilder::new()
            .variable("used_var", "value1")
            .variable("unused_var", "value2")
            .output("result", "input.used_var")
            .validate();
        
        // Should warn about unused variable
        assert_validation_warning!(result, "unused_var");
        assert_validation_warning!(result, "not used");
    }
    
    #[test]
    fn test_validate_naming_conventions() {
        let result = RunbookBuilder::new()
            .variable("myVariable", "value1")      // camelCase
            .variable("_private_var", "value2")    // underscore prefix
            .variable("my-var", "value3")          // hyphens
            .validate();
        
        // Should have warnings about naming conventions
        assert_validation_warning!(result, "my_variable");
        assert_validation_warning!(result, "underscore");
        assert_validation_warning!(result, "hyphens");
    }
}