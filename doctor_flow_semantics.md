# txtx Flow Execution Semantics for Doctor Validation

## Key Concept: Sequential Multi-Execution
When a runbook contains flow blocks, the **entire runbook executes once for each flow**, in definition order.

## Example Execution

Given this runbook:
```hcl
flow "mainnet" {
    chain_id = 1
    gas = 30
}

flow "testnet" {
    chain_id = 5
    gas = 1
}

action "deploy" "evm::deploy" {
    chain_id = flow.chain_id
    gas_price = flow.gas
}

output "result" {
    value = action.deploy.address
}
```

### Execution Sequence:
1. **First Execution (mainnet flow)**:
   - `flow.chain_id` = 1
   - `flow.gas` = 30
   - Deploy action runs with chain_id=1, gas_price=30
   - Output captures mainnet deployment address

2. **Second Execution (testnet flow)**:
   - `flow.chain_id` = 5
   - `flow.gas` = 1
   - Deploy action runs with chain_id=5, gas_price=1
   - Output captures testnet deployment address

## Validation Rules for Doctor Command

### 1. Flow Attribute Consistency ✅
- **MUST** exist in all flows if referenced
- Missing attribute in any flow = **ERROR** (not warning)
- Rationale: Will cause runtime failure when that flow executes

### 2. Variable Scoping ✅
- `flow.*` references are valid **everywhere** in the runbook
- `input.*` references are global (CLI inputs)
- `variable.*` must be defined before use
- `action.*` must be defined before use
- `signer.*` must be defined before use

### 3. Invalid References ✅
- Cannot reference specific flow by name (e.g., `flow.mainnet.*`)
- Cannot reference undefined flow attributes
- Cannot use forward references (using before definition)

## Static Analysis Implementation

The doctor command validates:

1. **Parse-time checks**:
   - Syntax errors
   - Block structure
   - Expression validity

2. **Semantic checks**:
   - All flow attributes referenced exist in ALL flows
   - All variable/action/signer references are defined
   - No circular dependencies

3. **NOT checked** (left for runtime):
   - Actual values of flow attributes
   - Network connectivity
   - Transaction success/failure

## Common Issues Detected

### Issue 1: Inconsistent Flow Attributes
```hcl
flow "mainnet" {
    chain_id = 1
    gas_price = 30
}

flow "testnet" {
    chain_id = 5
    # Missing gas_price!
}

action "deploy" "evm::deploy" {
    gas = flow.gas_price  # ERROR: gas_price missing in testnet
}
```

### Issue 2: Undefined References
```hcl
action "transfer" "evm::send_eth" {
    signer = signer.wallet  # ERROR: signer.wallet not defined
}
```

### Issue 3: Invalid Action Outputs
```hcl
action "send" "evm::send_eth" {
    # ...
}

output "sender" {
    value = action.send.from  # ERROR: send_eth doesn't output 'from'
}
```

## Implementation Status

✅ **Completed**:
- Flow block parsing in tree-sitter grammar
- AST representation of flows
- Visitor pattern for flow validation
- Attribute consistency checking across flows
- Reference validation for all construct types

⏳ **Future Enhancements**:
- Line/column information for errors
- Quickfix suggestions
- Auto-correction capabilities