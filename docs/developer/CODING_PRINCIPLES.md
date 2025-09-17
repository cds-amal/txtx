# txtx Coding Principles & Workstyle Guide

This guide captures the architectural principles and coding patterns established during the 2024 refactoring of the doctor and LSP modules. These principles should guide all future development in the txtx codebase.

## Core Principles

### 1. Modular Architecture Over Monolithic Files

**❌ Avoid**: Single files exceeding 500 lines with mixed responsibilities  
**✅ Prefer**: Modular structure with clear separation of concerns

```console
module/
├── mod.rs           # Thin orchestrator (<200 lines)
├── config.rs        # Configuration types
├── types.rs         # Shared types
├── submodule1/      # Feature-specific logic
└── submodule2/      # Feature-specific logic
```

### 2. Trait-Based Extensibility

**❌ Avoid**: Hard-coded switch statements and if-else chains  
**✅ Prefer**: Trait-based design for extensible behavior

```rust
// Define clear trait boundaries
pub trait ValidationRule {
    fn name(&self) -> &str;
    fn validate(&self, context: &Context) -> Vec<Diagnostic>;
}

// Implement specific behaviors
struct MyRule;
impl ValidationRule for MyRule { ... }
```

**Real Examples**:

- [`ValidationRule` trait`](crates/txtx-cli/src/cli/doctor/analyzer/rules.rs:6-17) - Base trait for all validation rules
- [`Handler` trait`](crates/txtx-cli/src/cli/lsp/handlers/mod.rs:22-25) - Base trait for LSP handlers
- [`InputDefinedRule` implementation`](crates/txtx-cli/src/cli/doctor/analyzer/rules.rs:47-94) - Complete validation rule example

### 3. Composition Over Inheritance

**❌ Avoid**: Deep inheritance hierarchies or complex state machines  
**✅ Prefer**: Compose small, focused components

```rust
// Compose validators
let validator = Validator::new()
    .add_rule(InputRule::new())
    .add_rule(FlowRule::new())
    .add_rule(SecurityRule::new());
```

### 4. Explicit Over Implicit

**❌ Avoid**: Magic strings, hidden dependencies, global state  
**✅ Prefer**: Explicit dependencies, clear interfaces

```rust
// Bad: Hidden dependency
fn validate() {
    let config = CONFIG.get(); // Global state
}

// Good: Explicit dependency
fn validate(config: &Config) {
    // Use provided config
}
```

## Architectural Patterns

### 1. Handler Pattern for Request/Response

When building request/response systems (like LSP):

```rust
trait Handler {
    type Request;
    type Response;
    fn handle(&self, req: Self::Request, ctx: &Context) -> Result<Self::Response>;
}
```

### 2. Visitor Pattern for AST Traversal

When processing hierarchical data:

```rust
trait Visitor {
    fn visit_block(&mut self, block: &Block);
    fn visit_expression(&mut self, expr: &Expression);
}
```

### 3. Builder Pattern for Complex Configuration

When constructing complex objects:

```rust
WorkspaceBuilder::new()
    .manifest_path("./txtx.yml")
    .environment("production")
    .build()?
```

### 4. Adapter Pattern for Integration

When integrating different subsystems:

```rust
// Adapt doctor validation for LSP use
struct DoctorToLspAdapter;
impl LspValidator for DoctorToLspAdapter {
    fn validate(&self, doc: &Document) -> Vec<LspDiagnostic> {
        doctor_validator.validate(doc)
            .into_iter()
            .map(convert_diagnostic)
            .collect()
    }
}
```

**Real Example**: [`crates/txtx-cli/src/cli/lsp/validation/adapter.rs`](crates/txtx-cli/src/cli/lsp/validation/adapter.rs)

## Code Organization

### 1. File Structure Guidelines

- **mod.rs**: Public API and orchestration only
- **types.rs**: Shared types and traits
- **impl.rs**: Private implementation details
- **tests.rs**: Unit tests (or separate tests/ directory)

### 2. Module Boundaries

- Each module should have a single, clear purpose
- Dependencies should flow in one direction
- Circular dependencies indicate poor boundaries

### 3. Error Handling

```rust
// Define module-specific error types
#[derive(Debug, thiserror::Error)]
pub enum DoctorError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Use Result type alias
pub type Result<T> = std::result::Result<T, DoctorError>;
```

## Testing Strategy

### 1. Test Organization

```console
tests/
├── unit/           # Fast, isolated unit tests
├── integration/    # Cross-module integration tests
└── fixtures/       # Test data and examples
```

### 2. Test Principles

- **Fast**: Unit tests should run in milliseconds
- **Isolated**: Tests shouldn't depend on external state
- **Descriptive**: Test names should explain the scenario

```rust
#[test]
fn validation_rule_detects_missing_flow_attribute() {
    // Arrange
    let rule = FlowAttributeRule::new();
    let context = test_context();
    
    // Act
    let diagnostics = rule.validate(&context);
    
    // Assert
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("missing attribute"));
}
```

## Performance Considerations

### 1. Lazy Evaluation

**❌ Avoid**: Eagerly computing all possibilities  
**✅ Prefer**: Compute only what's needed

```rust
// Bad: Loads all files immediately
let all_files = load_all_files(&directory)?;

