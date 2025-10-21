# Branch `feat/addon-kit-factor`: Proposed Improvements

## Purpose

This document catalogs refactoring patterns explored in the `feat/addon-kit-factor` branch.
It's intended as a discussion guide for maintainers to review each change category and decide
which improvements are worth integrating into the main codebase.

Presenting these changes beforehand allows for incremental, consensus-driven adoption rather
than a large surprise PR.

---

## 1. DRY Violation in Diagnostic Constructors

**Commit:** `2d64c57af:` Eliminate DRY violation in Diagnostic constructors

### Before (51 lines of duplicated code)
```rust
pub fn error_from_string(message: String) -> Diagnostic {
    Diagnostic {
        level: DiagnosticLevel::Error,
        message,
        code: None,
        span: None,
        span_range: None,
        location: None,
        file: None,
        line: None,
        column: None,
        context: None,
        related_locations: Vec::new(),
        documentation: None,
        suggestion: None,
        example: None,
        parent_diagnostic: None,
    }
}

pub fn warning_from_string(message: String) -> Diagnostic {
    Diagnostic {
        level: DiagnosticLevel::Warning,
        message,
        // ... same 14 fields repeated ...
    }
}

pub fn note_from_string(message: String) -> Diagnostic {
    Diagnostic {
        level: DiagnosticLevel::Note,
        message,
        // ... same 14 fields repeated again ...
    }
}
```

### After (23 lines, centralized logic)
```rust
impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            span: None,
            span_range: None,
            location: None,
            message: String::new(),
            level: DiagnosticLevel::Error,
            documentation: None,
            example: None,
            parent_diagnostic: None,
        }
    }
}

impl Diagnostic {
    pub fn with_level(level: DiagnosticLevel, message: String) -> Self {
        Self { message, level, ..Default::default() }
    }

    pub fn error_from_string(message: String) -> Diagnostic {
        Self::with_level(DiagnosticLevel::Error, message)
    }

    pub fn warning_from_string(message: String) -> Diagnostic {
        Self::with_level(DiagnosticLevel::Warning, message)
    }

    pub fn note_from_string(message: String) -> Diagnostic {
        Self::with_level(DiagnosticLevel::Note, message)
    }
}
```

**Impact:** -51 lines, +23 lines. Single point of maintenance for default field initialization.

---

## 2. Type-Safe Constant Enums (Eliminating String Constants)

**Commit:** `f921c55a2:` Replace string constants with type-safe enums using Strum derives

### Before (scattered string constants)
```rust
// Signers
pub const SIGNED_MESSAGE_BYTES: &str = "signed_message_bytes";
pub const SIGNED_TRANSACTION_BYTES: &str = "signed_transaction_bytes";
pub const TX_HASH: &str = "tx_hash";
pub const SIGNATURE_APPROVED: &str = "signature_approved";
pub const SIGNATURE_SKIPPABLE: &str = "signature_skippable";

pub const ACTION_ITEM_CHECK_ADDRESS: &str = "check_address";
pub const CHECKED_ADDRESS: &str = "checked_address";
pub const ACTION_ITEM_CHECK_BALANCE: &str = "check_balance";
// ... more scattered constants ...
```

### After (organized, type-safe enums)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum SignerKey {
    SignedMessageBytes,
    SignedTransactionBytes,
    TxHash,
    SignatureApproved,
    SignatureSkippable,
    ProvidePublicKeyActionResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum ActionItemKey {
    CheckAddress,
    CheckedAddress,
    CheckBalance,
    IsBalanceChecked,
    BeginFlow,
    ReExecuteCommand,
}
```

**Benefits:**
- Compile-time validation (typos caught at compile time)
- Organized by domain (SignerKey, ActionItemKey, DocumentationKey, etc.)
- IDE autocomplete and refactoring support
- `Display`, `FromStr`, `AsRef<str>` derived automatically

---

## 3. ValueStore API Ergonomics

**Commit:** `0dcd1c56f:` Improve ValueStore API ergonomics with `impl AsRef<str>` and `impl ToString`

### Before (verbose `.as_ref()` everywhere)
```rust
let description = values.get_string(DocumentationKey::Description.as_ref())
    .map(|d| d.to_string());

if let Some(_) = signer_state.get_scoped_value(
    &construct_did_str,
    SignerKey::SignatureApproved.as_ref()
) { ... }

result.outputs.insert(
    SignerKey::TxHash.as_ref().to_string(),
    EvmValue::tx_hash(tx_hash.to_vec())
);
```

### After (clean enum usage)
```rust
let description = values.get_string(DocumentationKey::Description)
    .map(|d| d.to_string());

if let Some(_) = signer_state.get_scoped_value(
    &construct_did_str,
    SignerKey::SignatureApproved
) { ... }

result.outputs.insert(
    SignerKey::TxHash.to_string(),
    EvmValue::tx_hash(tx_hash.to_vec())
);
```

**Impact:** Removed hundreds of `.as_ref()` calls across 30 files while maintaining type safety.

---

## 4. Structured Function Errors

**Commit:** `9c31373e4:` Add Strum traits to Type enum and structured FunctionError

### Before (ad-hoc error string formatting)
```rust
return Err(diagnosed_error!(
    "function '{}::{}' missing required argument #{} ({})",
    namespace, function_name, position, arg_name
));
```

### After (structured error types with Display)
```rust
#[derive(Debug, Clone)]
pub enum FunctionError {
    MissingArgument {
        namespace: String,
        function: String,
        position: usize,
        name: String,
    },
    TypeMismatch {
        namespace: String,
        function: String,
        position: usize,
        name: String,
        expected: Vec<Type>,
        found: Type,
    },
    ExecutionError {
        namespace: String,
        function: String,
        message: String,
    },
}

