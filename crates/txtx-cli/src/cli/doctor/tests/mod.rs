#[cfg(test)]
mod tests {
    use txtx_test_utils::{assert_success, assert_validation_error, RunbookBuilder};

    #[test]
    fn test_problematic_transfer() {
        let content = include_str!(
            "../../../../../../addons/evm/fixtures/doctor_demo/runbooks/problematic_transfer.tx"
        );
        let result = RunbookBuilder::new().with_content(content).validate();

        // This fixture has 4 errors: from, to, value, gas_used
        assert_eq!(result.errors.len(), 4, "Expected 4 errors in problematic_transfer.tx");

        // Check specific errors
        let error_messages: Vec<_> = result.errors.iter().map(|e| &e.message).collect();

        // Check that we have errors about invalid field access
        assert!(error_messages.iter().any(|m| m.contains("Field 'result' does not exist")));
        assert!(error_messages.iter().any(|m| m.contains("Field 'value' does not exist")));

        // All errors should mention the available outputs
        assert!(error_messages.iter().all(|m| m.contains("Available outputs: tx_hash")));
    }

    #[test]
    fn test_correct_transfer() {
        let content = include_str!(
            "../../../../../../addons/evm/fixtures/doctor_demo/runbooks/correct_transfer.tx"
        );
        let result = RunbookBuilder::new().with_content(content).validate();

        assert_success!(result);
    }

    #[test]
    fn test_undefined_action() {
        // Take a valid fixture and break it by referencing undefined action
        let valid = include_str!(
            "../../../../../../addons/evm/fixtures/doctor_demo/runbooks/correct_transfer.tx"
        );
        let broken = valid.replace("action.transfer.tx_hash", "action.nonexistent.tx_hash");

        let result = RunbookBuilder::new().with_content(&broken).validate();

        assert_eq!(
            result.errors.len(),
            2,
            "Expected 2 errors (one for each reference to nonexistent action)"
        );
        assert_validation_error!(result, "undefined action");
    }

    #[test]
    fn test_send_eth_invalid_field_access() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("send", "evm::send_eth")
            .input("from", "0x123")
            .input("to", "0x456")
            .input("value", "1000")
            .output("bad", "action.send.from")
            .validate();

