# txtx Language Server Protocol (LSP) Implementation

## Overview

The txtx LSP provides IDE support for txtx runbook files (`.tx`) and manifest files (`txtx.yml`), offering features like go-to-definition, hover information, completions, and diagnostics.

## Architecture

### Core Components

1. **`txtx-cli/src/cli/lsp/`** - Complete LSP implementation integrated in the CLI
   - `mod.rs` - LSP server entry point, message loop, and request routing
   - `backend.rs` - Synchronous LSP backend implementing all protocol handlers (~500 lines)
   - Uses `lsp-server` crate for protocol handling (same as rust-analyzer)
   - No async runtime overhead - purely synchronous for better performance
   - Direct integration - no separate crate needed (see ADR-001)
   - **Parser**: Uses `hcl-edit` (same as txtx-core) for all parsing

2. **`vscode-extension`** - VSCode client extension
   - Connects VSCode to the txtx LSP server
   - Auto-detects development binary or uses system txtx
   - Provides debugging commands and status indicators

### Key Design Decisions

- **Synchronous Architecture**: Following rust-analyzer's proven pattern, we use synchronous code instead of async/await for the LSP protocol, which is inherently request/response based.
- **lsp-server over tower-lsp**: Migrated from the unmaintained tower-lsp to rust-analyzer's actively maintained lsp-server crate.
- **Direct Protocol Control**: We handle JSON-RPC messages directly, giving us better debugging capabilities and control over the protocol.

## Features Implemented

### Core LSP Features

- ✅ **Document Synchronization** - Track open/close/change events for `.tx` and `txtx.yml` files
- ✅ **Go-to-Definition** - Jump from `input.variable_name` references to definitions in `txtx.yml`
- ✅ **Hover Information** - Show variable values and environment info on hover
- ✅ **Completions** - Auto-complete input variables after typing `input.`
- ✅ **Workspace State** - Automatically discover and index manifest files
- ✅ **Multi-Environment Support** - Search variables across all environments (default, global, buildbear, etc.)

### txtx-Specific Features

- **Manifest Discovery**: Automatically finds `txtx.yml` in parent directories when opening runbooks
- **Environment Resolution**: Searches variables in priority order: default → global → other environments
- **Proper Syntax Support**: Correctly handles `input.` (singular) syntax, not `inputs.`
- **Nested Runbook Support**: Works with runbooks in subdirectories (e.g., `runbooks/owner/test.tx`)

## Installation & Usage

### Building the LSP

```bash
# Build the txtx binary with LSP support
cd /home/amal/dev/tx/txtx
cargo build --package txtx-cli --no-default-features --features cli

# The binary will be at: target/debug/txtx
# Run LSP: target/debug/txtx lsp
```

### VSCode Extension Setup

```bash
# Install extension dependencies
cd vscode-extension
npm install

# Compile the extension
npm run compile
```

### Configuration

The extension automatically detects the development binary. You can also configure it manually in VSCode settings:

```json
{
  "txtx.lspPath": "/path/to/txtx",
  "txtx.trace.server": "verbose"  // For debugging
}
```

### Testing the LSP

1. **In VSCode**:
   - Open a txtx project with `txtx.yml` and `.tx` files
   - Put cursor on an `input.variable_name` reference
   - Press F12 for go-to-definition
   - Hover for variable information
   - Type `input.` for completions

2. **Debug Commands** (Command Palette):
   - `Txtx: Show Language Server Logs` - View all LSP communication
   - `Txtx: Test Go-to-Definition at Cursor` - Manual definition test
   - `Txtx: Restart Language Server` - Restart if needed

## Implementation Details

### Message Flow

```
VSCode ←→ stdio ←→ txtx lsp ←→ lsp-server ←→ TxtxLspBackend
```

1. VSCode sends JSON-RPC messages over stdio
2. `lsp-server` handles protocol parsing and serialization
3. Our backend processes requests synchronously
4. Responses flow back through the same path

### State Management

```rust
struct WorkspaceState {
    documents: HashMap<Url, Document>,      // Open documents
    manifests: HashMap<Url, Manifest>,      // Parsed txtx.yml files
    runbook_to_manifest: HashMap<Url, Url>, // Runbook → Manifest mapping
    environment_vars: HashMap<String, HashMap<String, String>>, // Cached variables
}
```

