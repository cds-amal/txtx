# How Doctor Command Uses the Core Parser

## Overview
The doctor command performs comprehensive validation of txtx runbook files using a multi-layered approach that combines Tree-sitter parsing with visitor-pattern validation.

## Architecture Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                     User runs: txtx doctor                       │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                  doctor/mod.rs::run_doctor()                     │
│  - Locates runbook files (from manifest or direct path)         │
│  - Reads file content from disk                                 │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│           doctor/mod.rs::analyze_runbook_with_context()         │
│  - Initiates parsing and validation pipeline                    │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│              txtx_parser::parse() [CORE PARSER]                 │
│  1. Creates Tree-sitter parser instance                         │
│  2. Sets tree-sitter-txtx language grammar                      │
│  3. Parses source text into Tree-sitter AST                    │
│  4. Converts Tree-sitter nodes to txtx AST structures           │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Parse Result Handling                         │
│  - Success: Returns Runbook AST structure                       │
│  - Failure: Returns ParseError with location info               │
└─────────────────────────────────────────────────────────────────┘
                                    │
                        ┌───────────┴───────────┐
                        │    Parse Success      │
                        ▼                        ▼
┌─────────────────────────────┐    ┌──────────────────────────────┐
│  Parse Error Path           │    │  Validation Path              │
│  - Create DoctorError       │    │  - Create ValidationVisitor  │
│  - Add to result.errors     │    │  - Walk AST with visitor     │
│  - Include line/column info │    │  - Validate all constructs   │
└─────────────────────────────┘    └──────────────────────────────┘
                                                │
                                                ▼
┌─────────────────────────────────────────────────────────────────┐
│     parser_validator::ValidationVisitor (Visitor Pattern)        │
│  Implements RunbookVisitor trait to traverse AST:               │
│  - visit_runbook(): Entry point, sets up validation context     │
│  - visit_action(): Validates action blocks and types            │
│  - visit_output(): Validates output blocks                      │
│  - visit_flow(): Validates flow blocks and their inputs         │
│  - visit_signer(): Validates signer blocks                      │
│  - visit_variable(): Validates variable declarations            │
│  - visit_expression(): Validates references and expressions     │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│              Validation Checks Performed                         │
│  1. Reference validation (action.foo.bar exists)                │
│  2. Type checking (field access is valid for action type)       │
│  3. Input validation (required inputs are provided)             │
│  4. Dependency ordering (no forward references)                 │
│  5. Addon specification matching                                │
│  6. Flow input validation                                       │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                    DoctorResult Collection                       │
│  - Errors: Parse failures, undefined references, type errors    │
│  - Warnings: Best practice violations, deprecations             │
│  - Suggestions: Improvements and documentation links            │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Output Formatting                             │
│  - Pretty: Human-readable with colors and context               │
│  - JSON: Machine-readable for CI/CD integration                 │
│  - GitHub: Formatted for GitHub Actions annotations             │
└─────────────────────────────────────────────────────────────────┘
```

## Key Components

### 1. Tree-sitter Integration (`txtx-parser`)
- **Grammar Definition**: `crates/tree-sitter-txtx/grammar.js` defines the txtx language syntax
- **Parser**: Uses Tree-sitter's incremental parsing for efficiency
- **AST Conversion**: Converts Tree-sitter nodes to Rust AST structures

### 2. Core Parser (`txtx_parser::parse`)
```rust
pub fn parse(source: &str) -> Result<Runbook, ParseError> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_txtx::language())?;
    let tree = parser.parse(source, None)?;
    convert_node(&tree.root_node(), source)
}
```

### 3. AST Structures (`txtx-parser/src/ast.rs`)
- `Runbook`: Top-level container for all blocks
- `ActionBlock`: Represents action definitions
- `OutputBlock`: Represents output definitions
- `Expression`: Represents value expressions and references
- `FlowBlock`, `SignerBlock`, `VariableDeclaration`: Other constructs

### 4. Visitor Pattern (`parser_validator.rs`)
The `ValidationVisitor` implements the `RunbookVisitor` trait to:
- Traverse the AST in a structured manner
- Build context during traversal (action types, variable definitions)
- Perform validation at each node
- Collect errors, warnings, and suggestions

### 5. Validation Types
- **Structural**: Valid syntax and block structure
- **Semantic**: References exist, types match
- **Contextual**: Inputs match manifest, environment variables exist
- **Best Practices**: Naming conventions, deprecated features

## Benefits of This Architecture

1. **Separation of Concerns**
   - Parsing is separate from validation
   - Tree-sitter handles syntax, doctor handles semantics

2. **Incremental Parsing**
   - Tree-sitter can reparse only changed portions
   - Enables future LSP optimizations

3. **Rich Error Information**
   - Tree-sitter provides accurate line/column info
   - Visitor pattern allows contextual error messages

4. **Extensibility**
   - New validation rules can be added to visitor
   - Grammar changes are isolated to tree-sitter-txtx

5. **Performance**
   - Tree-sitter is highly optimized C library
   - Single-pass visitor pattern for validation

## Example Validation Flow

```rust
// 1. Parse the runbook
let runbook = parse(content)?;

// 2. Create visitor with result collector
let mut result = DoctorResult::new();
let mut visitor = ValidationVisitor::new(&mut result, file_path);

// 3. Walk the AST
visitor.visit_runbook(&runbook);

// 4. Visitor validates as it walks:
//    - Collects action definitions
//    - Validates references
//    - Checks types against addon specs
//    - Reports errors with context

// 5. Return collected results
result
```

## Integration with Addon System

The doctor command leverages the addon system for validation:
1. Loads all available addons via `get_available_addons()`
2. Extracts `CommandSpecification` for each action
3. Validates action fields against specifications
4. Provides documentation links for errors

This tight integration ensures doctor's validation matches runtime behavior.