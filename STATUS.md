# txtx Project Status

## Recent Architectural Decisions

### ADR-001: Eliminate txtx-lsp-server Crate (Implemented)
- **Status**: ✅ Completed
- **Date**: 2024-12-12
- **Impact**: Simplified LSP architecture by moving backend directly into txtx-cli
- **Result**: Removed unnecessary crate indirection, cleaner codebase

### ADR-002: Integrate HCL Validation into LSP (Implemented)
- **Status**: ✅ Completed
- **Date**: 2025-01-15
- **Impact**: Now leveraging HCL parser's native diagnostics in LSP
- **Implementation**: 
  - Created `hcl_diagnostics.rs` for HCL diagnostic extraction
  - Added `hcl_converter.rs` for HCL-to-LSP conversion
  - Integrated HCL validation into LSP diagnostics pipeline
  - Added cargo test aliases: `test-hcl-diagnostics`, `test-lsp-validation`

## Current Architecture

### LSP Implementation
- **Location**: `txtx-cli/src/cli/lsp/`
- **Message Protocol**: Using `lsp-server` crate (v0.7.6) from crates.io
- **Key Components**:
  - `mod.rs` - Main message loop and request routing
  - `handlers/` - Request handlers for completion, hover, diagnostics, etc.
  - `workspace/` - Shared state management
  - `validation/` - Current validation implementation (to be enhanced with HCL diagnostics)

### Validation Architecture
- **Current State**: 
  - HCL parser provides rich diagnostics but they're not fully utilized
  - Custom validation duplicates some HCL parser capabilities
  - Doctor command has the most comprehensive validation
  
- **Current State** (implemented per ADR-002):
  - HCL parser diagnostics are now the primary source of syntax errors
  - Custom validation layer handles txtx-specific rules only
  - Unified validation pipeline usable by CLI, LSP, and tests

## Active Development Areas

### 1. ~~HCL Diagnostic Integration~~ ✅ Completed
- Diagnostic information is now extracted from HCL parser
- HCL diagnostics are mapped to LSP diagnostic format
- Suggestions and fix hints from HCL are preserved

### 2. Validation Pipeline Refactoring
- Move manifest validation from doctor to core
- Create reusable validation components
- Enable comprehensive validation in test utils

### 3. Test Infrastructure
- Enhanced RunbookBuilder with validation modes
- Doctor-like validation in test utils
- Comprehensive test coverage for all validation scenarios

## Key Files and Locations

### LSP Implementation
- `/crates/txtx-cli/src/cli/lsp/` - Main LSP implementation
- `/crates/txtx-cli/src/cli/lsp/handlers/` - Request handlers
- `/crates/txtx-cli/src/cli/lsp/validation/` - Validation logic

### Core Components
- `/crates/txtx-core/src/` - Core parsing and execution
- `/crates/txtx-addon-kit/` - Addon framework
- `/crates/txtx-test-utils/` - Testing utilities

### Documentation
- `/doc/adr/` - Architecture Decision Records
- `/CLAUDE.md` - AI assistant guidance
- `/LSP_ARCHITECTURE.md` - LSP implementation details

## Testing Status

### Current Test Coverage
- ✅ Core functionality tests passing
- ✅ LSP protocol tests passing
- ✅ Doctor validation tests passing
- 🟡 Some validation tests ignored pending refactoring

### Test Infrastructure
- Using `txtx-test-utils` for integration testing
- Enhanced RunbookBuilder for validation testing
- Comprehensive LSP test suite

## Next Steps

1. ~~**Implement ADR-002**: Integrate HCL diagnostics into LSP~~ ✅ Completed
2. **Refactor Validation**: Create unified validation pipeline in core
3. **Enable Ignored Tests**: Fix circular dependency issues
4. **Improve Documentation**: Update architecture docs to reflect current state
5. **Clean up remaining tower-lsp references**: Remove any lingering references to the old architecture

## Dependencies and Versions

- **Rust**: 1.84.0 (see rust-toolchain)
- **lsp-server**: 0.7.6 (message protocol)
- **lsp-types**: 0.94.0 (LSP type definitions)
- **HCL parser**: Custom implementation in txtx-core

## Known Issues

1. **Circular Dependencies**: Test utils can't directly use CLI validation
2. **Diagnostic Duplication**: Some errors reported by both HCL and custom validation
3. **Limited HCL Integration**: Not fully leveraging HCL parser capabilities

## Recent Changes

- Eliminated txtx-lsp-server crate (ADR-001)
- Documented HCL validation integration plan (ADR-002)
- Enhanced test infrastructure with validation modes
- Improved doctor validation coverage