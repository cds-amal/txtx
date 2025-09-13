//! AST types for txtx runbook language

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Source location tracking with full span information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub start_line: usize,    // 0-based line number
    pub start_column: usize,  // 0-based column number  
    pub end_line: usize,      // 0-based end line
    pub end_column: usize,    // 0-based end column
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Runbook {
    pub addons: Vec<AddonBlock>,
    pub signers: Vec<SignerBlock>,
    pub actions: Vec<ActionBlock>,
    pub outputs: Vec<OutputBlock>,
    pub variables: Vec<VariableDeclaration>,
    pub flows: Vec<FlowBlock>,
    pub modules: Vec<ModuleBlock>,
    pub runbook_blocks: Vec<RunbookBlock>,
}

impl Runbook {
    pub fn new() -> Self {
        Self {
            addons: Vec::new(),
            signers: Vec::new(),
            actions: Vec::new(),
            outputs: Vec::new(),
            variables: Vec::new(),
            flows: Vec::new(),
            modules: Vec::new(),
            runbook_blocks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddonBlock {
    pub network: String,
    pub attributes: HashMap<String, Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignerBlock {
    pub name: String,
    pub signer_type: String,
    pub attributes: HashMap<String, Expression>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionBlock {
    pub name: String,
    pub action_type: String,
    pub attributes: HashMap<String, Expression>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputBlock {
    pub name: String,
    pub attributes: HashMap<String, Expression>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub name: String,
    pub attributes: HashMap<String, Expression>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowBlock {
    pub name: String,
    pub attributes: HashMap<String, Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleBlock {
    pub name: String,
    pub attributes: HashMap<String, Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunbookBlock {
    pub name: String,
    pub attributes: HashMap<String, Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Located<T> {
    pub value: T,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Expression {
    String(String),
    Number(f64),
    Bool(bool),
    Reference(Vec<String>),
    Array(Vec<Expression>),
    Object(HashMap<String, Expression>),
    FunctionCall { name: String, args: Vec<Expression> },
}

// Convenience methods
impl Expression {
    pub fn string(s: impl Into<String>) -> Self {
        Expression::String(s.into())
    }

    pub fn number(n: impl Into<f64>) -> Self {
        Expression::Number(n.into())
    }

    pub fn reference(parts: Vec<&str>) -> Self {
        Expression::Reference(parts.iter().map(|s| s.to_string()).collect())
    }

    pub fn input_ref(name: &str) -> Self {
        Expression::reference(vec!["input", name])
    }

    pub fn action_ref(action: &str, field: &str) -> Self {
        Expression::reference(vec!["action", action, field])
    }

    pub fn signer_ref(name: &str) -> Self {
        Expression::reference(vec!["signer", name])
    }

    pub fn signer_field(name: &str, field: &str) -> Self {
        Expression::reference(vec!["signer", name, field])
    }

    pub fn function_call(name: impl Into<String>, args: Vec<Expression>) -> Self {
        Expression::FunctionCall { name: name.into(), args }
    }

    pub fn bool(b: bool) -> Self {
        Expression::Bool(b)
    }

    pub fn object(entries: Vec<(&str, Expression)>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in entries {
            map.insert(key.to_string(), value);
        }
        Expression::Object(map)
    }
}
