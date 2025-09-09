# Txtx Doctor Command Proposal

## Overview

This proposal introduces `txtx doctor`, a diagnostic command that validates runbook configurations and provides actionable feedback. This command would have saved 2+ hours of debugging time on the `send_eth` output field issue.

## Problem Statement

Current txtx error messages like "DependencyNotComputed" provide no actionable information when developers try to access non-existent output fields. In our case, we tried to access `action.transfer.result.from` when `send_eth` only provides `tx_hash`.

## Solution: txtx doctor

A comprehensive validation tool that:
1. Validates manifest structure (txtx.yml)
2. Checks runbook file resolution
3. Validates action usage and output access
4. Provides clear, actionable error messages with examples

## Command Usage

```bash
# Validate all runbooks in the workspace
txtx doctor

# Validate a specific runbook
txtx doctor my_runbook

# Validate with custom manifest path
txtx doctor --manifest-file-path ./custom/txtx.yml
```

## Example Output

When detecting the send_eth issue:

```
🏥 Txtx Doctor Results

📊 Summary:
   Runbooks checked: 1
   Actions validated: 1
   Outputs validated: 2

📋 Issues found:
   ❌ Errors: 2
   ⚠️  Warnings: 0
   ℹ️  Info: 0

📤 Output Validation Issues (2 issues):

  ❌ [runbooks/problematic.tx:19] Invalid output access: 'send_eth' action 'transfer' only provides 'tx_hash' output
     💡 Suggestion: To get transaction details, use 'evm::get_transaction' with the tx_hash
     📝 Example:
        action "tx_details" "evm::get_transaction" {
            tx_hash = action.transfer.tx_hash
            rpc_api_url = var.rpc_url
        }

        output "from_address" {
            value = action.tx_details.from
        }
```

## Implementation with Error-Stack

The doctor command is a perfect use case for the new error-stack integration:

```rust
// Enhanced error attachments for rich diagnostics
let error = Report::new(OutputFieldError)
    .attach(ErrorLocation {
        file: "problematic.tx",
        line: 19,
        column: 23,
    })
    .attach(ActionContext {
        action_name: "transfer",
        action_type: "evm::send_eth",
    })
    .attach(AvailableOutputs {
        outputs: vec![("tx_hash", "string", "Transaction hash")],
    })
    .attach(Suggestion {
        text: "Use evm::get_transaction to get full details",
        example: example_code,
    });
```

## Categories of Validation

1. **Manifest Issues**
   - Missing or invalid txtx.yml
   - Invalid runbook definitions
   - Missing environment configurations

2. **Runbook Resolution**
   - Runbook files not found
   - Invalid file paths
   - Circular dependencies

3. **Action Validation**
   - Unknown action types
   - Missing required inputs
   - Invalid input types

4. **Output Validation**
   - Accessing non-existent outputs
   - Type mismatches in output usage
   - Invalid output expressions

5. **Dependency Issues**
   - Referencing undefined actions
   - Circular action dependencies
   - Missing signer references

6. **Type Mismatches**
   - Passing wrong types to action inputs
   - Output type incompatibilities

## Benefits

1. **Immediate Feedback**: Catch errors before running runbooks
2. **Learning Tool**: Examples teach correct patterns
3. **Time Savings**: What took hours to debug is caught instantly
4. **Better DX**: Clear, actionable error messages
5. **Prevents Runtime Failures**: Catch issues early in development

## Future Enhancements

1. **Auto-fix Suggestions**: Automatically apply fixes for common issues
2. **IDE Integration**: Real-time validation in VS Code
3. **Custom Rules**: Allow teams to define project-specific validations
4. **Performance Analysis**: Suggest optimizations for runbook execution
5. **Security Checks**: Warn about potential security issues

## Migration Path

1. Start with output validation (highest impact based on our experience)
2. Add action input validation
3. Implement dependency checking
4. Add type checking
5. Integrate with CI/CD pipelines

## Conclusion

The `txtx doctor` command would transform the developer experience by providing proactive validation and clear guidance. The 2+ hours we spent debugging the `send_eth` output issue would have been reduced to seconds with this tool.

Combined with the error-stack migration, txtx would provide world-class error reporting and validation, making blockchain automation more accessible and less error-prone.