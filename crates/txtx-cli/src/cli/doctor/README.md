# txtx Doctor Command

The doctor command is a static analysis tool for txtx runbooks that catches common configuration and syntax errors before runtime.

## Overview

The doctor command validates:
- Action output references (e.g., checking if `action.transfer.from` actually exists)
- Input references against manifest environments
- Signer configurations
- General syntax correctness

## Architecture

### Core Components

#### `mod.rs`
Main implementation containing:
- `run_doctor()` - Entry point that handles manifest loading and runbook discovery
- `analyze_runbook_with_context()` - Core analysis function with manifest context
- `validate_inputs_against_manifest()` - Validates input references have values
- `check_output_field_exists()` - Verifies action outputs exist
- `display_results()` - Formats and displays errors/warnings/suggestions

#### `parser_validator.rs`
(Currently unused) Contains experimental pattern-based validation for cases where full parsing isn't available.

### Key Features

1. **Manifest-Aware Validation**
   - Loads txtx.yml to understand available environments
   - Validates input references have corresponding values
   - Warns about unused environment variables

2. **Action Output Validation**
   - Uses addon specifications to know what outputs each action provides
   - Special handling for common mistakes (e.g., `send_eth` only outputs `tx_hash`)
   - Provides documentation links for each action type

3. **Smart Error Messages**
   - Context-aware messages explaining what went wrong
   - Suggestions for how to fix issues
   - Documentation links to relevant addon pages

## Implementation Details

### Parser Integration
Uses `txtx-parser` (tree-sitter based) to:
- Parse runbook files into AST
- Extract all references (input.*, action.*, signer.*, etc.)
- Traverse nested expressions in outputs

### Addon Integration
Queries addon system for action specifications:
- Gets list of available actions from each addon
- Retrieves input/output specifications
- Uses this to validate references

### Error Reporting
Three levels of issues:
- **Errors**: Must be fixed (e.g., non-existent output field)
- **Warnings**: Should be addressed (e.g., unused environment variable)
- **Suggestions**: Helpful tips (e.g., how to get transaction details)

## Usage Examples

```bash
# Check all runbooks in manifest
txtx doctor

# Check specific runbook from manifest
txtx doctor my_runbook

# Check with specific environment
txtx doctor --env production my_runbook

# Check a file directly
txtx doctor ./path/to/runbook.tx
```

## Common Issues Detected

### 1. Non-Existent Action Outputs
```
Error: Field 'from' does not exist on action 'transfer' (evm::send_eth). 
The send_eth action only outputs: tx_hash
```

### 2. Missing Input Values
```
Error: Input 'input.private_key' is not defined in environment 'default'
Add 'private_key' to the 'default' environment in your txtx.yml file
```

### 3. Unused Environment Variables
```
Warning: Environment variable 'unused_var' is defined but not used in this runbook
```

## Future Enhancements

- [ ] Line number reporting (requires parser enhancement)
- [ ] Variable type checking
- [ ] Cross-reference validation (e.g., signer.X used before defined)
- [ ] Action dependency validation
- [ ] Module import validation
- [ ] Custom validation rules via plugins