// Zero-allocation borrowing variant for hot paths
pub enum FunctionErrorRef<'a> { ... }

impl From<FunctionErrorRef<'a>> for Diagnostic { ... }
```

**Benefits:**
- Consistent error formatting across the codebase
- Structured data for programmatic error handling
- `FunctionErrorRef` avoids allocations in performance-critical code

---

## 5. Type Compatibility Logic Extraction

**Commit:** `e122d3ac9:` Extract type compatibility logic into dedicated module

### Before (inline complex matching in arg_checker)
```rust
for typing in input.typing.iter() {
    let arg_type = arg.get_type();
    // special case if both are addons
    if let Type::Addon(_) = arg_type {
        if let Type::Addon(_) = typing {
            has_type_match = true;
            break;
        }
    }
    // special case for empty arrays
    if let Type::Array(_) = arg_type {
        if arg.expect_array().len() == 0 {
            has_type_match = true;
            break;
        }
    }
    // we don't have an "any" type, so if the array is of type null...
    if let Type::Array(inner) = typing {
        if let Type::Null(_) = **inner {
            has_type_match = true;
            break;
        }
    }
    if arg_type.eq(typing) {
        has_type_match = true;
        break;
    }
}
```

### After (dedicated TypeChecker with unit tests)
```rust
pub struct TypeChecker;

impl TypeChecker {
    pub fn matches(value: &Value, expected_type: &Type) -> bool {
        match (value.get_type(), expected_type) {
            (Type::Addon(_), Type::Addon(_)) => true,
            (Type::Array(_), _) if value.expect_array().is_empty() => true,
            (_, Type::Array(inner)) if matches!(**inner, Type::Null(_)) => true,
            (actual_type, expected) => actual_type.eq(expected),
        }
    }
}

// Usage in arg_checker:
let type_matches = input.typing.iter().any(|typing| {
    TypeChecker::matches(arg, typing)
});
```

**Benefits:**
- Logic centralized and testable
- Pattern matching is clearer and more idiomatic
- Unit tests ensure correctness

---

## 6. Idiomatic Function API Signatures

**Commit:** `f0604ad09:` Apply idiomatic Rust patterns to function APIs

### Before
```rust
type FunctionRunner = fn(
    &FunctionSpecification,
    &AuthorizationContext,
    &Vec<Value>  // Takes ownership reference to Vec
) -> Result<Value, Diagnostic>;

fn run(
    _fn_spec: &FunctionSpecification,
    _auth_ctx: &AuthorizationContext,
    _args: &Vec<Value>,  // Should be slice
) -> Result<Value, Diagnostic>;
```

### After
```rust
type FunctionRunner = fn(
    &FunctionSpecification,
    &AuthorizationContext,
    &[Value]  // Idiomatic slice reference
) -> Result<Value, Diagnostic>;

fn run(
    _fn_spec: &FunctionSpecification,
    _auth_ctx: &AuthorizationContext,
    _args: &[Value],  // Accepts any contiguous sequence
) -> Result<Value, Diagnostic>;
```

**Benefits:** `&[T]` is more flexible (accepts arrays, vectors, slices) and follows Rust API guidelines.

---

## 7. Namespace Type Safety

**Commit:** `4fa1f05c0:` Replace string namespaces with Namespace enum

### Before
```rust
fn get_namespace() -> &'static str {
    "std"
}

fn register_addon(namespace: &str) { ... }
```

### After
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Namespace {
    WellKnown(WellKnownNamespace),
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Display, EnumString)]
pub enum WellKnownNamespace {
    Std,
}

impl Namespace {
    pub fn std() -> Self { Namespace::WellKnown(WellKnownNamespace::Std) }
    pub fn custom(name: impl Into<String>) -> Self { Namespace::Custom(name.into()) }
}
```

**Benefits:**
- Well-known namespaces are validated at compile time
- Custom namespaces remain flexible
- Type-safe comparisons and serialization

---

## Metrics Summary

| Category | Before | After | Potential Benefit |
|----------|--------|-------|-------------------|
| Diagnostic constructors | 51 lines | 23 lines | -55% code, single maintenance point |
| String constants | Scattered | Organized enums | Compile-time typo detection |
| `.as_ref()` calls | Hundreds | Zero (for ValueStore) | Cleaner call sites |
| Type checking logic | Inline, untested | Extracted, tested | Testable, maintainable |
| Function signatures | `&Vec<T>` | `&[T]` | Idiomatic Rust |
| Namespace handling | Strings | Enum + Custom | Type-safe at boundaries |

**Branch scope:** 81 files, +1,346 / -764 lines

---

## For Maintainers

Each section above represents an independent improvement that can be adopted or rejected on its own merits. Consider:

1. **Which changes align with the project's direction?**
2. **Are there any patterns here that conflict with existing conventions?**

Feedback welcome; this document exists to start that conversation.
