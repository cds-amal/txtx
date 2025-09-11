# Doctor Command Flow Validation Enhancement Plan

## Current State
The doctor command uses the tree-sitter parser for static analysis but needs enhancements to properly validate variable references in the context of flows.

## Variable Scoping Rules in txtx

### 1. Global Scope Variables
- `input.*` - Command-line inputs available everywhere
- `variable.*` - Variables defined at the root level
- `module.*` - Module-level metadata

### 2. Flow-Scoped Variables
When flows are defined:
- ALL flows are executed sequentially (first to last)
- Each flow creates a complete execution context
- Flow attributes become available as `flow.<attribute>` during that flow's execution
- The entire runbook runs once per flow with that flow's attributes
- Example: With 3 flows, the runbook executes 3 times with different `flow.*` values each time

### 3. Construct References
- `action.<name>.*` - Action outputs (must be defined before use)
- `signer.<name>.*` - Signer references
- `output.<name>.*` - Output values (for cross-output references)

## Required Enhancements

### 1. Flow-Aware Variable Resolution

```rust
// Enhanced ValidationVisitor
pub struct ValidationVisitor<'a> {
    // ... existing fields ...
    
    /// Track which flow context we're in (if any)
    current_flow: Option<String>,
    
    /// Map of flow names to their defined inputs
    flow_inputs: HashMap<String, Vec<String>>,
    
    /// Track all variable definitions
    defined_variables: HashMap<String, String>, // name -> type/value
    
    /// Track construct definition order for dependency validation
    definition_order: Vec<(String, String)>, // (type, name)
}
```

### 2. Enhanced Reference Validation

```rust
impl<'a> ValidationVisitor<'a> {
    fn validate_reference(&mut self, parts: &[String]) {
        match parts.get(0).map(|s| s.as_str()) {
            Some("input") => {
                // Validate CLI input reference
                self.validate_input_reference(parts);
            }
            Some("variable") => {
                // Check if variable is defined
                self.validate_variable_reference(parts);
            }
            Some("flow") => {
                // Validate flow input reference
                self.validate_flow_reference(parts);
            }
            Some("action") => {
                // Existing action validation
                self.validate_action_reference(parts);
            }
            Some("signer") => {
                // Check if signer is defined
                self.validate_signer_reference(parts);
            }
            Some("output") => {
                // Check for circular dependencies
                self.validate_output_reference(parts);
            }
            Some("module") => {
                // Validate module metadata reference
                self.validate_module_reference(parts);
            }
            _ => {
                // Unknown reference type
                self.report_unknown_reference(parts);
            }
        }
    }
    
    fn validate_flow_reference(&mut self, parts: &[String]) {
        if self.current_flow.is_none() {
            self.result.errors.push(DoctorError {
                message: "Flow references can only be used inside a flow block".to_string(),
                // ...
            });
            return;
        }
        
        if parts.len() < 2 {
            return;
        }
        
        let input_name = &parts[1];
        if let Some(flow_name) = &self.current_flow {
            if let Some(inputs) = self.flow_inputs.get(flow_name) {
                if !inputs.contains(input_name) {
                    self.result.errors.push(DoctorError {
                        message: format!(
                            "Undefined flow input '{}' in flow '{}'",
                            input_name, flow_name
                        ),
                        // ...
                    });
                }
            }
        }
    }
}
```

### 3. Flow Context Tracking

```rust
impl<'a> RunbookVisitor for ValidationVisitor<'a> {
    fn visit_flow(&mut self, flow: &FlowBlock) {
        // Set current flow context
        let previous_flow = self.current_flow.clone();
        self.current_flow = Some(flow.name.clone());
        
        // Collect flow inputs
        let mut flow_inputs = Vec::new();
        for (key, _value) in &flow.attributes {
            if key != "description" {
                flow_inputs.push(key.clone());
            }
        }
        self.flow_inputs.insert(flow.name.clone(), flow_inputs);
        
        // Visit flow attributes
        self.visit_attributes(&flow.attributes);
        
        // Restore previous context
        self.current_flow = previous_flow;
    }
}
```

### 4. Dependency Order Validation

```rust
impl<'a> ValidationVisitor<'a> {
    fn check_definition_order(&self, ref_type: &str, ref_name: &str) -> bool {
        // Check if the referenced construct is defined before current position
        let current_pos = self.definition_order.len();
        
        for (i, (def_type, def_name)) in self.definition_order.iter().enumerate() {
            if def_type == ref_type && def_name == ref_name {
                return i < current_pos;
            }
        }
        
        false
    }
}
```

## Implementation Steps

### Phase 1: Basic Flow Support
1. ✅ Parse flow blocks (DONE)
2. ✅ Add FlowBlock to AST (DONE)
3. ✅ Visit flow blocks in visitor (DONE)
4. ⏳ Track flow context during traversal
5. ⏳ Validate flow input references

### Phase 2: Complete Variable Scoping
1. ⏳ Track all variable definitions
2. ⏳ Validate variable references
3. ⏳ Check for undefined variables
4. ⏳ Detect circular dependencies

### Phase 3: Enhanced Error Reporting
1. ⏳ Add line/column information from source
2. ⏳ Provide fix suggestions
3. ⏳ Generate quickfix format for editors

## Testing Strategy

### Test Cases Needed

1. **Flow with undefined input**
```hcl
flow "test" {
    chain_id = 1
}
action "deploy" "evm::deploy" {
    value = flow.undefined_input  // ERROR: undefined flow input
}
```

2. **Reference outside flow context**
```hcl
variable "test" {
    value = flow.chain_id  // ERROR: flow references only valid inside flow
}
```

3. **Cross-flow references**
```hcl
flow "mainnet" {
    chain_id = 1
}
flow "testnet" {
    chain_id = flow.chain_id  // ERROR: cannot reference other flow's inputs
}
```

4. **Proper flow usage**
```hcl
flow "mainnet" {
    chain_id = 1
    rpc_url = input.rpc_url  // OK: input reference
}

action "deploy" "evm::deploy" {
    chain_id = flow.chain_id  // OK: flow input in action
}
```

## Benefits

1. **Catch errors at parse time** - No need to run the runbook to find undefined variables
2. **Better error messages** - Precise location and context for each error
3. **IDE integration** - Quickfix format enables automatic fixes in editors
4. **Flow-aware validation** - Properly handles the scoping rules of flows

## Next Steps

1. Implement `ValidationVisitor` enhancements in `parser_validator.rs`
2. Add flow context tracking to the visitor pattern
3. Create comprehensive test suite for flow validation
4. Update doctor command output formats to include flow-specific errors
5. Document the validation rules for users