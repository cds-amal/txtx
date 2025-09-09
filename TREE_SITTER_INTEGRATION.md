# Tree-sitter Integration for txtx

This document describes the tree-sitter-based AST parser and manipulation system for txtx runbooks.

## Overview

We've migrated from string-based template generation to a full AST-based system using tree-sitter. This provides:

- **Robust parsing** with error recovery
- **Semantic understanding** of runbook structure
- **Format preservation** when transforming runbooks
- **Type-safe AST manipulation**
- **Powerful analysis capabilities**

## Architecture

### Components

1. **tree-sitter-txtx** (`crates/tree-sitter-txtx/`)
   - Grammar definition in `grammar.js`
   - Defines the syntax of txtx runbooks
   - Generates C parser code

2. **txtx-parser** (`crates/txtx-parser/`)
   - Rust bindings and AST types
   - Parser, builder, visitor, and transform APIs
   - Renderer to convert AST back to text

3. **AST-based templates** (`addons/evm/src/tests/test_harness/templates/ast_core.rs`)
   - Templates defined using AST builder API
   - Environment-specific transformations
   - Input substitution support

## Grammar Structure

The txtx grammar defines these key elements:

```javascript
runbook = (addon_block | signer_block | action_block | output_block | variable)*

addon_block = 'addon' string block
signer_block = 'signer' string string block  
action_block = 'action' string string block
output_block = 'output' string block

block = '{' attribute* '}'
attribute = identifier '=' expression

expression = string | number | boolean | array | object | reference | function_call
```

## AST Types

```rust
pub struct Runbook {
    pub addons: Vec<AddonBlock>,
    pub signers: Vec<SignerBlock>,
    pub actions: Vec<ActionBlock>,
    pub outputs: Vec<OutputBlock>,
    pub variables: Vec<VariableDeclaration>,
}

pub enum Expression {
    String(String),
    Number(f64),
    Boolean(bool),
    Reference(Vec<String>), // e.g., ["input", "chain_id"]
    Array(Vec<Expression>),
    Object(HashMap<String, Expression>),
    FunctionCall { name: String, args: Vec<Expression> },
    // ...
}
```

## Usage Examples

### Parsing a Runbook

```rust
use txtx_parser::RunbookParser;

let mut parser = RunbookParser::new()?;
let runbook = parser.parse(source_code)?;

// Access parsed elements
for action in &runbook.actions {
    println!("Action: {} ({})", action.name, action.action_type);
}
```

### Building a Runbook Programmatically

```rust
use txtx_parser::{builder::RunbookBuilder, ast::Expression};

let runbook = RunbookBuilder::new()
    .addon("evm")
        .chain_id(Expression::number(31337))
        .rpc_url(Expression::string("http://localhost:8545"))
        .done()
    .action("transfer", "evm::send_eth")
        .signer("test_signer")
        .amount(Expression::number(1000))
        .done()
    .build();
```

### Visiting AST Nodes

```rust
use txtx_parser::visitor::{RunbookVisitor, ActionTypeCollector};

let mut collector = ActionTypeCollector::new();
collector.visit_runbook(&runbook);
println!("Action types: {:?}", collector.action_types);
```

### Transforming AST

```rust
use txtx_parser::transform::{RunbookTransform, InputSubstitution};

let mut substitution = InputSubstitution::new();
substitution.add_input("chain_id", Expression::number(1));
substitution.transform_runbook(&mut runbook);
```

### Rendering Back to Text

```rust
use txtx_parser::renderer::RunbookRenderer;

let mut renderer = RunbookRenderer::new();
let output = renderer.render(&runbook);
```

## AST-Based Templates

Templates now use the AST builder API instead of string concatenation:

```rust
AstRunbookTemplate::new("eth_transfer", "Simple ETH transfer")
    .build(|builder| {
        builder
            .addon("evm")
                .chain_id(Expression::input_ref("chain_id"))
                .rpc_url(Expression::input_ref("rpc_url"))
                .done()
            .action("transfer", "evm::send_eth")
                .recipient_address(Expression::input_ref("recipient"))
                .amount(Expression::input_ref("amount"))
                .done()
            .build()
    })
```

Benefits over string templates:
- Type safety at compile time
- Automatic input discovery
- Environment-specific transformations
- Preserves formatting and comments

## Migration Guide

### From Regex to AST Analysis

**Before:**
```rust
let action_regex = Regex::new(r#"action\s+"\w+"\s+"evm::send_eth""#)?;
let matches = action_regex.find_iter(&content).count();
```

**After:**
```rust
let runbook = parser.parse(&content)?;
let eth_transfers = runbook.actions.iter()
    .filter(|a| a.action_type == "evm::send_eth")
    .count();
```

### From String Templates to AST Templates

**Before:**
```rust
format!(r#"action "transfer" "evm::send_eth" {{
    recipient_address = {}
    amount = {}
}}"#, recipient, amount)
```

**After:**
```rust
builder.action("transfer", "evm::send_eth")
    .recipient_address(Expression::string(recipient))
    .amount(Expression::number(amount))
    .done()
```

## Tools and Utilities

### analyze_tests_ast

Enhanced test analyzer using AST parsing:
- Accurately identifies action types
- Understands runbook structure
- Provides better recommendations

```bash
cargo run --bin analyze_tests_ast
```

### Tree-sitter CLI

Test grammar changes:
```bash
cd crates/tree-sitter-txtx
npm install
npx tree-sitter generate
npx tree-sitter test
```

## Future Enhancements

1. **IDE Support**
   - Syntax highlighting using tree-sitter queries
   - Go-to-definition for references
   - Auto-completion

