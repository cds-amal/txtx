# AST Source Location Tracking Implementation

## Current Design: Native hcl-edit Span Support

After migrating from Tree-sitter to hcl-edit, source location tracking is now handled natively by the hcl-edit parser, which provides built-in span information for all AST nodes.

### Implementation Details

#### 1. HCL Validator with Span Support
The `HclValidationVisitor` uses hcl-edit's native span tracking:

```rust
pub struct HclValidationVisitor<'a> {
    result: &'a mut DoctorResult,
    file_path: String,
    source: &'a str,  // Source text for span-to-line/column conversion
    // ... validation state fields ...
    current_block: Option<BlockContext>,
}

struct BlockContext {
    block_type: String,
    name: String,
    span: Option<std::ops::Range<usize>>,  // Native span from hcl-edit
}
```

#### 2. Span Collection from hcl-edit
When processing blocks, we capture the span directly from hcl-edit's AST:

```rust
fn process_block(&mut self, block: &Block) {
    let block_type = block.ident.value().as_str();
    
    // Get span directly from hcl-edit
    let span = block.span();
    
    self.current_block = Some(BlockContext {
        block_type: block_type.to_string(),
        name: String::new(), // Filled based on block labels
        span,
    });
    
    // Process block based on type...
}
```

#### 3. Span to Line/Column Conversion
We convert byte spans to line/column positions for error reporting:

```rust
fn span_to_position(&self, span: &std::ops::Range<usize>) -> (usize, usize) {
    let start = span.start;
    let mut line = 1;
    let mut col = 1;
    
    for (i, ch) in self.source.char_indices() {
        if i >= start {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    
    (line, col)
}
```

#### 4. Error Reporting with Precise Locations
Errors include exact line and column information:

```rust
let (line, col) = self.current_block
    .as_ref()
    .and_then(|ctx| ctx.span.as_ref())
    .map(|span| self.span_to_position(span))
    .unwrap_or((0, 0));

self.result.errors.push(DoctorError {
    message: format!("Reference to undefined action '{}'", action_name),
    file: self.file_path.clone(),
    line: if line > 0 { Some(line) } else { None },
    column: if col > 0 { Some(col) } else { None },
    context: Some("Make sure the action is defined before using it".to_string()),
    documentation_link: None,
});
```

## Key Design Benefits

### 1. **Native Parser Support**
hcl-edit provides spans as a first-class feature, eliminating the need for custom location tracking.

### 2. **Unified Parser**
The same hcl-edit parser is used by both txtx-core (runtime) and the doctor command (validation), ensuring consistency.

### 3. **Zero Overhead**
No additional data structures needed - spans are part of the AST nodes themselves.

### 4. **Precise Error Locations**
Every AST node has span information, enabling accurate error reporting for all validation issues.

### 5. **Standards Compliant**
hcl-edit follows the HCL specification exactly, ensuring compatibility with the broader HCL ecosystem.

## Result
Doctor provides precise error locations for all issues:
```
test_doctor_two_pass.tx:37:2: error[1]: Reference to undefined action 'undefined_action'
   Make sure the action is defined before using it in outputs
```

## Architecture Diagram
```
┌──────────────────────────────────────┐
│           hcl-edit Parser             │
│                                      │
│  - Parses HCL/txtx syntax            │
│  - Provides AST with native spans    │
│  - Used by both txtx-core & doctor   │
└────────────────┬─────────────────────┘
                 │ AST with spans
                 ▼
┌──────────────────────────────────────┐
│      Doctor HclValidationVisitor      │
│                                      │
│  ┌─────────────────────────────┐    │
│  │ Uses hcl-edit visitor API    │    │
│  │ Traverses AST with spans     │    │
│  │ Converts spans to line:col   │    │
│  └─────────────────────────────┘    │
│                                      │
│  Two-pass validation:                │
│  1. Collect definitions              │
│  2. Validate references              │
└──────────────────────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────┐
│         Error Output                  │
│                                      │
│  file.tx:10:5: error[1]: message    │
│     Helpful context about the error  │
└──────────────────────────────────────┘
```

## Migration from Tree-sitter

The previous Tree-sitter-based approach required:
- Custom AST types with optional SourceLocation fields
- Manual location tracking in a HashMap
- Separate parser implementation from txtx-core

The current hcl-edit approach provides:
- Native span support in all AST nodes
- No custom location tracking needed
- Single parser used throughout the codebase

## Why This Approach is Superior

1. **Single Source of Truth** - One parser (hcl-edit) for all parsing needs
2. **Native Support** - Spans are built into the parser, not bolted on
3. **Maintainability** - Less code to maintain (removed ~13,700 lines)
4. **Consistency** - Doctor validates using the same parser that runs the code
5. **Performance** - No additional data structures or lookups needed
6. **Reliability** - hcl-edit is battle-tested and actively maintained

## Lesson Learned
Starting with a well-established parser (hcl-edit) that has native span support proved superior to building a custom Tree-sitter grammar. The unified parser approach ensures consistency between validation and execution while providing precise error locations throughout.