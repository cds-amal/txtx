# How the Doctor Command Works

The doctor command is a diagnostic tool that analyzes txtx runbook files to find common errors and provide helpful suggestions. Here's how it works:

## 1. Command Entry Point

```rust
// In crates/txtx-cli/src/cli/mod.rs
Command::Doctor(cmd) => {
    run_doctor(cmd.manifest_path, cmd.runbook, cmd.environment).await?;
}
```

The CLI invokes `run_doctor()` with optional manifest path, runbook name, and environment.

## 2. File Discovery

The doctor can work in several modes:

```rust
// In doctor/mod.rs - run_doctor()
if let Some(runbook_name) = runbook_name {
    // First try as direct file path
    if path.exists() && path.extension() == "tx" {
        analyze_runbook_file(&path)?;
    } else {
        // Try to find in manifest
        // Look for runbook in manifest.runbooks
    }
} else {
    // No runbook specified - check all in manifest
}
```

## 3. Parse and Analyze

Once a runbook file is found:

```rust
// In analyze_runbook_with_context()
let content = std::fs::read_to_string(path)?;

// Parse the runbook using hcl-edit parser
match hcl_validator::validate_with_hcl(content, &mut result, file_path) {
    Ok(input_refs) => {
        // If manifest provided, validate inputs
        if let Some(manifest) = manifest {
            validate_inputs_against_manifest(...);
        }
    }
    Err(e) => // Record parse error
}
```

## 4. Visitor Pattern Validation

The core validation uses the visitor pattern to traverse the AST:

```rust
// In parser_validator.rs
pub struct ValidationVisitor<'a> {
    result: &'a mut DoctorResult,
    action_types: HashMap<String, String>,      // action name -> type
    action_specs: HashMap<String, CommandSpec>, // action name -> specification
    addon_specs: HashMap<String, Vec<...>>,     // addon -> available actions
}

impl RunbookVisitor for ValidationVisitor {
    fn visit_runbook(&mut self, runbook: &Runbook) {
        // First pass: collect all action definitions
        for action in &runbook.actions {
            self.visit_action(action);
        }
        
        // Second pass: validate outputs
        for output in &runbook.outputs {
            self.visit_output(output);
        }
    }
    
    fn visit_action(&mut self, action: &ActionBlock) {
        // Record action type (e.g., "send" -> "evm::send_eth")
        self.action_types.insert(action.name, action.action_type);
        
        // Get specification from addon
        let parts = action.action_type.split("::");
        if let Some(addon_actions) = self.addon_specs.get(addon_name) {
            // Find and store the action specification
        }
    }
    
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Reference(parts) => {
                // Check references like action.send.from
                self.validate_action_reference(parts);
            }
            // Recursively check other expressions
        }
    }
}
```

## 5. Validation Rules

The doctor checks for several types of errors:

### a) Undefined Action References
```rust
// If output references action.nonexistent.result
if !self.action_types.contains_key(action_name) {
    result.errors.push(DoctorError {
        message: "Reference to undefined action 'nonexistent'",
        context: "Make sure the action is defined before using it"
    });
}
```

### b) Invalid Field Access
```rust
// If output references action.send.from but send_eth only outputs tx_hash
let available_outputs = spec.outputs.iter().map(|o| &o.name);
if !available_outputs.contains(requested_field) {
    result.errors.push(DoctorError {
        message: "Field 'from' does not exist on action 'send' (evm::send_eth). 
                  The send_eth action only outputs: tx_hash",
        documentation_link: "https://docs.txtx.sh/addons/evm/actions#send-eth"
    });
}
```

### c) Special Cases (e.g., send_eth)
```rust
// Common mistake: trying to access input fields as outputs
if action_type.contains("send_eth") && 
   (field == "from" || field == "to" || field == "value") {
    // Add specific error with helpful context
    result.suggestions.push(DoctorSuggestion {
        message: "To access transaction details, you would need to use 
                  a different action that queries transaction data."
    });
}
```

## 6. Addon Specifications

The doctor loads specifications from all available addons:

```rust
fn get_addon_specifications() -> HashMap<String, Vec<(String, CommandSpecification)>> {
    let mut specs = HashMap::new();
    
    for addon in get_available_addons() {
        let provider = addon.get_provider();
        let actions = provider.get_actions_definitions();
        
        // Convert PreCommandSpec to CommandSpec
        for action in actions {
            // Extract inputs, outputs, examples, etc.
        }
        
        specs.insert(addon.get_namespace(), actions);
    }
    
    specs
}
```

## 7. Error Reporting

Results are collected in a structured format:

```rust
pub struct DoctorResult {
    pub errors: Vec<DoctorError>,
    pub warnings: Vec<DoctorWarning>,
    pub suggestions: Vec<DoctorSuggestion>,
}

pub struct DoctorError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,      // Future: line numbers
    pub column: Option<usize>,     // Future: column numbers
    pub context: Option<String>,   // Additional helpful context
    pub documentation_link: Option<String>,
}
```

## 8. Display Results

Finally, results are displayed with color coding:

```rust
fn display_results(result: &DoctorResult) {
    if result.errors.is_empty() {
        println!("{} No issues found!", Blue.paint("✓"));
        return;
    }
    
    println!("{}", Red.bold().paint("Found X issue(s):"));
    
    for error in &result.errors {
        println!("{}", Red.paint(&error.message));
        if let Some(context) = &error.context {
            println!("   {}", context);
        }
        if let Some(link) = &error.documentation_link {
            println!("   Documentation: {}", link);
        }
    }
}
```

## Example Flow

For a runbook with `action.send.from`:

1. **Parse** → Creates AST with action "send" and output referencing "action.send.from"
2. **Visit Actions** → Records "send" → "evm::send_eth" with spec showing only "tx_hash" output
3. **Visit Output** → Finds reference to "action.send.from"
4. **Validate** → Checks if "from" exists in send_eth outputs (it doesn't)
5. **Error** → Adds error with message, context, and documentation link
6. **Display** → Shows colored error message with helpful suggestions

## Architecture Benefits

This design makes it easy to:
- Add new validation rules by extending the visitor pattern
- Support new addons automatically (they provide their own specifications)
- Maintain clean separation between parsing, validation, and display
- Test validation logic independently from file I/O and CLI concerns

## Testing Strategy

The doctor command is tested at multiple levels:

1. **Unit Tests** (`doctor/mod.rs`) - Test validation logic with fixtures and synthetic examples
2. **Integration Tests** (`tests/doctor_tests.rs`) - Test full CLI execution with real binary
3. **Fixture-based Tests** - Leverage existing `doctor_demo` fixtures for real-world scenarios

The visitor pattern makes it particularly easy to test validation logic in isolation by creating AST structures programmatically and running the visitor on them.