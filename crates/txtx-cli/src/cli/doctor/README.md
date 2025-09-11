# txtx Doctor Command

The doctor command is a static analysis tool for txtx runbooks that catches common configuration and syntax errors before runtime.

## Overview

The doctor command validates:
- Action output references (e.g., checking if `action.transfer.from` actually exists)
- Input references against manifest environments (with line-accurate error reporting)
- Environment inheritance (global values inherited by specific environments)
- Signer configurations
- General syntax correctness

## Output Formats

The doctor command supports multiple output formats for different use cases:

### Format Options
- `--format auto` (default): Auto-detects based on output context
- `--format pretty`: Human-readable with colors and detailed context
- `--format quickfix`: Single-line format for editor integration (Vim/Neovim)
- `--format json`: Machine-readable format for tooling

### Environment Variables
- `TXTX_DOCTOR_FORMAT`: Set default format (e.g., `export TXTX_DOCTOR_FORMAT=quickfix`)

## Architecture

### Core Components

#### `mod.rs`
Main implementation containing:
- `run_doctor()` - Entry point that handles manifest loading and runbook discovery
- `analyze_runbook_with_context()` - Core analysis function with manifest context
- `validate_inputs_against_manifest_with_locations()` - Validates input references with precise line/column info
- `find_input_location()` - Searches for exact location of input references in source
- `check_output_field_exists()` - Verifies action outputs exist
- `display_results()` - Formats output based on selected format (pretty/quickfix/json)

#### `parser_validator.rs`
Visitor-pattern based validation using the AST:
- `ValidationVisitor` - Traverses the runbook AST collecting validation data
- `LocatedInputRef` - Tracks input references with their source locations
- Validates action output field access against addon specifications

### Key Features

1. **Manifest-Aware Validation**
   - Loads txtx.yml to understand available environments
   - Validates input references have corresponding values
   - Supports environment inheritance (global values inherited by specific environments)
   - CLI input overrides via `--input KEY=VALUE`

2. **Action Output Validation**
   - Uses addon specifications to know what outputs each action provides
   - Special handling for common mistakes (e.g., `send_eth` only outputs `tx_hash`)
   - Provides documentation links for each action type
   - Visitor pattern traversal for complete validation coverage

3. **Smart Error Messages**
   - Context-aware messages explaining what went wrong
   - Precise line/column information for all errors (including input validation)
   - Suggestions for how to fix issues
   - Documentation links to relevant addon pages

4. **Multiple Output Formats**
   - Pretty format: Human-readable with colors and context
   - Quickfix format: Editor-compatible single-line errors
   - JSON format: Machine-readable for tooling integration
   - Auto-detection based on output context (TTY, CI, pipes)

## Implementation Details

### Parser Integration
Uses `txtx-parser` (tree-sitter based) to:
- Parse runbook files into AST
- Extract all references (input.*, action.*, signer.*, etc.)
- Traverse nested expressions in outputs
- Visitor pattern implementation for complete AST coverage

### Location Tracking
The doctor now provides precise error locations:
- `ValidationVisitor` collects input references during AST traversal
- `find_input_location()` searches source text for exact line/column
- All errors include file:line:column for editor navigation
- Compatible with Vim/Neovim quickfix and similar editor features

### Addon Integration
Queries addon system for action specifications:
- Gets list of available actions from each addon
- Retrieves input/output specifications
- Uses this to validate references
- Documentation links generated for each action type

### Error Reporting
Three levels of issues:
- **Errors**: Must be fixed (e.g., non-existent output field)
- **Warnings**: Should be addressed (e.g., unused environment variable)  
- **Suggestions**: Helpful tips (e.g., how to get transaction details)

All errors include:
- Precise location (file:line:column)
- Context-aware error message
- Relevant documentation links
- Suggested fixes

## Usage Examples

```bash
# Check all runbooks in manifest
txtx doctor

# Check specific runbook from manifest
txtx doctor my_runbook

# Check with specific environment
txtx doctor --env production my_runbook

# Check with CLI input overrides
txtx doctor --input API_KEY=test123 --input RPC_URL=http://localhost:8545

# Check a file directly
txtx doctor ./path/to/runbook.tx

# Output in quickfix format for editor integration
txtx doctor --format quickfix

# Output as JSON for tooling
txtx doctor --format json > results.json

# Pipe to Neovim quickfix
txtx doctor --format quickfix | nvim -q -
```

## Common Issues Detected

### 1. Non-Existent Action Outputs
```
Error: Field 'from' does not exist on action 'transfer' (evm::send_eth). 
The send_eth action only outputs: tx_hash
```

### 2. Missing Input Values  
```
./runbook.tx:15:10: error: Input 'input.private_key' is not defined in environment 'default'
Add 'private_key' to the 'default' environment in your txtx.yml file
```
Note: Error now includes precise line/column for easy navigation in editors.

### 3. Unused Environment Variables
```
Warning: Environment variable 'unused_var' is defined but not used in this runbook
```

## Recent Enhancements

- [x] Line number reporting for all errors (including input validation)
- [x] Multiple output formats (pretty, quickfix, json)
- [x] Environment inheritance support
- [x] CLI input overrides via --input flag
- [x] Visitor pattern based validation
- [x] Editor integration via quickfix format

## Future Enhancements

- [ ] Variable type checking
- [ ] Cross-reference validation (e.g., signer.X used before defined)
- [ ] Action dependency validation
- [ ] Module import validation
- [ ] Custom validation rules via plugins
- [ ] Watch mode for continuous validation
- [ ] Integration with LSP for real-time validation