# Architectural Fork: Validation Architecture

## Context

We've identified a circular dependency between `txtx-cli` and `txtx-test-utils` when trying to use the doctor validation in tests. The RunbookBuilder in test-utils needs access to the RunbookAnalyzer from txtx-cli, but txtx-cli depends on test-utils for testing.

## The Fork

We're exploring two architectural approaches to resolve this:

### Branch: `feat/qa-unified` - Unified Processor Architecture

**Core Insight**: Validation is just another form of "processing" a runbook, similar to execution, planning, or transformation.

**Approach**:
- Define a generic `RunbookProcessor` trait in `txtx-core`
- Implement validators, executors, and doctor as processors
- Enable composition and pipeline processing
- All processors follow the same pattern

**Key Changes**:
```rust
pub trait RunbookProcessor {
    type Output;
    type Error;
    type Context;
    
    fn process(&self, runbook: &ParsedRunbook, context: Self::Context) 
        -> Result<Self::Output, Self::Error>;
}
```

**Benefits**:
- Conceptual unity: everything is a processor
- Composability: chain processors together
- Extensibility: easy to add new processing types
- Clean testing: mock any processor

### Branch: `feat/qa-crate` - Separate Doctor Crate

**Approach**:
- Extract doctor validation into `txtx-doctor-core` crate
- Both `txtx-cli` and `txtx-test-utils` depend on this new crate
- Clear separation of validation logic

**Key Structure**:
```
txtx-doctor-core/
├── src/
│   ├── analyzer.rs      # RunbookAnalyzer
│   ├── rules/           # Validation rules
│   └── diagnostics.rs   # Result types
```

**Benefits**:
- Simple and direct solution
- Clear ownership and boundaries
- Independent versioning
- Focused testing

## Evaluation Criteria

We'll build both approaches and compare:

1. **Implementation Complexity**
   - How much refactoring is required?
   - How many files need to change?

2. **Developer Experience**
   - How easy is it to add new validation rules?
   - How clear is the API?

3. **Testing**
   - How easy is it to test validators?
   - Can we mock effectively?

4. **Performance**
   - Any runtime overhead?
   - Build time impact?

5. **Extensibility**
   - How easy to add new types of processing?
   - How well does it handle future needs?

6. **Maintenance**
   - Long-term maintenance burden
   - Clarity for new contributors

## Decision Timeline

1. Create both branches from current `feat/qa`
2. Implement minimal working version of each approach
3. Port existing doctor tests to use new architecture
4. Compare results and choose direction
5. Complete full implementation of chosen approach

## Success Metrics

- All doctor tests can use full validation
- No circular dependencies
- Clean, understandable architecture
- Minimal disruption to existing code
- Clear path for future enhancements