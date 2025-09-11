# Parser Parity Analysis: HCL vs Tree-sitter

## Current State: NOT IN PARITY ❌

The HCL parser (txtx-core) and tree-sitter parser (tree-sitter-txtx) are **NOT currently equivalent**. There are several missing constructs in the tree-sitter grammar that are supported by the HCL parser.

## Block Types Comparison

| Construct | HCL Parser | Tree-sitter | Status |
|-----------|------------|-------------|---------|
| `addon` | ✅ Supported | ✅ Supported | ✅ Parity |
| `action` | ✅ Supported | ✅ Supported | ✅ Parity |
| `signer` | ✅ Supported | ✅ Supported | ✅ Parity |
| `output` | ✅ Supported | ✅ Supported | ✅ Parity |
| `variable` | ✅ Supported | ✅ Supported | ✅ Parity |
| `import` | ✅ Supported | ✅ Supported | ✅ Parity |
| `input` | ✅ Supported | ✅ Supported | ✅ Parity |
| `module` | ✅ Supported | ❌ Missing | ❌ Gap |
| `flow` | ✅ Supported | ❌ Missing | ❌ Gap |
| `runbook` | ✅ Supported | ❌ Missing | ❌ Gap |

## Missing Constructs in Tree-sitter

### 1. `module` Block
**HCL Parser Location**: `crates/txtx-core/src/runbook/workspace_context.rs:213-229`
```hcl
module "runbook" {
  name = "Runbook 101"
  description = "Lorem ipsum dolor sit amet"
}
```
**Used in**: Core examples, metadata definition

### 2. `flow` Block  
**HCL Parser Location**: `crates/txtx-core/src/runbook/mod.rs:142-150`
```hcl
flow "mainnet" {
  chain_id = 1
  rpc_url = "https://mainnet.infura.io"
}
```
**Used in**: Multi-environment runbooks

### 3. `runbook` Block (Embedded Runbooks)
**HCL Parser Location**: `crates/txtx-core/src/runbook/workspace_context.rs:327-345`
```hcl
runbook "embedded_name" {
  location = "./path/to/runbook.tx"
  inputs = {
    key = "value"
  }
}
```
**Used in**: Runbook composition and embedding

## Expression/Feature Gaps

### Features in HCL but potentially missing/incomplete in Tree-sitter:

1. **Multi-line strings with HCL heredoc syntax**
   - HCL supports: `<<EOF ... EOF`
   - Tree-sitter has: `"""..."""` (Python-style)

2. **Complex traversals with index access**
   - HCL: Full traversal operator support
   - Tree-sitter: Basic implementation, may need verification

3. **Conditional expressions**
   - HCL: `condition ? true_val : false_val`
   - Tree-sitter: Not visible in grammar

4. **For expressions**
   - HCL: `[for item in list : transform]`
   - Tree-sitter: Not visible in grammar

## Impact Assessment

### High Priority (Breaks functionality)
- **`module` block**: Used in many examples for metadata
- **`flow` block**: Required for multi-environment support
- **`runbook` block**: Required for embedded runbooks feature

### Medium Priority (Limits expressiveness)
- Conditional expressions
- For expressions
- Complex traversals

### Low Priority (Style differences)
- Heredoc vs triple-quote strings

## Recommended Actions

### Immediate Actions Required

1. **Update tree-sitter grammar** (`crates/tree-sitter-txtx/grammar.js`):
```javascript
// Add to _statement choice:
$.module_block,
$.flow_block,
$.runbook_block,

// Add new rules:
module_block: $ => seq(
  'module',
  field('name', $.string),
  field('config', $.block),
),

flow_block: $ => seq(
  'flow',
  field('name', $.string),
  field('config', $.block),
),

runbook_block: $ => seq(
  'runbook',
  field('name', $.string),
  field('config', $.block),
),
```

2. **Update AST types** (`crates/txtx-parser/src/ast.rs`):
   - Add `ModuleBlock`, `FlowBlock`, `RunbookBlock` structs
   - Update `Runbook` struct to include these

3. **Update parser conversion** (`crates/txtx-parser/src/lib.rs`):
   - Add conversion functions for new block types
   - Update `convert_node` to handle new cases

### Testing Strategy

1. **Create parity test suite**:
   - Collect all `.tx` files from the project
   - Parse with both HCL and tree-sitter
   - Compare AST structure (not just success/failure)

2. **Continuous validation**:
   - Add CI check that runs both parsers on all test files
   - Fail if tree-sitter cannot parse what HCL can

3. **Grammar change protocol**:
   - When HCL grammar changes, require corresponding tree-sitter update
   - Document in CONTRIBUTING.md

## Maintenance Protocol

To maintain parity going forward:

1. **Before modifying HCL parser**:
   - Document the change in this file
   - Create test case showing new syntax

2. **After HCL parser change**:
   - Update tree-sitter grammar
   - Update AST types and conversion
   - Run parity tests

3. **Regular audits**:
   - Weekly: Run parity test suite
   - Monthly: Review new `.tx` files for unsupported constructs
   - Quarterly: Full grammar comparison audit

## Current Test Coverage

Files that would fail tree-sitter parsing due to missing constructs:
- `/examples/core/simple-project/run/main.tx` - uses `module`
- Any files using `flow` blocks for multi-environment support
- Any files using embedded `runbook` blocks

## Conclusion

The tree-sitter grammar is **incomplete** and needs immediate updates to support all HCL constructs currently in use. The missing `module`, `flow`, and `runbook` blocks are actively used in the codebase and their absence breaks compatibility.