### Request Handlers

- **textDocument/definition**: Parses `input.variable` references and finds definitions in manifest
- **textDocument/hover**: Shows variable values and environment information
- **textDocument/completion**: Provides available input variables after `input.`
- **textDocument/didOpen**: Indexes documents and auto-discovers manifests

## Development

### Running Tests

```bash
# Run LSP backend tests
cargo test --package txtx-lsp --test backend_tests

# These tests verify:
# - Workspace state building
# - Different file opening orders
# - Go-to-definition functionality
# - Missing manifest handling
```

### Debugging the LSP

1. **Enable verbose logging** in VSCode settings:
   ```json
   { "txtx.trace.server": "verbose" }
   ```

2. **View logs**: Command Palette → "Txtx: Show Language Server Logs"

3. **Check stderr output**: The LSP logs to stderr with `eprintln!`

4. **Test protocol directly**:
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | txtx lsp
   ```

### Adding New Features

1. **Add handler in `backend_sync.rs`**:
   ```rust
   pub fn new_feature(&self, params: NewFeatureParams) -> Option<Response> {
       // Implementation
   }
   ```

2. **Route in `lsp.rs`**:
   ```rust
   "textDocument/newFeature" => {
       let params = serde_json::from_value(req.params)?;
       let result = backend.new_feature(params);
       Some(Response::new_ok(req.id, result))
   }
   ```

3. **Add tests** in `crates/txtx-lsp/tests/`

## Architecture Benefits

### Why lsp-server Instead of tower-lsp?

| Aspect | tower-lsp (old) | lsp-server (current) |
|--------|-----------------|---------------------|
| **Maintenance** | Unmaintained since 2023 | Actively maintained by rust-analyzer team |
| **Performance** | Async overhead for sync protocol | Synchronous, minimal overhead |
| **Debugging** | Abstractions hide protocol details | Direct protocol visibility |
| **Testing** | Difficult to test at protocol level | Easy protocol-level testing |
| **Dependencies** | Heavy (tokio, tower, etc.) | Minimal (just serde & channels) |
| **Proven Usage** | Limited adoption | Powers rust-analyzer (millions of users) |

### Performance Characteristics

- **No async runtime**: Eliminates tokio overhead for inherently synchronous operations
- **Direct message handling**: No tower middleware abstractions
- **Efficient state management**: RwLock for concurrent read access
- **Fast manifest parsing**: Simple YAML parsing optimized for txtx format

## Known Limitations & Future Work

### Current Limitations

- Basic YAML parsing (could use full YAML parser for complex manifests)
- No cross-file refactoring support yet
- Limited to single workspace at a time

### Planned Enhancements

- [ ] Full doctor command integration for validation
- [ ] Multi-file symbol analysis
- [ ] Code actions and quick fixes
- [ ] Formatting support
- [ ] Semantic highlighting
- [ ] Support for more editors (Neovim built-in LSP, Helix, etc.)

## Troubleshooting

### Go-to-Definition Not Working

1. Check if manifest is discovered:
   - Look for "Found manifest at:" in logs
   - Ensure `txtx.yml` exists in project root or parent directory

2. Verify correct syntax:
   - Use `input.` (singular), not `inputs.`
   - Variable must exist in manifest environments

3. Check file associations:
   - `.tx` files should open with txtx language mode
   - `txtx.yml` should be recognized

### LSP Not Starting

1. Check binary exists:
   ```bash
   ls -la /home/amal/dev/tx/txtx/target/debug/txtx
   ```

2. Test manually:
   ```bash
   /home/amal/dev/tx/txtx/target/debug/txtx lsp
   ```

3. Check VSCode output panel for errors

### Performance Issues

- The LSP is synchronous and should be very fast
- If slow, check for large manifest files
- Enable verbose logging to identify bottlenecks

## Contributing

The LSP implementation welcomes contributions! Key areas:

1. **Improve YAML parsing** - Use proper YAML parser for robustness
2. **Add more language features** - References, rename, code actions
3. **Enhance completions** - Context-aware suggestions for actions, signers
4. **Add validation** - Integrate doctor command for semantic checks

### Code Style

- Follow Rust standard style (rustfmt)
- Use synchronous code (no async/await in LSP path)
- Add tests for new features
- Document public APIs

## License

See the main txtx repository LICENSE file.