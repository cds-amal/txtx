// This file contains the validation rules used by the doctor analyzer
// Most of the original content remains the same, only tests are converted

// ... (original non-test code would go here) ...

#[cfg(test)]
mod tests {
    use super::*;
    use txtx_test_utils::{RunbookBuilder, assert_validation_error, assert_validation_warning};

    #[test]
    fn test_input_defined_rule_pass() {
        // Test that inputs are properly defined
        let result = RunbookBuilder::new()
            .variable("my_var", "value")
            .output("result", "input.my_var")
            .validate();
            
        // Should pass - input is defined
        assert!(result.success);
    }

    #[test]
    fn test_input_defined_rule_fail() {
        // Test missing input detection
        let result = RunbookBuilder::new()
            .output("result", "input.missing_var")
            .validate();
            
        assert_validation_error!(result, "not defined");
    }

    #[test]
    fn test_naming_convention_underscore() {
        // Test underscore naming convention warning
        let result = RunbookBuilder::new()
            .variable("_private_var", "value")
            .output("result", "input._private_var")
            .validate();
            
        // Should warn about underscore prefix
        assert_validation_warning!(result, "underscore");
    }

    #[test]
    fn test_cli_override_rule() {
        // Test CLI input override detection
        let result = RunbookBuilder::new()
            .variable("api_key", "env.API_KEY")
            .output("result", "input.api_key")
            .with_environment("test", vec![("API_KEY", "env_value")])
            .with_cli_input("api_key", "cli_value")
            .validate();
            
        // Should warn about CLI override
        assert_validation_warning!(result, "overridden by CLI");
    }

    #[test]
    fn test_sensitive_data_rule() {
        // Test sensitive data detection in various contexts
        let test_cases = vec![
            ("password", "password"),
            ("api_key", "api_key"),
            ("secret_token", "secret"),
            ("private_key", "private_key"),
            ("access_token", "token"),
        ];

        for (var_name, expected_keyword) in test_cases {
            let result = RunbookBuilder::new()
                .variable(var_name, "sensitive_value")
                .output("exposed", &format!("input.{}", var_name))
                .validate();
                
            assert_validation_warning!(result, expected_keyword);
            assert_validation_warning!(result, "consider encrypting");
        }
    }

    #[test]
    fn test_sensitive_data_in_output() {
        // Test that outputting sensitive data triggers warning
        let result = RunbookBuilder::new()
            .with_content(r#"
                variable "api_key" { value = "secret123" }
                output "exposed_key" { value = input.api_key }
            "#)
            .validate();
            
        assert_validation_warning!(result, "api_key");
        assert_validation_warning!(result, "output");
    }

    #[test]
    fn test_hardcoded_credential_detection() {
        // Test detection of hardcoded credentials
        let result = RunbookBuilder::new()
            .with_content(r#"
                variable "password" { value = "hardcoded123" }
                action "test" "std::send_http_request" {
                    url = "https://api.example.com"
                    headers = { "X-API-Key" = "sk_live_1234567890" }
                }
            "#)
            .validate();
            
        assert_validation_warning!(result, "hardcoded");
    }

    #[test]
    fn test_naming_convention_camelcase() {
        // Test camelCase naming convention
        let result = RunbookBuilder::new()
            .variable("myVariable", "value")
            .variable("AnotherVar", "value2")
            .validate();
            
        // Should suggest snake_case
        assert_validation_warning!(result, "my_variable");
        assert_validation_warning!(result, "another_var");
    }

    #[test]
    fn test_naming_convention_hyphens() {
        // Test hyphenated names
        let result = RunbookBuilder::new()
            .variable("my-variable", "value")
            .output("result", "input.my-variable")
            .validate();
            
        assert_validation_warning!(result, "hyphens");
        assert_validation_warning!(result, "underscores");
    }

    #[test]
    fn test_environment_specific_validation() {
        // Test environment-specific input validation
        let result = RunbookBuilder::new()
            .with_content(r#"
                variable "db_url" { value = env.DATABASE_URL }
                variable "api_key" { value = env.API_KEY }
            "#)
            .with_environment("production", vec![
                ("DATABASE_URL", "postgres://prod"),
                ("API_KEY", "prod_key")
            ])
            .with_environment("development", vec![
                ("DATABASE_URL", "postgres://dev")
                // Missing API_KEY in dev
            ])
            .validate();
            
        // Should detect missing API_KEY in dev environment
        assert!(result.errors.iter().any(|e| 
            e.message.contains("API_KEY") && e.message.contains("development")
        ));
    }

    #[test]
    fn test_unused_variable_detection() {
        // Test detection of unused variables
        let result = RunbookBuilder::new()
            .variable("used_var", "value1")
            .variable("unused_var", "value2")
            .output("result", "input.used_var")
            .validate();
            
        assert_validation_warning!(result, "unused_var");
        assert_validation_warning!(result, "not used");
    }

    #[test]
    fn test_circular_reference_detection() {
        // Test circular reference detection
        let result = RunbookBuilder::new()
            .with_content(r#"
                variable "a" { value = input.b }
                variable "b" { value = input.c }
                variable "c" { value = input.a }
            "#)
            .validate();
            
        assert_validation_error!(result, "circular");
    }

    #[test]
    fn test_type_mismatch_detection() {
        // Test type mismatch warnings
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("send", "evm::send_eth")
                .input("value", "not_a_number")  // Should be numeric
                .input("to", "0x123")
                .input("from", "0x456")
            .validate();
            
        assert_validation_error!(result, "value");
        assert_validation_error!(result, "number");
    }
}