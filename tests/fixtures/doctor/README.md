# Doctor Command Test Fixtures

This directory contains test runbook files for testing the `txtx doctor` command.

## Test Files

- `test_doctor.tx` - Basic doctor command functionality
- `test_doctor_errors.tx` - Error detection and reporting
- `test_doctor_validation.tx` - Validation rules testing
- `test_doctor_flow_validation.tx` - Flow validation testing
- `test_doctor_parser.tx` - Parser edge cases
- `test_doctor_nocomments.tx` - Tests without comments
- `test_doctor_two_pass.tx` - Two-pass validation testing
- `test_doctor_valid.tx` - Valid runbook examples
- `test_doctor_simple.tx` - Simple test cases

## Usage

These files are used by:
- Integration tests in `crates/txtx-cli/tests/doctor_tests.rs`
- Shell scripts in `tests/scripts/test_doctor_*.sh`

## Adding New Tests

When adding new doctor test fixtures:
1. Name the file with `test_doctor_` prefix
2. Include comments explaining what is being tested
3. Update this README with the test purpose