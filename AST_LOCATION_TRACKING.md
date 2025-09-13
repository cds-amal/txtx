# AST Source Location Tracking Implementation

## The Solution: Location HashMap in Doctor

We implemented source location tracking using a **HashMap scoped to the doctor command**, avoiding any changes to the parser or core AST handling.

### Implementation Details

#### 1. Added HashMap to ValidationVisitor
```rust
pub struct ValidationVisitor<'a> {
    // ... existing fields ...
    
    /// Map of construct names to their source locations
    source_locations: HashMap<String, (usize, usize)>,
}
```

#### 2. Populate During Collection Phase
When the visitor collects definitions (first pass), it also stores their locations if available:

```rust
// During collection of signers
for signer in &runbook.signers {
    self.defined_signers.insert(signer.name.clone(), signer.signer_type.clone());
    
    if let Some(loc) = &signer.source_location {
        self.source_locations.insert(
            format!("signer.{}", signer.name),
            (loc.line + 1, loc.column + 1), // Convert to 1-based
        );
    }
}
```

#### 3. Use During Validation Phase
When creating error messages, look up the location from the HashMap:

```rust
// When reporting an error about undefined signer
let (line, column) = self.source_locations
    .get(&format!("{}.{}", self.current_block_type, self.current_block_name))
    .copied()
    .unwrap_or((0, 0));

self.result.errors.push(DoctorError {
    message: format!("Reference to undefined action '{}'", action_name),
    file: self.file_path.clone(),
    line: if line > 0 { Some(line) } else { None },
    column: if column > 0 { Some(column) } else { None },
    // ...
});
```

## Key Design Benefits

### 1. **No Parser Changes**
The parser (`txtx-parser`) remains completely untouched. It continues to work exactly as before.

### 2. **Scoped to Doctor**
The location tracking is entirely contained within the doctor command's ValidationVisitor. No other parts of the codebase are affected.

### 3. **Uses Existing AST Fields**
The AST already had optional `source_location` fields on some nodes. We simply:
- Check if they're populated
- Store them in our local HashMap if present
- Use them for error reporting

### 4. **Graceful Degradation**
If source_location is not available (None), we simply don't add it to the HashMap, and errors are reported without line/column info.

### 5. **No Breaking Changes**
Since we're only reading optional fields that already existed, there are zero breaking changes to any existing code.

## Result
Doctor now provides precise error locations when available:
```
test_doctor_two_pass.tx:37:2: error[1]: Reference to undefined action 'undefined_action'
   Make sure the action is defined before using it in outputs
```

## Architecture Diagram
```
┌──────────────────────────────────────┐
│           txtx-parser                 │
│  (Unchanged - may or may not populate │
│   source_location fields in AST)      │
└────────────────┬─────────────────────┘
                 │ AST with optional
                 │ source_location
                 ▼
┌──────────────────────────────────────┐
│      Doctor ValidationVisitor         │
│                                      │
│  ┌─────────────────────────────┐    │
│  │ source_locations: HashMap    │    │
│  │ -------------------------   │    │
│  │ "action.send" -> (10, 5)    │    │
│  │ "signer.alice" -> (5, 2)    │    │
│  │ "output.result" -> (20, 3)  │    │
│  └─────────────────────────────┘    │
│                                      │
│  Uses HashMap for error reporting   │
└──────────────────────────────────────┘
```

## Why This Approach is Superior

1. **Minimal Impact** - Changes are isolated to doctor command only
2. **Future Flexibility** - Parser can be enhanced to populate source_location without breaking doctor
3. **Clean Separation** - Doctor's validation logic is independent of how locations are provided
4. **Easy to Extend** - Can add more location tracking without touching parser
5. **No Performance Impact** - HashMap is small and only built when doctor runs

## Lesson Learned
The best solution often involves working with what's already there and keeping changes scoped to where they're needed. By using a local HashMap in the doctor command, we achieved precise error reporting without any parser modifications or breaking changes.