// Good: Returns iterator that loads on demand
let files = directory.files()
    .filter(|f| f.extension() == "tx")
    .map(|f| load_file(f));
```

### 2. Caching Strategy

Cache expensive computations at appropriate boundaries:

```rust
struct Workspace {
    manifests: Cache<PathBuf, Manifest>,
}
```

## Documentation Standards

### 1. Module Documentation

Every module should have a clear purpose:

```rust
//! # Doctor Module
//! 
//! Provides validation and linting for txtx runbooks.
//! 
//! ## Architecture
//! - `analyzer/`: Core validation logic
//! - `formatter/`: Output formatting
//! - `rules/`: Validation rule implementations
```

### 2. Public API Documentation

All public items need documentation:

```rust
/// Validates a runbook and returns diagnostics.
/// 
/// # Arguments
/// * `runbook` - Path to the runbook file
/// * `config` - Validation configuration
/// 
/// # Returns
/// A vector of diagnostics, empty if validation passes
pub fn validate(runbook: &Path, config: &Config) -> Vec<Diagnostic> {
```

## Refactoring Workflow

When refactoring existing code:

1. **Measure First**: Understand current state (line count, complexity)
2. **Identify Boundaries**: Find natural module boundaries
3. **Extract Incrementally**: Move one piece at a time
4. **Maintain Tests**: Keep tests passing throughout
5. **Document Changes**: Update docs and examples

## Code Review Checklist

Before submitting PRs, ensure:

- [ ] No single file exceeds 500 lines
- [ ] Clear module boundaries with single responsibilities
- [ ] Traits used for extensible behavior
- [ ] Dependencies are explicit (no global state)
- [ ] Tests pass and cover new functionality
- [ ] Documentation updated for public APIs
- [ ] Error handling uses proper types
- [ ] Performance implications considered

## Examples from the Refactoring

### Before (Monolithic)

```rust
// 1000+ line file mixing all concerns
fn doctor_command(args: Args) {
    // Parse config
    // Load files
    // Validate
    // Format output
    // ... hundreds more lines
}
```

### After (Modular)

```rust
// mod.rs - Orchestration only (from crates/txtx-cli/src/cli/doctor/mod.rs:20-40)
pub fn run_doctor(
    manifest_path: Option<String>,
    runbook_name: Option<String>,
    environment: Option<String>,
    cli_inputs: Vec<(String, String)>,
    format: DoctorOutputFormat,
) -> Result<(), String> {
    // Create and resolve configuration
    let config = DoctorConfig::new(manifest_path, runbook_name, environment, cli_inputs, format)
        .resolve_format();

    // Run the doctor analysis
    match config.runbook_name {
        Some(ref name) => run_specific_runbook(&config, name),
        None => run_all_runbooks(&config),
    }
}
```

**Key improvements demonstrated**:

- Clear separation: config, workspace, analyzer, formatter are distinct modules
- Single responsibility: main function only orchestrates
- Explicit dependencies: all parameters passed explicitly
- Testable: each component can be tested in isolation

## Metrics for Success

A well-architected module should have:

- **Orchestrator**: <200 lines
- **Components**: <300 lines each
- **Clear boundaries**: Can explain purpose in one sentence
- **Testability**: >80% test coverage achievable
- **Extensibility**: Adding features doesn't require modifying core

## Living Document

This guide is based on successful patterns from the lsp/doctor development. As we discover new patterns or improvements, this document should be updated to reflect current best practices.

Remember: **Good architecture makes the code tell a story**. Each module should have a clear narrative that any developer can follow.
