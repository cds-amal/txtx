# Doctor Command Test Fixtures

This directory contains test runbook files for testing the `txtx doctor` command.

## ⚠️ Current Status: INEFFICIENT TEST USAGE

**Problem**: These fixtures are NOT being used in automated tests!
- Only 1 fixture is referenced in shell scripts
- The unit tests create inline test data instead of using these fixtures
- No integration tests actually use these fixtures systematically

## Test Files

### test_doctor_valid.tx
- **Purpose**: Valid runbook with no errors
- **Expected**: Should pass with 0 errors
- **Current Usage**: ❌ Not used in tests

### test_doctor_simple.tx
- **Purpose**: Common errors (undefined signer, invalid field access)
- **Expected**: 2 errors (undefined signer, invalid field)
- **Current Usage**: ✅ Used in test_doctor_simple.sh

### test_doctor_two_pass.tx
- **Purpose**: Test two-pass validation logic
- **Expected**: 1 error (undefined action reference)
- **Current Usage**: ❌ Not used in tests

### test_doctor_bad_flow_detection.tx
- **Purpose**: Test action type validation (evm::deploy error)
- **Expected**: 1 error (unknown action type)
- **Current Usage**: ❌ Not used in tests

### test_doctor_errors.tx
- **Purpose**: Multiple error types in one file
- **Expected**: Multiple errors
- **Current Usage**: ❌ Not used in tests

### test_doctor_flow_validation.tx
- **Purpose**: Flow block validation
- **Expected**: Errors for missing required attributes
- **Current Usage**: ❌ Not used in tests

### test_doctor_nocomments.tx
- **Purpose**: Test parsing without comments
- **Current Usage**: ❌ Not used in tests

### test_doctor_parser.tx
- **Purpose**: Parser edge cases
- **Current Usage**: ❌ Not used in tests

### test_doctor_validation.tx
- **Purpose**: General validation scenarios
- **Current Usage**: ❌ Not used in tests

### test_doctor.tx
- **Purpose**: Complex runbook with dependencies
- **Current Usage**: ❌ Not used in tests

## Recommended Improvements

1. **Create Integration Tests**: Add `tests/doctor_fixture_tests.rs` that systematically tests all fixtures
2. **Document Expected Outcomes**: Each fixture should clearly state expected error count and types
3. **Use JSON Format**: Test with `--format json` for reliable assertions
4. **Remove Duplicates**: Consolidate similar test cases
5. **Add Test Matrix**: Create a data-driven test that runs all fixtures

## Example Integration Test

```rust
#[test]
fn test_all_doctor_fixtures() {
    let fixtures = [
        ("test_doctor_valid.tx", true, 0),
        ("test_doctor_simple.tx", false, 2),
        ("test_doctor_two_pass.tx", false, 1),
        // ... etc
    ];
    
    for (fixture, should_pass, expected_errors) in fixtures {
        // Run doctor command with JSON output
        // Assert on pass/fail and error count
    }
}