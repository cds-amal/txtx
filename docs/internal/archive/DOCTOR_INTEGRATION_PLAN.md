# Doctor Command Proper Integration Plan

## Status: ✅ IMPLEMENTED

The doctor command has been properly integrated as part of the architectural refactoring. The implementation addresses all the issues identified below.

## Implementation Details

### Current Architecture (Implemented)

1. **Parser Integration**: The doctor command now uses the txtx-core parser via the validation system
   - See: [`crates/txtx-cli/src/cli/doctor/analyzer/mod.rs`](crates/txtx-cli/src/cli/doctor/analyzer/mod.rs)
   - Uses `txtx_core::validation` for proper AST-based validation

2. **Addon System Integration**: Full addon specifications are loaded and used
   - See: [`crates/txtx-cli/src/cli/common/addon_registry.rs`](crates/txtx-cli/src/cli/common/addon_registry.rs)
   - Shared registry between doctor and LSP ensures consistency

3. **Accurate Validation**: The system now provides factual, actionable feedback
   - See: [`crates/txtx-cli/src/cli/doctor/analyzer/rules.rs`](crates/txtx-cli/src/cli/doctor/analyzer/rules.rs)
   - Rule-based validation with proper error context

## Original Issues (Now Resolved)

1. ~~**Not using txtx-core parser**~~ - Now uses proper AST parsing
2. ~~**Not using addon specifications**~~ - Full addon integration implemented
3. ~~**Giving incorrect suggestions**~~ - Only provides factual, verified suggestions
4. ~~**Limited error detection**~~ - Comprehensive validation using actual runtime checks

## Proper Integration Approach

### 1. Use txtx-core Parser

```rust
use txtx_core::{
    parser::{parse_runbook, RunbookRequest},
    runbook::Runbook,
};

async fn validate_runbook_content(
    content: &str,
    path: &Path,
    name: &str,
    results: &mut DoctorResults,
) -> Result<(), String> {
    // Use the actual parser
    let runbook_request = RunbookRequest {
        content: content.to_string(),
        source: path.to_string_lossy().to_string(),
        // ... other fields
    };
    
    match parse_runbook(runbook_request).await {
        Ok(runbook) => validate_parsed_runbook(runbook, results),
        Err(diagnostics) => {
            // Convert parser diagnostics to doctor results
            for diag in diagnostics {
                results.add_issue(convert_diagnostic(diag));
            }
        }
    }
}
```

### 2. Use Addon System for Validation

```rust
use txtx_addon_kit::Addon;

fn validate_action_outputs(
    action: &Action,
    runbook: &Runbook,
    addon: &dyn Addon,
    results: &mut DoctorResults,
) {
    // Get the actual command specification from the addon
    let command_spec = addon.get_command(&action.command_type);
    
    // Check each output reference in the runbook
    for output_ref in find_output_references(&action.name, runbook) {
        if !command_spec.outputs.contains(&output_ref.field) {
            results.add_issue(DiagnosticIssue {
                severity: Severity::Error,
                category: Category::OutputValidation,
                message: format!(
                    "Action '{}' (type '{}') does not have output field '{}'",
                    action.name, action.command_type, output_ref.field
                ),
                location: Some(output_ref.location),
                suggestion: Some(format!(
                    "Available outputs: {}",
                    command_spec.outputs.keys().join(", ")
                )),
                example: None, // Only provide examples when we're certain
            });
        }
    }
}
```

### 3. Detect Actual Runtime Errors

```rust
// Run the same validation that would happen at runtime
match runbook.validate() {
    Ok(_) => {},
    Err(errors) => {
        for error in errors {
            results.add_issue(convert_runtime_error(error));
        }
    }
}
```

### 4. Only Suggest What We Know

Instead of guessing solutions, only provide factual information:

```rust
suggestion: Some(format!(
    "The '{}' action only provides these outputs: {}. \
     You are trying to access '{}'.",
    action_type,
    available_outputs.join(", "),
    requested_field
)),
// Don't suggest non-existent actions or patterns we're not sure about
example: None,
```

## Benefits of Proper Integration

1. **Accurate error detection** - Catches the same errors that would occur at runtime
2. **No false positives** - Only reports real issues
3. **No incorrect advice** - Suggestions based on actual system capabilities
4. **Comprehensive validation** - Checks everything the runtime would check
5. **Consistent with runtime** - Uses the same validation logic

## Implementation Steps

1. Add dependency on txtx-core to doctor module
2. Initialize addon system with all available addons
3. Use parse_runbook to parse the content
4. Walk the AST to find all action output references
5. Validate each reference against actual addon specifications
6. Report only confirmed issues with factual information

## Example of Proper Error Message

```
❌ [runbooks/transfer.tx:21] Invalid output access: Action 'transfer' of type 'evm::send_eth' does not have output field 'result'
   Available outputs: tx_hash
   
   The field 'result' does not exist. The 'evm::send_eth' action only outputs 'tx_hash'.
```

No incorrect suggestions about non-existent `get_transaction` actions!