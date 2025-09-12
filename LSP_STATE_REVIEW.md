# txtx LSP Implementation State Review

## Current State: INCOMPLETE / STALLED ⚠️

The LSP implementation for txtx is in an incomplete state and appears to be stalled. The codebase shows signs of being adapted from a Clarity (Stacks smart contract) LSP implementation but the conversion to txtx is unfinished.

## Key Issues

### 1. Missing CLI Integration ❌
- The CLI declares `mod lsp;` but the `lsp.rs` file doesn't exist
- Running `txtx lsp` will fail with a module not found error
- No actual LSP server startup code

### 2. Mixed Clarity/txtx Code 🔄
- 230+ references to Clarity throughout the LSP codebase
- Completion handlers still reference Clarity functions like `SetVar`, `FetchVar`, `MintToken`
- Type system references Clarity types instead of txtx constructs

### 3. Incorrect Language Assumptions ⚠️
- The LSP assumes a Lisp-like syntax (S-expressions) from Clarity
- txtx uses HCL-like syntax which is fundamentally different
- Parser integration uses wrong AST structure

## What Currently Exists

### Infrastructure ✅
```
crates/txtx-lsp/
├── src/
│   ├── common/
│   │   ├── backend.rs         # Core LSP backend logic
│   │   ├── state.rs           # Editor state management (38KB!)
│   │   └── requests/          # LSP request handlers
│   │       ├── completion.rs  # Clarity completions (needs rewrite)
│   │       ├── definitions.rs # Go-to-definition (needs adaptation)
│   │       ├── hover.rs       # Hover provider (minimal)
│   │       └── ...
│   └── vsce_bridge.rs        # VS Code WASM bridge
```

### Capabilities Declared
- ✅ Text synchronization
- ⚠️ Completion (Clarity-specific, needs rewrite)
- ⚠️ Hover (minimal implementation)
- ⚠️ Go-to-definition (needs txtx adaptation)
- ⚠️ Document symbols (disabled by default)
- ⚠️ Signature help (Clarity-specific)

### WASM Support
- Built for VS Code extension via WASM
- Uses wasm-bindgen for JavaScript interop
- Has VS Code bridge implementation

## What Needs to Be Done

### Phase 1: Remove Clarity Code 🧹
1. Remove all Clarity-specific completions
2. Remove S-expression parsing logic
3. Remove Clarity type system references
4. Clean up imports and dependencies

### Phase 2: Integrate Tree-sitter Parser 🌳
1. Use the tree-sitter-txtx parser we just enhanced
2. Parse `.tx` files into proper AST
3. Build symbol tables from AST
4. Track construct definitions and references

### Phase 3: Implement txtx-Specific Features 🚀

#### Completions
- **Addon types**: `evm::`, `stacks::`, `bitcoin::`, etc.
- **Action types**: `deploy`, `send_eth`, `call_contract`
- **Signer types**: `secret_key`, `ledger`, `web_wallet`
- **References**: `input.`, `variable.`, `action.`, `signer.`, `flow.`
- **Functions**: `evm::encode_calldata()`, `stacks::cv_uint()`, etc.

#### Diagnostics
- Integrate doctor command validation
- Show undefined references as errors
- Validate flow attribute consistency
- Check action output fields

#### Navigation
- Go-to-definition for variables/actions/signers
- Find all references
- Symbol outline (flows, actions, variables)

#### Hover Information
- Show action specifications (inputs/outputs)
- Display variable values where known
- Link to documentation

### Phase 4: CLI Integration 🔧
1. Implement proper `lsp.rs` in CLI
2. Start LSP server on stdio
3. Handle initialization handshake
4. Route requests to txtx-lsp handlers

### Phase 5: VS Code Extension 📝
1. Create proper extension manifest
2. Package WASM build of LSP
3. Implement language client
4. Add syntax highlighting via tree-sitter grammar

## Estimated Effort

### Minimum Viable LSP
- **Remove Clarity code**: 2-3 days
- **Basic tree-sitter integration**: 3-4 days
- **Minimal completions**: 2-3 days
- **CLI integration**: 1-2 days
- **Testing**: 2-3 days

**Total: ~2-3 weeks for basic LSP**

### Full-Featured LSP
- **Complete completions**: 1 week
- **Diagnostics integration**: 1 week
- **Navigation features**: 1 week
- **VS Code extension**: 1 week
- **Documentation**: 3-4 days

**Total: ~6-8 weeks for production-ready LSP**

## Recommendation

The LSP implementation needs significant work to be functional. The current state suggests it was:
1. Originally built for Clarity/Stacks
2. Partially adapted for txtx
3. Abandoned before completion

### Options:
1. **Complete the implementation** - Significant effort but provides excellent DX
2. **Start fresh** - Use tower-lsp or lsp-server crate for cleaner architecture
3. **Minimal implementation** - Just completions and diagnostics via doctor
4. **Defer LSP** - Focus on tree-sitter grammar for syntax highlighting only

Given that we now have:
- ✅ Complete tree-sitter grammar
- ✅ Robust doctor command for validation
- ✅ AST parser via txtx-parser

The foundation is ready for a proper LSP implementation. The main decision is whether to salvage the existing code or start fresh with a txtx-first design.

## Quick Wins

If we want quick improvements without full LSP:
1. **Tree-sitter syntax highlighting** - Works today in Neovim/Helix
2. **Doctor command integration** - Can be used by editors for diagnostics
3. **Simple completions** - Could be generated from addon specifications

## Files to Review

Key files that show the current state:
- `crates/txtx-lsp/src/common/requests/completion.rs` - Shows Clarity bias
- `crates/txtx-lsp/src/common/state.rs` - Complex state management
- `crates/txtx-lsp/src/vsce_bridge.rs` - VS Code integration approach
- `crates/txtx-cli/src/cli/mod.rs:365` - Missing LSP entry point