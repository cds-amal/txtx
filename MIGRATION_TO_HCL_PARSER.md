# Migration to Unified HCL Parser

## Overview

As of September 2025, the txtx project has completed a migration to use a single, unified parser implementation based on `hcl-edit`. This eliminates the previous dual-parser architecture and simplifies the codebase significantly.

## What Changed

### Removed Components
- **Deleted `crates/txtx-parser/`** - The Tree-sitter based parser crate
- **Deleted `crates/tree-sitter-txtx/`** - The Tree-sitter grammar definition
- **Removed Tree-sitter dependencies** - No longer needed in the project

### Updated Components
- **Doctor Command** - Now uses `hcl-edit` for validation (same parser as txtx-core)
- **LSP Implementation** - Uses `hcl-edit` for parsing runbook files
- **Test Suite** - Updated to use HCL validation functions

## Benefits

1. **Single Source of Truth** - One parser means consistent behavior everywhere
2. **Reduced Maintenance** - No need to keep two parsers in sync
3. **Better Performance** - `hcl-edit` is well-optimized and battle-tested
4. **Simpler Codebase** - Removed ~5000 lines of parser-related code
5. **Consistent Validation** - Doctor command validates exactly like runtime

## Technical Details

### Parser Architecture
- **Parser Library**: `hcl-edit` (re-exported as `txtx_addon_kit::hcl`)
- **Visitor Pattern**: Used for AST traversal in validation
- **Two-Pass Validation**: 
  1. First pass collects all definitions
  2. Second pass validates references

### Key Files
- `crates/txtx-cli/src/cli/doctor/hcl_validator.rs` - HCL-based validation
- `crates/txtx-core/src/runbook/` - Core runbook parsing logic
- `crates/txtx-addon-kit/src/helpers/hcl.rs` - HCL helper utilities

## Migration Notes for Developers

If you were extending or modifying the parser:

1. **Custom Validations** - Implement using the `hcl_edit::visit::Visit` trait
2. **AST Access** - Use `hcl_edit::structure` and `hcl_edit::expr` types
3. **Expression Handling** - Use pattern matching on `Expression` enum
4. **Traversal** - Use the visitor pattern with `visit_block` and `visit_expr`

## Breaking Changes

None. The Tree-sitter parser was introduced and removed entirely within this feature branch (`feat/lsp`), so there are no breaking changes for anyone. The txtx runbook syntax remains unchanged, and all existing runbooks continue to work exactly as before.

## Future Improvements

With a unified parser, future enhancements include:
- Better error recovery and reporting
- Incremental parsing for large files
- Enhanced IDE support through the LSP
- Consistent validation across all tools