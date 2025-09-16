# Doctor Test Refactoring - COMPLETED (2025-09-16)

## Problem Statement

The doctor tests currently use a custom parser in `txtx-test-utils` that doesn't match the actual doctor command implementation. This creates several issues:

1. **Test Accuracy**: Tests may pass while the actual CLI fails (or vice versa)
2. **Maintenance Burden**: Two separate validation implementations to maintain
3. **Feature Parity**: New validation features must be implemented twice
4. **Incomplete Coverage**: Custom parser only validates signers, actions, and env vars (missing flows, types, addons, etc.)

## Current State

### Actual Doctor Command (Production)
```rust
// txtx-cli/src/cli/doctor/analyzer/mod.rs
hcl_validator::validate_with_hcl_and_addons(
    content,
    &mut result,
    &file_path.to_string_lossy(),
    addon_specs,
)
```

### Test Utils Doctor Mode
```rust
// txtx-test-utils/src/builders/runbook_builder_enhanced.rs
ValidationMode::Doctor { ... } => {
    // Use custom parser - NOT the same as production!
    use super::parser::{parse_runbook_content, ...};
}
```

## Implementation Summary

The refactoring has been successfully completed! The `ValidationMode::Doctor` in test utils now uses the same HCL validator as the production doctor command.

### What Was Done

1. **Updated ValidationMode::Doctor** to use `hcl_validator::validate_with_hcl_and_addons()`
2. **Removed custom parser logic** and replaced it with HCL validator calls
3. **Added proper manifest validation** when manifest and environment are provided
4. **All doctor tests pass** without any modifications needed
5. **Flow validation test** now works correctly with doctor mode

### Code Changes

The key change in `txtx-test-utils/src/builders/runbook_builder_enhanced.rs`:

```rust
ValidationMode::Doctor { manifest, environment, file_path } => {
    // Now uses the same HCL validator as production!
    let mut core_result = CoreResult::new();
    let addon_specs = extract_addon_specifications(&get_all_addons());
    
    match hcl_validator::validate_with_hcl_and_addons(
        &content,
        &mut core_result,
        &file_path_str,
        addon_specs,
    ) {
        Ok(input_refs) => {
            // Validate inputs against manifest if provided
            if let (Some(manifest), Some(env_name)) = (&manifest, &environment) {
                validate_inputs_against_manifest(...);
            }
        }
        // ... error handling
    }
}
```

## Original Proposed Solution (For Reference)

### Option 1: Use HCL Validator Directly (Recommended)

Update the test utils to use the same validation path as production:

```rust
ValidationMode::Doctor { manifest, environment, file_path } => {
    // Use the same validator as the actual doctor command
    let mut core_result = CoreResult::new();
    let addon_specs = extract_addon_specifications(&get_all_addons());
    
    let input_refs = hcl_validator::validate_with_hcl_and_addons(
        content,
        &mut core_result,
        file_path.as_deref().unwrap_or("test.tx"),
        addon_specs,
    )?;
    
    // If manifest provided, validate inputs
    if let Some(manifest) = manifest {
        validate_inputs_against_manifest(
            &input_refs,
            content,
            &manifest,
            environment.as_ref(),
            &mut core_result,
            file_path.as_deref().unwrap_or("test.tx"),
            &[], // CLI inputs
        );
    }
    
    // Convert to test utils result type
    convert_core_result(core_result)
}
```

### Option 2: Extract Doctor Logic to Core

Move the entire doctor validation logic to `txtx-core` so both CLI and test utils can use it:

```rust
// txtx-core/src/validation/doctor.rs
pub fn validate_with_doctor(
    content: &str,
    file_path: &str,
    manifest: Option<&WorkspaceManifest>,
    environment: Option<&str>,
    cli_inputs: &[(String, String)],
) -> ValidationResult {
    // Full doctor validation logic here
}
```

## Benefits of Refactoring

1. **Single Source of Truth**: One validation implementation used everywhere
2. **Accurate Tests**: Tests reflect actual CLI behavior
3. **Easier Maintenance**: Changes to validation logic only need to be made once
4. **Complete Validation**: All validation features available in tests (flows, types, etc.)

## Migration Plan

1. **Phase 1**: Update `ValidationMode::Doctor` to use HCL validator
2. **Phase 2**: Update existing tests that rely on custom parser behavior
3. **Phase 3**: Remove custom parser code from test utils
4. **Phase 4**: Add tests for newly available validation features

## Results

### All Tests Pass Without Modification

Remarkably, all doctor tests passed without any changes needed! This demonstrates that:
1. The HCL validator provides compatible error detection
2. The test expectations were already aligned with proper validation
3. The refactoring was successful and non-breaking

### Benefits Achieved

1. **Single Source of Truth**: Tests now use the exact same validation as production
2. **Complete Validation**: Tests can now validate flows, types, addons, and all HCL features
3. **Future-Proof**: New validation features automatically available in tests
4. **Reduced Maintenance**: No more custom parser to maintain
5. **Better Test Coverage**: The flow validation test now works correctly with doctor mode

### Next Steps

The custom parser code in `txtx-test-utils/src/builders/parser.rs` can now be removed if it's not used elsewhere, further simplifying the codebase.