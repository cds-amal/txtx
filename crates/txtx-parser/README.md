# txtx-parser

Tree-sitter based parser and AST manipulation library for the txtx runbook language.

## Overview

This crate provides:
- Full tree-sitter grammar for txtx syntax
- Type-safe AST representation
- Builder API for programmatic AST construction
- Visitor pattern for AST traversal
- Transform utilities for AST manipulation
- Renderer to convert AST back to txtx source

## Usage

### Parsing txtx Source

```rust
use txtx_parser::parse;

let source = r#"
addon "evm" {
    chain_id = 1
    rpc_api_url = "https://mainnet.infura.io"
}

action "deploy" "evm::deploy_contract" {
    signer = signer.deployer
    contract = "MyContract"
}
"#;

let runbook = parse(source)?;
```

### Building AST Programmatically

```rust
use txtx_parser::{builder::RunbookBuilder, ast::Expression};

let runbook = RunbookBuilder::new()
    .addon("evm")
        .chain_id(Expression::number(1))
        .rpc_url(Expression::string("https://mainnet.infura.io"))
        .done()
    .signer("deployer", "evm::secret_key")
        .secret_key(Expression::input_ref("private_key"))
        .done()
    .action("transfer", "evm::send_eth")
        .signer("deployer")
        .recipient_address(Expression::input_ref("recipient"))
        .amount(Expression::number(1000000000000000000.0))
        .confirmations(1)
        .done()
    .output("tx_hash")
        .value(Expression::action_ref("transfer", "tx_hash"))
        .description("Transaction hash")
        .done()
    .build();
```

### Transforming AST

```rust
use txtx_parser::{transform::{RunbookTransform, InputSubstitution}, ast::Expression};

// Replace input references with concrete values
let mut transform = InputSubstitution::new();
transform.add_input("private_key".to_string(), Expression::string("0x..."));
transform.add_input("recipient".to_string(), Expression::string("0x123..."));

let mut runbook = parse(source)?;
transform.transform_runbook(&mut runbook);
```

### Visiting AST Nodes

```rust
use txtx_parser::visitor::RunbookVisitor;

struct ActionTypeCollector {
    action_types: Vec<String>,
}

impl RunbookVisitor for ActionTypeCollector {
    fn visit_action(&mut self, action: &ActionBlock) {
        self.action_types.push(action.action_type.clone());
    }
}

let mut collector = ActionTypeCollector { action_types: vec![] };
collector.visit_runbook(&runbook);
```

### Rendering to Source

```rust
use txtx_parser::renderer::RunbookRenderer;

let mut renderer = RunbookRenderer::new();
let source = renderer.render(&runbook);
println!("{}", source);
```

## Tree-sitter Grammar

The grammar is defined in `grammar.js` and supports:
- Addon blocks with configuration
- Signer declarations
- Action invocations
- Output definitions
- Variable declarations
- Comments (line and block)
- All expression types (strings, numbers, booleans, references, arrays, objects, function calls)

## Development

### Building the Grammar

```bash
cd crates/tree-sitter-txtx
npm install
npm run build
```

### Running Tests

```bash
cargo test --package txtx-parser
```

The test suite includes:
- Grammar parsing tests
- AST builder tests
- Transform tests
- Round-trip rendering tests

## Recent Improvements

### Object Expression Support

The parser now includes enhanced support for object expressions:

```rust
use txtx_parser::ast::Expression;

// Create complex object outputs easily
let output = Expression::object(vec![
    ("tx_hash", Expression::action_ref("transfer", "tx_hash")),
    ("from", Expression::signer_field("deployer", "address")),
    ("to", Expression::string("0x742d35Cc...")),
    ("amount", Expression::number(1000000000000000000.0)),
    ("timestamp", Expression::string("2024-01-01T00:00:00Z")),
]);
```

### Parser Fixes

- Fixed attribute parsing to correctly handle both `config` and `content` field names in blocks
- Fixed string field parsing to properly unwrap quoted strings
- Fixed object renderer to use `=` instead of `:` for txtx syntax compatibility

These improvements enable proper support for complex object outputs and better error reporting in the txtx doctor command.