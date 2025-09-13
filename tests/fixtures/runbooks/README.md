# General Runbook Test Fixtures

This directory contains general test runbook files for various txtx features.

## Test Files

- `test_error.tx` - Error handling and reporting
- `test_quickfix.tx` - Quick fix suggestions
- `test_flow_semantics.tx` - Flow control and semantics
- `test_step5.tx` - Multi-step execution tests

## Purpose

These files test general runbook functionality including:
- Syntax validation
- Execution flow
- Error handling
- Variable resolution
- Action execution

## Usage

Used by various integration tests to verify:
- Parser functionality
- Runtime execution
- Error reporting
- Flow control

## Adding New Tests

When adding new general test fixtures:
1. Use descriptive names for the test scenario
2. Document the specific feature being tested
3. Include both positive and negative test cases where applicable