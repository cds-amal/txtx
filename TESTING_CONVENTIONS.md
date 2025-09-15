# Testing Conventions for txtx

This document outlines the testing conventions and cargo aliases used in the txtx project.

## Test Organization

### Unit Tests
- **Location**: Within `src/` directories alongside the code they test
- **Purpose**: Test individual functions, modules, and components in isolation
- **Naming**: Follow Rust conventions (`#[cfg(test)] mod tests`)

### Integration Tests  
- **Location**: In `tests/` directories at the crate level
- **Purpose**: Test complete workflows and interactions between components
- **Naming**: Descriptive test file names (e.g., `doctor_tests.rs`, `lsp_hover_test.rs`)

## Cargo Test Aliases

We use a consistent naming pattern for test aliases: `test-[scope]-[type]-[target]`

### Pattern Components
- **scope**: The crate being tested (e.g., `cli`, `core`, `addon-kit`)
- **type**: Either `unit` or `int` (integration)
- **target**: Optional specific module or test file

### Available Aliases

#### Unit Test Aliases
```bash
cargo test-cli-unit           # All unit tests in txtx-cli
cargo test-cli-unit-doctor    # Only doctor module unit tests
cargo test-cli-unit-lsp       # Only LSP module unit tests
cargo test-core-unit          # All unit tests in txtx-core
cargo test-addon-kit-unit     # All unit tests in txtx-addon-kit
```

#### Integration Test Aliases
```bash
cargo test-cli-int            # All integration tests for txtx-cli
cargo test-cli-int-doctor     # Original doctor integration tests
cargo test-cli-int-doctor-new # New doctor tests using RunbookBuilder
cargo test-cli-int-lsp        # LSP integration tests
```

#### Convenience Aliases
```bash
cargo test-cli                # All CLI tests (unit + integration)
cargo build-cli               # Build CLI without supervisor UI
cargo build-cli-release       # Release build without supervisor UI
```

## Examples

### Testing a specific module
```bash
# Run only doctor unit tests
cargo test-cli-unit-doctor

# Run only doctor integration tests
cargo test-cli-int-doctor
```

### Testing during development
```bash
# Quick test run without supervisor UI build
cargo test-cli-unit

# Test the new RunbookBuilder API
cargo test-cli-int-doctor-new
```

### Running specific test patterns
```bash
# Run a specific test by name
cargo test-cli-unit test_input_defined_rule

# Run tests matching a pattern
cargo test-cli-int validation
```

## Notes

- All CLI test aliases use `--no-default-features --features cli` to avoid building the supervisor UI
- The supervisor UI is an optional dependency that significantly increases build time
- Use the specific aliases to run only the tests you need during development