# PR Summary: Doctor Command and LSP Implementation

## Table of Contents

- [PR Summary: Doctor Command and LSP Implementation](#pr-summary-doctor-command-and-lsp-implementation)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Major Features Introduced](#major-features-introduced)
    - [1. **Doctor Command** (`txtx doctor`)](#1-doctor-command-txtx-doctor)
    - [2. **Language Server Protocol (LSP) Implementation**](#2-language-server-protocol-lsp-implementation)
    - [3. **Unified Validation Architecture**](#3-unified-validation-architecture)
  - [Architecture Overview](#architecture-overview)
    - [System Architecture](#system-architecture)
    - [Validation Pipeline](#validation-pipeline)
  - [Key Technical Decisions](#key-technical-decisions)
    - [1. **Integrated Design**](#1-integrated-design)
    - [2. **Shared Validation Core**](#2-shared-validation-core)
    - [3. **Synchronous LSP Design**](#3-synchronous-lsp-design)
    - [4. **Manifest-First Validation**](#4-manifest-first-validation)
  - [Developer Experience Improvements](#developer-experience-improvements)
    - [1. **Build and Test Aliases**](#1-build-and-test-aliases)
    - [2. **Structured Test Writing with RunbookBuilder**](#2-structured-test-writing-with-runbookbuilder)
    - [3. **Flexible Validation Modes**](#3-flexible-validation-modes)
    - [4. **Test Organization Pattern**](#4-test-organization-pattern)
  - [Supporting Infrastructure](#supporting-infrastructure)
    - [1. **Test Utilities** (`txtx-test-utils`)](#1-test-utilities-txtx-test-utils)
    - [2. **Common Utilities** (`txtx-cli/src/cli/common`)](#2-common-utilities-txtx-clisrcclicommon)
    - [3. **Enhanced Error Types**](#3-enhanced-error-types)
  - [File Structure](#file-structure)
    - [New Directories](#new-directories)
  - [Usage Examples](#usage-examples)
    - [Doctor Command](#doctor-command)
    - [LSP with VSCode](#lsp-with-vscode)
  - [Testing](#testing)
    - [Test Coverage](#test-coverage)
    - [New Testing Patterns](#new-testing-patterns)
  - [Documentation](#documentation)
  - [Impact](#impact)
    - [For Users](#for-users)
    - [For txtx Development](#for-txtx-development)
  - [Future Enhancements](#future-enhancements)
    - [Doctor Command Capabilities](#doctor-command-capabilities)
    - [LSP Features](#lsp-features)
    - [Test Infrastructure Integration](#test-infrastructure-integration)
  - [Summary](#summary)

---

## Overview

This branch introduces two major new features to txtx:

1. **Doctor Command** - A validation and analysis tool for txtx runbooks
2. **Language Server Protocol (LSP)** - Full IDE integration with real-time validation and intelligent code assistance

These additions transform txtx from a basic CLI tool into a professional development environment with advanced validation capabilities and IDE support.

## Major Features Introduced

### 1. **Doctor Command** (`txtx doctor`)

A new CLI command that provides validation and analysis of txtx runbooks with multiple output formats.

**Key Features:**

- **Multi-level validation** - Syntax, semantic, and best practice checks
- **Manifest validation** - Validates runbooks against workspace manifest (txtx.yml)
- **Multiple output formats**:
  - Terminal (colored, human-readable)
  - JSON (machine-readable)
  - Quickfix (editor integration)
- **Enhanced error reporting** - Context, suggestions, and documentation links
- **Multi-file runbook support** - Validates entire runbook directories
- **Extensible rule system** - Trait-based validation rules

**Usage:**

```bash
# Validate all runbooks in workspace
txtx doctor

# Validate specific runbook with environment
txtx doctor deploy -e production

# Output as JSON
txtx doctor --format json
```

### 2. **Language Server Protocol (LSP) Implementation**

A full-featured LSP server integrated directly into the txtx CLI, providing real-time IDE support.

**Key Features:**

- **Real-time validation** - Syntax and semantic errors as you type
- **Auto-completion** - Smart completions for input variables
- **Hover documentation** - Inline docs for functions, actions, and variables
- **Go-to-definition** - Jump to variable and signer definitions
- **Multi-file support** - Understands cross-file references
- **Workspace awareness** - Recognizes txtx.yml manifests

**VSCode Extension:**

- Custom syntax highlighting for `.tx` files
- Full LSP client implementation
- Integrated with txtx CLI

### 3. **Unified Validation Architecture**

Both doctor and LSP share a common validation infrastructure built on a new architecture in `txtx-core`.

**Components:**

- **ValidationContext** - Centralized validation configuration
- **HCL Validator** - Deep integration with `hcl-edit` parser
- **Manifest Validator** - Environment-aware input validation
- **Doctor Rules** - Extensible validation rules
- **Addon Integration** - Validates against all blockchain addons

## Architecture Overview

### System Architecture

```text
┌─────────────────┐     ┌─────────────────┐
│   VSCode/IDE    │     │    CLI User     │
└────────┬────────┘     └────────┬────────┘
         │                       │
         ↓                       ↓
┌─────────────────┐     ┌─────────────────┐
│   LSP Server    │     │ Doctor Command  │
└────────┬────────┘     └────────┬────────┘
         │                       │
         └───────────┬───────────┘
                     ↓
         ┌───────────────────────┐
         │  Validation Core      │
         │  (txtx-core)          │
         └───────────────────────┘
                     ↓
         ┌───────────────────────┐
         │   Addon System        │
         │  (txtx-addon-kit)     │
         └───────────────────────┘
```

### Validation Pipeline

```text
Runbook Content
      ↓
HCL Parsing (hcl-edit)
      ↓
Syntax Validation
      ↓
Semantic Validation
      ↓
Manifest Validation (if available)
      ↓
Doctor Rules (enhanced checks)
      ↓
Formatted Output (Terminal/JSON/LSP)
```

## Key Technical Decisions

### 1. **Integrated Design**

- **Decision**: Both doctor and LSP are integrated into the main CLI
- **Rationale**: Simpler distribution, shared validation logic, no separate binaries

### 2. **Shared Validation Core**

- **Decision**: Extract validation logic to `txtx-core` for reuse
- **Rationale**: Consistency between doctor and LSP, easier maintenance

### 3. **Synchronous LSP Design**

- **Decision**: Follow rust-analyzer's synchronous pattern
- **Rationale**: Simpler implementation, predictable performance, proven approach

### 4. **Manifest-First Validation**

- **Decision**: Require environment specification for full validation
- **Rationale**: Prevents false confidence from partial validation

## Developer Experience Improvements

### 1. **Build and Test Aliases**

New cargo aliases to avoid supervisor UI build issues and make testing more convenient:

```toml
# Build without supervisor dependencies
cargo build-cli           # Debug build
cargo build-cli-release   # Release build

# Granular test commands
cargo test-cli-unit       # Unit tests only
cargo test-cli-unit-doctor # Doctor unit tests
cargo test-cli-unit-lsp   # LSP unit tests
cargo test-cli-int        # Integration tests
cargo test-cli-int-doctor # Doctor integration tests
cargo test-cli-int-lsp    # LSP integration tests
```

**Why this matters:**

- **No supervisor dependencies** - Contributors can work on core features without dealing with supervisor UI build issues
- **Faster iteration** - Test specific modules without running entire test suite
- **Clear test organization** - Unit vs integration tests are clearly separated

### 2. **Structured Test Writing with RunbookBuilder**

The new `txtx-test-utils` crate provides a fluent API for writing maintainable tests:

```rust
// Old way - string concatenation
let content = r#"
addon "evm" "ethereum" {
    network_id = 1
}
action "deploy" "evm::deploy_contract" {
    contract = "./Token.sol"
}
"#;

// New way - type-safe builder
let result = RunbookBuilder::new()
    .addon("evm", vec![("network_id", "1")])
    .action("deploy", "evm::deploy_contract")
        .input("contract", "./Token.sol")
    .validate();
```

**Benefits:**

- **Type-safe** - Compiler catches errors in test construction
- **Refactorable** - IDE can rename across all tests
- **Readable** - Clear structure without string manipulation
- **Reusable** - Common patterns can be extracted

### 3. **Flexible Validation Modes**

Tests can choose appropriate validation levels:

```rust
// Quick syntax validation (no manifest needed)
builder.validate()  // HCL validation only

// Full validation with manifest
builder
    .with_environment("production", vec![("API_KEY", "$KEY")])
    .set_current_environment("production")
    .validate()  // Full manifest validation
```

### 4. **Test Organization Pattern**

Clear separation of test types:

```text
crates/txtx-cli/
├── src/                 # Source code
│   └── cli/
│       ├── doctor/     
│       │   └── tests/  # Unit tests close to code
│       └── lsp/
│           └── tests/  # Unit tests close to code
└── tests/              # Integration tests
    ├── doctor_tests_builder.rs
    └── lsp_tests_builder.rs
```

## Supporting Infrastructure

### 1. **Test Utilities** (`txtx-test-utils`)

A new crate providing test infrastructure for validation-focused testing:

- **RunbookBuilder** - Fluent API for building test runbooks (focused on validation scenarios)
- **SimpleValidator** - Lightweight validation for tests (uses core validation without full execution)
- **Addon Registry** - Centralized addon loading for validation
- **Assertion Helpers** - Structured validation testing
- **TestHarness** - The existing execution harness moved from `txtx-core` (for full runbook execution testing)

**Important distinction:**

- **RunbookBuilder + SimpleValidator**: For testing validation logic (syntax, semantics, manifest validation)
- **TestHarness**: For testing full runbook execution with mocked blockchain responses

Example:

```rust
// Validation testing with RunbookBuilder
let result = RunbookBuilder::new()
    .action("deploy", "evm::deploy_contract")
    .validate();  // Just validates, doesn't execute

// Execution testing with TestHarness
let harness = TestHarness::new(...);
harness.start_runbook(runbook, addons);
harness.expect_action_item_request(...);  // Test actual execution
```

### 2. **Common Utilities** (`txtx-cli/src/cli/common`)

Shared infrastructure for CLI commands:

- **Addon Registry** - Loads all blockchain addons
- **Specification Extraction** - Extracts command specs from addons

### 3. **Enhanced Error Types**

New error types with rich context:

```rust
pub struct ValidationError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub context: Option<String>,
    pub suggestion: Option<ValidationSuggestion>,
    pub documentation_link: Option<String>,
}
```

## File Structure

### New Directories

```text
crates/txtx-cli/src/cli/
├── doctor/                 # Doctor command implementation
│   ├── analyzer/          # Validation rules and logic
│   ├── formatter/         # Output formatters
│   └── tests/            # Doctor-specific tests
├── lsp/                   # LSP server implementation
│   ├── handlers/         # Request handlers
│   ├── workspace/        # State management
│   ├── validation/       # Validation adapters
│   └── tests/           # LSP tests
└── common/               # Shared utilities

crates/txtx-core/src/
└── validation/           # Core validation logic
    ├── hcl_validator.rs  # HCL-based validation
    ├── manifest_validator.rs # Manifest validation
    └── doctor_rules.rs   # Validation rules

crates/txtx-test-utils/   # New test utilities crate

vscode-extension/         # VSCode extension
```

## Usage Examples

### Doctor Command

```bash
# Basic validation
txtx doctor

# Validate specific runbook
txtx doctor deploy

# With environment and inputs
txtx doctor deploy -e production --input api_key=$API_KEY

# JSON output for CI
txtx doctor --format json > validation-results.json
```

### LSP with VSCode

1. Install the extension from `vscode-extension/`
2. Open a `.tx` file
3. Get real-time validation, completions, and hover docs
4. Ctrl+Click to go to definitions

## Testing

### Test Coverage

Both features include extensive test suites:

**Doctor Tests:**

- Unit tests for each validation rule
- Integration tests with sample runbooks  
- Multi-file runbook validation tests
- Output format tests

**LSP Tests:**

- Protocol handling tests
- Handler functionality tests
- Workspace state management tests
- Multi-file support tests

### New Testing Patterns

With the RunbookBuilder API, tests are now more maintainable:

```rust
#[test]
fn test_undefined_signer_reference() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![])
        .action("transfer", "evm::send_transaction")
            .input("signer", "signer.missing")  // Undefined signer
        .validate();
    
    assert_validation_error!(result, "undefined signer 'missing'");
}

#[test] 
fn test_environment_variable_resolution() {
    let result = RunbookBuilder::new()
        .addon("evm", vec![])
        .action("deploy", "evm::deploy_contract")
            .input("rpc_url", "env.RPC_URL")
        .with_environment("test", vec![("RPC_URL", "http://localhost:8545")])
        .set_current_environment("test")
        .validate();
    
    assert_validation_passes!(result);
}
```

## Documentation

New documentation added:

- `crates/txtx-cli/src/cli/doctor/README.md` - Doctor implementation guide
- `crates/txtx-cli/src/cli/lsp/README.md` - LSP implementation guide
- `docs/VALIDATION_ARCHITECTURE.md` - Validation system architecture
- `docs/developer/doctor-architecture.md` - Doctor command internals
- `docs/developer/lsp-architecture.md` - LSP implementation details
- `docs/user/doctor-guide.md` - Doctor user guide
- `docs/user/lsp-guide.md` - LSP/IDE setup guide

## Impact

### For Users

- **Better Developer Experience** - Real-time feedback in IDE
- **Confidence in Runbooks** - Validate before deployment
- **Clear Error Messages** - Actionable suggestions and fixes
- **Professional Tooling** - On par with modern development tools

### For txtx Development

- **Maintainable Codebase** - Shared validation logic
- **Extensible Architecture** - Easy to add new validation rules
- **Better Testing** - Test utilities
- **Future-Proof Design** - Ready for additional IDE features

## Future Enhancements

### Doctor Command Capabilities

- Auto-fix capabilities for common issues
- Custom rule plugins
- Performance profiling
- Security scanning

### LSP Features

- Code actions (quick fixes)
- Find all references
- Rename refactoring
- Code lens (inline hints)

### Test Infrastructure Integration

The RunbookBuilder and TestHarness could be integrated to provide a unified testing experience:

```rust
// Future: Use RunbookBuilder for both validation AND execution testing
let harness = RunbookBuilder::new()
    .addon("evm", vec![("network_id", "1")])
    .action("deploy", "evm::deploy_contract")
        .input("contract", "Token.sol")
    .validate()           // First validate
    .into_test_harness()  // Then convert to execution harness
    .execute()?;

// Test the execution flow
harness.expect_action_item_request(...);
harness.send_mock_response(...);
```

**Benefits of this integration:**

- **Unified API** - Single fluent interface for all testing scenarios
- **Less boilerplate** - No manual RunbookSources construction
- **Progressive testing** - Validate structure before testing execution
- **Better discoverability** - One API to learn instead of two
- **Type safety** - Builder ensures correct construction before execution

This would make the testing experience more cohesive and reduce the learning curve for writing both validation and execution tests.

## Summary

This PR introduces two major features that work together to provide a professional development experience for txtx users. The doctor command offers validation with clear, actionable feedback, while the LSP integration brings txtx into modern IDEs with real-time assistance.

Additionally, significant improvements to the developer experience make it easier for contributors to work on txtx:

- **Build aliases** eliminate supervisor UI dependency issues
- **Granular test commands** allow focused testing of specific modules
- **RunbookBuilder API** makes tests more maintainable and type-safe
- **Clear test organization** separates unit and integration tests

These changes establish txtx as both a professional tool for users and a maintainable project for contributors.