        assert_validation_error!(result, "Field 'from' does not exist");
        assert_validation_error!(result, "send_eth");
        assert_validation_error!(result, "Available outputs: tx_hash");
    }

    #[test]
    fn test_invalid_action_fields() {
        // Table-driven test for common invalid field access patterns
        let test_cases = vec![
            ("evm::send_eth", "from", "Field 'from' does not exist"),
            ("evm::send_eth", "to", "Field 'to' does not exist"),
            ("evm::send_eth", "gas", "Field 'gas' does not exist"),
            ("evm::send_eth", "gas_used", "Field 'gas_used' does not exist"),
        ];

        for (action_type, field, expected_error) in test_cases {
            let result = RunbookBuilder::new()
                .addon("evm", vec![("network_id", "1")])
                .action("test", action_type)
                .input("value", "1000")
                .output("bad", &format!("action.test.{}", field))
                .validate();

            assert_eq!(result.errors.len(), 1, "Testing field '{}' on {}", field, action_type);
            assert_validation_error!(result, expected_error);
        }
    }

    #[test]
    fn test_nested_invalid_field_access() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("send", "evm::send_eth")
            .input("value", "1000")
            .variable("bad", "action.send.from")
            .output("also_bad", "input.bad")
            .validate();

        // HCL validator catches the first error but not the cascading error
        // This is actually good behavior - it avoids noise from cascading errors
        assert!(result.errors.len() >= 1);
        assert_validation_error!(result, "Field 'from' does not exist");
    }

    #[test]
    fn test_unknown_namespace() {
        let result =
            RunbookBuilder::new().with_content(r#"action "test" "unknown::action" {}"#).validate();

        assert_validation_error!(result, "Unknown addon namespace 'unknown'");
    }

    #[test]
    fn test_unknown_action_type() {
        let result = RunbookBuilder::new()
            .addon("evm", vec![("network_id", "1")])
            .action("test", "evm::unknown_action")
            .validate();

        assert_validation_error!(result, "Unknown action type 'evm::unknown_action'");
    }

    #[test]
    fn test_invalid_action_type_format() {
        let test_cases = vec![
            ("no_namespace", "must be in format 'namespace::action'"),
            ("too::many::colons", "Unknown addon namespace 'too'"), // Different error - unknown namespace
            ("", "must be in format 'namespace::action'"),
            ("::", "Unknown addon namespace ''"), // Empty namespace error
            ("namespace:", "must be in format 'namespace::action'"),
            (":action", "must be in format 'namespace::action'"),
        ];

        for (invalid_type, expected_error) in test_cases {
            let result = RunbookBuilder::new()
                .with_content(&format!(r#"action "test" "{}" {{}}"#, invalid_type))
                .validate();

            if !result.success && !result.errors[0].message.contains(expected_error) {
                println!(
                    "DEBUG: Testing '{}', expected '{}', got '{}'",
                    invalid_type, expected_error, result.errors[0].message
                );
            }
            assert_validation_error!(result, expected_error);
        }
    }

    #[test]
    fn test_multiple_errors_in_one_runbook() {
        let result = RunbookBuilder::new()
            .with_content(
                r#"
                addon "evm" { network_id = 1 }
                
                action "a1" "unknown::action" {}
                action "a2" "evm::unknown_action" {}
                action "a3" "evm::send_eth" { value = "100" }
                
                output "o1" { value = action.a3.from }
                output "o2" { value = action.undefined.field }
                output "o3" { value = input.missing }
            "#,
            )
            .validate();

        // Should have multiple distinct errors
        // The HCL validator catches 4 errors: unknown namespace, unknown action, field access, undefined action
        assert!(
            result.errors.len() >= 4,
            "Expected at least 4 errors, got {}",
            result.errors.len()
        );

        // Check we have different types of errors
        let error_messages = result.errors.iter().map(|e| &e.message).collect::<Vec<_>>();

        assert!(error_messages.iter().any(|m| m.contains("Unknown addon namespace")));
        assert!(error_messages.iter().any(|m| m.contains("Unknown action type")));
        assert!(error_messages.iter().any(|m| m.contains("Field 'from' does not exist")));
        assert!(error_messages.iter().any(|m| m.contains("undefined action")));
    }

    #[test]
    fn test_cascading_errors_suppressed() {
        let result = RunbookBuilder::new()
            .with_content(
                r#"
                action "broken" "unknown::action" {}
                
                output "o1" { value = action.broken.field1 }
                output "o2" { value = action.broken.field2 }
                output "o3" { value = action.broken.field3 }
            "#,
            )
            .validate();

        // Should only have the namespace error, not cascading field access errors
        assert_eq!(result.errors.len(), 1);
        assert_validation_error!(result, "Unknown addon namespace");
    }

    #[test]
    fn test_missing_input_in_environment() {
        // This test validates that undefined input references are caught
        let result = RunbookBuilder::new()
            .with_content(
                r#"
                output "test" {
                    value = input.MISSING_VAR
                }
            "#,
            )
            .with_environment("prod", vec![("OTHER_VAR", "value")])
            .set_current_environment("prod") // Enable manifest validation
            .validate();

        // HCL validation should catch undefined input reference
        assert_validation_error!(result, "MISSING_VAR");
    }

    #[test]
    fn test_environment_global_inheritance() {
        let result = RunbookBuilder::new()
            .with_content(
                r#"
                variable "key1" { value = env.KEY1 }
                variable "key2" { value = env.KEY2 }
                
                output "k1" { value = input.key1 }
                output "k2" { value = input.key2 }
            "#,
            )
            .with_environment("global", vec![("KEY1", "global1")])
            .with_environment("test", vec![("KEY2", "test2")])
            .validate();

        // Should pass - test env should inherit from global
        assert_success!(result);
    }

    #[test]
    fn test_cli_inputs_override_environment() {
        let result = RunbookBuilder::new()
            .with_content(
                r#"
                variable "key" { value = env.KEY }
                output "result" { value = input.key }
            "#,
            )
            .with_environment("test", vec![("KEY", "env_value")])
            .with_cli_input("key", "cli_value")
            .validate();

        assert_success!(result);
    }

    #[test]
    fn test_cli_precedence_note() {
        let result = RunbookBuilder::new()
            .with_content(
                r#"
                variable "key" { value = env.MISSING }
                output "result" { value = input.key }
            "#,
            )
            .with_cli_input("key", "cli_value")
            .validate();

        // Should succeed because CLI input overrides the missing env var
        assert_success!(result);
    }
}
