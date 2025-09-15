# QA Infrastructure Improvement Plan

## Executive Summary

This plan outlines a comprehensive overhaul of the txtx testing infrastructure to reduce friction, improve test coverage, and make testing a first-class concern. The implementation will be done in phases, with clear success criteria for each phase.

## Current State Analysis

### Problems
1. **Build Issues**: `cargo test` fails due to supervisor-ui build dependencies
2. **Complex Test Setup**: Existing TestHarness is overly complex for simple scenarios
3. **No Test Patterns**: Lack of established patterns for common test scenarios
4. **Poor Discoverability**: Test utilities exist but are rarely used
5. **Mixed Test Types**: Shell scripts, Rust tests, and manual tests with no clear organization
6. **No Mocking**: Difficult to test blockchain interactions without real networks
7. **Manual Assertions**: No snapshot testing for complex outputs like doctor/LSP responses

### Existing Assets
- `TestHarness` in txtx-test-utils (good for UI flow testing)
- Some integration tests in various crates
- Shell scripts for end-to-end testing
- Test fixtures in various directories

## Implementation Plan

### Phase 1: Foundation (Days 1-3)

**Goal**: Make tests runnable and establish basic patterns

**Tasks**:
1. Fix build configuration to allow `cargo test` without supervisor-ui
2. Create `RunbookBuilder` API for simple test scenarios
3. Add basic assertion macros
4. Document testing patterns

**Deliverables**:
- [ ] `crates/txtx-test-utils/src/builders/runbook_builder.rs`
- [ ] `crates/txtx-test-utils/src/assertions/mod.rs`
- [ ] Working `cargo test-cli` command
- [ ] `TESTING_GUIDE.md` with examples

**Success Criteria**:
- `cargo test --workspace --exclude txtx-supervisor-ui` runs without errors
- At least 5 existing tests converted to use RunbookBuilder
- New developer can write a test following the guide in <10 minutes

### Phase 2: Mock Infrastructure (Days 4-7)

**Goal**: Enable testing of blockchain interactions without external dependencies

**Tasks**:
1. Design `MockBlockchain` trait and base implementation
2. Create mock addons for EVM, Bitcoin, and Solana
3. Implement stateful mock behavior (balances, transactions, etc.)
4. Add mock verification capabilities

**Deliverables**:
- [ ] `crates/txtx-test-utils/src/mocks/blockchain.rs`
- [ ] `crates/txtx-test-utils/src/mocks/evm.rs`
- [ ] `crates/txtx-test-utils/src/mocks/bitcoin.rs`
- [ ] `crates/txtx-test-utils/src/mocks/solana.rs`
- [ ] Integration tests using mocks

**Success Criteria**:
- Can test full contract deployment without network connection
- Can simulate blockchain failures and edge cases
- Mock tests run 100x faster than network tests
- Test coverage increases by 20%

### Phase 3: Snapshot Testing (Days 8-10)

**Goal**: Make complex output testing maintainable

**Tasks**:
1. Integrate `insta` crate for snapshot testing
2. Create snapshot tests for doctor command output
3. Create snapshot tests for LSP responses
4. Add snapshot review workflow

**Deliverables**:
- [ ] Doctor command snapshot tests
- [ ] LSP hover/completion snapshot tests
- [ ] CLI output snapshot tests
- [ ] Snapshot review documentation

**Success Criteria**:
- All doctor validation messages covered by snapshots
- Can update snapshots with single command
- PR workflow includes snapshot diff review
- Reduced test maintenance time by 50%

### Phase 4: Test Organization (Days 11-14)

**Goal**: Establish clear test structure and patterns

**Tasks**:
1. Reorganize tests to be colocated with code
2. Convert shell scripts to Rust tests where appropriate
3. Create test template/snippet library
4. Set up property-based testing for parsers

**Deliverables**:
- [ ] Reorganized test structure
- [ ] Test templates in `.vscode/snippets/`
- [ ] Property-based parser tests
- [ ] Updated CI configuration

**Success Criteria**:
- Clear separation of unit/integration/e2e tests
- Test files next to implementation files
- 90% of shell script tests converted to Rust
- CI runs categorized test suites

### Phase 5: Advanced Features (Days 15+)

**Goal**: Add advanced testing capabilities

**Tasks**:
1. Implement test data generators
2. Add fuzzing infrastructure
3. Create performance benchmarks
4. Add mutation testing

**Deliverables**:
- [ ] Test data generators for common patterns
- [ ] Fuzzing harness for parser/validator
- [ ] Performance regression tests
- [ ] Mutation testing report

**Success Criteria**:
- Can generate random valid runbooks for testing
- Fuzzer runs nightly and finds edge cases
- Performance regressions caught automatically
- Mutation score >80%

## Specific Examples

### Example 1: Simple Validation Test
```rust
#[test]
fn test_undefined_variable_error() {
    let result = RunbookBuilder::new()
        .with_content(r#"
            action "test" "core::print" {
                message = input.undefined_var
            }
        "#)
        .validate();
        
    assert_validation_error!(result, ValidationError::UndefinedVariable {
        name: "undefined_var".to_string(),
        line: 3,
    });
}
```

### Example 2: Mock Blockchain Test
```rust
#[test]
fn test_insufficient_gas() {
    let mock = MockBlockchain::new()
        .with_account("0x123", 1000)  // Low balance
        .with_gas_price(500);          // High gas price
        
    let result = RunbookBuilder::new()
        .from_file("fixtures/deploy_contract.tx")
        .with_mock_blockchain(mock)
        .execute();
        
    assert_execution_error!(result, "Insufficient gas");
}
```

### Example 3: Snapshot Test
```rust
#[test]
fn test_doctor_multi_file_output() {
    let result = DoctorCommand::new()
        .add_file("main.tx", include_str!("fixtures/multi_file/main.tx"))
        .add_file("flows.tx", include_str!("fixtures/multi_file/flows.tx"))
        .run();
        
    insta::assert_snapshot!(result.formatted_output());
}
```

## Metrics for Success

### Quantitative
- **Test Execution Time**: <30 seconds for full suite (currently >5 minutes)
- **Test Coverage**: >80% line coverage (currently ~45%)
- **Test Count**: 500+ tests (currently ~150)
- **Build Success Rate**: 100% on all platforms (currently fails on some)
- **Time to Write Test**: <5 minutes for common scenarios (currently 20-30 minutes)

### Qualitative
- **Developer Feedback**: "Tests are easy to write and understand"
- **Maintenance Burden**: Test updates require <10% of feature development time
- **Confidence Level**: Team confident to refactor with test safety net
- **Documentation Quality**: New developers can write tests without asking questions

## Risk Mitigation

### Risk 1: Breaking Existing Tests
**Mitigation**: Run all changes in parallel with existing tests until stable

### Risk 2: Over-engineering
**Mitigation**: Start simple, iterate based on actual usage

### Risk 3: Adoption Resistance  
**Mitigation**: Convert high-value tests first to demonstrate benefits

### Risk 4: Performance Regression
**Mitigation**: Benchmark test execution time throughout implementation

## Timeline Summary

- **Week 1**: Foundation + Mock Infrastructure (Phases 1-2)
- **Week 2**: Snapshot Testing + Organization (Phases 3-4)
- **Week 3+**: Advanced Features + Iteration (Phase 5)

## Next Steps

1. Review and approve this plan
2. Create tracking issues for each phase
3. Begin Phase 1 implementation
4. Set up weekly progress reviews

---

*This plan is a living document and will be updated as we learn and iterate.*