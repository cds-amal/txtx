# Proposal: Adding Source Location to txtx AST

## Current State
The txtx parser uses Tree-sitter for parsing, which provides accurate source location information (line/column). However, when converting Tree-sitter nodes to our AST, this location information is lost, resulting in error messages without line/column information.

## Problem
Doctor command and other tools cannot provide precise error locations, making debugging difficult for users.

## Proposed Solutions

### Option 1: Comprehensive AST Modification (Attempted)
Add location as a first-class property throughout the AST:
- Add `span: Option<Span>` to every AST node
- Wrap all expressions in `Located<Expression>`
- Update all conversion functions to preserve locations

**Pros:**
- Complete location tracking for every node
- Type-safe location access
- Enables precise error reporting

**Cons:**
- Breaking change to entire codebase
- Must update: parser, builder, visitor, transform, renderer
- Affects txtx-core and all downstream consumers
- Significant refactoring effort

### Option 2: Location Sidecar (Recommended)
Keep AST unchanged but maintain a separate location map:

```rust
pub struct ParseResult {
    pub runbook: Runbook,
    pub locations: LocationMap,
}

pub struct LocationMap {
    // Map from node path to source location
    // e.g., "action.send_eth.attributes.from" -> Span
    locations: HashMap<String, Span>,
}
```

**Pros:**
- No breaking changes to existing code
- Can be added incrementally
- Doctor can opt-in to use locations
- Other tools continue working unchanged

**Cons:**
- Location lookup requires path construction
- Not type-safe (string-based paths)
- Possible path mismatches

### Option 3: Minimal AST Enhancement
Only add spans to top-level blocks:

```rust
pub struct ActionBlock {
    pub name: String,
    pub action_type: String,
    pub attributes: HashMap<String, Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,  // Only for the block itself
}
```

**Pros:**
- Minimal changes needed
- Provides block-level error locations
- Non-breaking (Option type)

**Cons:**
- No location for expressions/attributes
- Less precise than full tracking

## Implementation Plan

### Phase 1: Minimal Enhancement (Quick Win)
1. Add optional `span` field to block types
2. Update parser to capture block locations
3. Update doctor to use block spans when available
4. This provides immediate improvement with minimal changes

### Phase 2: Expression Locations (Future)
1. Design location sidecar system
2. Update parser to build location map
3. Enhance doctor with expression-level locations
4. Could be done without breaking changes

## Example Impact

### Current Error:
```
test.tx: error[1]: Reference to undefined action 'foo'
   Make sure the action is defined before using it
```

### With Block Locations (Phase 1):
```
test.tx:35: error[1]: Reference to undefined action 'foo'
   Make sure the action is defined before using it
```

### With Full Locations (Phase 2):
```
test.tx:35:12: error[1]: Reference to undefined action 'foo'
   Make sure the action is defined before using it
   |
35 |   value = action.foo.result
   |           ^^^^^^^^^^
```

## Recommendation

Start with Option 3 (Minimal AST Enhancement) for immediate improvement, then consider Option 2 (Location Sidecar) for comprehensive tracking without breaking changes.

The comprehensive approach (Option 1) should be considered for a future major version where breaking changes are acceptable.