2. **Advanced Transformations**
   - Runbook optimization (remove unused outputs)
   - Automatic dependency ordering
   - Cross-runbook refactoring

3. **Validation**
   - Type checking (ensure amounts are numbers)
   - Reference validation (check action.foo exists)
   - Network-specific validation

4. **Query Language**
   - Find all actions using specific signer
   - Extract all contract addresses
   - Analyze gas usage patterns

## Performance Considerations

- Tree-sitter parsing is very fast (microseconds)
- AST manipulation is memory-efficient
- Renderer preserves original formatting where possible
- Suitable for large runbook files

## Contributing

To modify the grammar:

1. Edit `grammar.js`
2. Run `npx tree-sitter generate`
3. Update test corpus
4. Run `npx tree-sitter test`
5. Update Rust bindings if needed

The AST types in `txtx-parser` should mirror the grammar structure for consistency.

## Testing

### Unit Tests

Test the parser and AST manipulation:

```bash
# Run parser tests
cargo test --package txtx-parser

# Run specific test
cargo test --package txtx-parser test_parse_simple_runbook
```

### Integration Tests

Test with the EVM addon:

```bash
# Test template generation
cargo test --package txtx-addon-network-evm test_ast_template_generation

# Test template synchronization
cargo test --package txtx-addon-network-evm test_eth_transfer_templates_are_synced

# Regenerate templates
cargo run --bin regenerate_templates --package txtx-addon-network-evm
```

### Test Harness Usage

The AST templates integrate seamlessly with the project test harness:

```rust
use crate::tests::test_harness::ProjectTestHarness;

// Create from template
let mut harness = ProjectTestHarness::for_eth_transfer()
    .with_anvil()
    .with_input("transfer_recipient_address", "0x742d35...")
    .with_input("transfer_amount", "1000000000000000000");

// Setup and execute
harness.setup().expect("Failed to setup");
let result = harness.execute_runbook().expect("Failed to execute");

// Verify outputs
assert!(result.outputs.contains_key("tx_hash"));
```

### Available Test Templates

- **Transfer Operations**
  - `eth_transfer` - Simple ETH transfer
  - `eth_transfer_named` - Transfer with named accounts
  - `batch_transfer` - Multiple transfers

- **Contract Operations**
  - `deploy_contract` - Deploy smart contract
  - `call_contract` - Call contract function
  - `monitor_events` - Monitor contract events

- **Testing Templates**
  - `transaction_cost_test` - Test gas costs
  - `gas_estimation` - Test gas estimation
  - `revert_test` - Test revert handling
  - `nonce_management` - Test nonce handling
  - `event_log_test` - Test event emission

## Complete Example

Here's a complete example showing how to create and use a custom template:

```rust
// Define template
use txtx_parser::{ast::*, builder::RunbookBuilder};
use crate::tests::test_harness::templates::ast_core::*;

pub fn custom_defi_template() -> AstRunbookTemplate {
    AstRunbookTemplate::new("defi_swap", "DeFi swap operation")
        .build(|builder| {
            builder
                .addon("evm")
                    .chain_id(Expression::input_ref("chain_id"))
                    .rpc_url(Expression::input_ref("rpc_url"))
                    .done()
                .signer("trader", "evm::secret_key")
                    .secret_key(Expression::input_ref("trader_private_key"))
                    .done()
                .action("approve", "evm::call_contract")
                    .signer("trader")
                    .contract_address(Expression::input_ref("token_address"))
                    .function_name(Expression::string("approve"))
                    .function_args(Expression::Array(vec![
                        Expression::input_ref("router_address"),
                        Expression::input_ref("amount_in"),
                    ]))
                    .done()
                .action("swap", "evm::call_contract")
                    .signer("trader")
                    .contract_address(Expression::input_ref("router_address"))
                    .function_name(Expression::string("swapExactTokensForTokens"))
                    .function_args(Expression::input_ref("swap_args"))
                    .done()
                .output("tx_hash")
                    .value(Expression::action_ref("swap", "tx_hash"))
                    .done()
                .output("amount_out")
                    .value(Expression::action_ref("swap", "return_value"))
                    .done()
                .build()
        })
}

// Register template
registry.register(AstTemplateInfo {
    name: "defi_swap".to_string(),
    description: "DeFi swap operation".to_string(),
    category: TemplateCategory::DeFi,
    required_inputs: vec![
        "chain_id".to_string(),
        "rpc_url".to_string(),
        "trader_private_key".to_string(),
        "token_address".to_string(),
        "router_address".to_string(),
        "amount_in".to_string(),
        "swap_args".to_string(),
    ],
    factory: custom_defi_template,
});

// Use in test
#[tokio::test]
async fn test_defi_swap() {
    let mut harness = ProjectTestHarness::from_template_name("defi_swap")
        .with_anvil()
        .with_input("token_address", "0x...")
        .with_input("router_address", "0x...")
        .with_input("amount_in", "1000000")
        .with_input("swap_args", "[...]");
    
    harness.setup().expect("Failed to setup");
    let result = harness.execute_runbook().expect("Failed to execute");
    
    assert!(result.outputs.contains_key("amount_out"));
}
```

## Summary

The tree-sitter integration provides a robust foundation for:

1. **Type-safe template generation** - No more string concatenation errors
2. **Semantic analysis** - Understand runbook structure deeply
3. **Powerful transformations** - Modify AST with confidence
4. **Better testing** - Templates adapt to different environments
5. **Future IDE support** - Foundation for language server

The migration from string-based to AST-based templates has made the test infrastructure more maintainable and less error-prone while laying groundwork for advanced txtx tooling.