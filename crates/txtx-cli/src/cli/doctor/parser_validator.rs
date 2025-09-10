//! Visitor-pattern based validation for the doctor command
//! 
//! This module uses the txtx-parser's visitor pattern to perform comprehensive
//! AST-based validation of runbook files.

use std::collections::HashMap;

use txtx_parser::{RunbookVisitor, Expression, ActionBlock, OutputBlock, Runbook};
use txtx_core::kit::types::commands::CommandSpecification;

use super::{DoctorResult, DoctorError, DoctorSuggestion, get_addon_specifications, get_action_doc_link};

/// Represents an input reference with its location in the source file
#[derive(Debug, Clone)]
pub struct LocatedInputRef {
    pub name: String,
    pub line: usize,
    pub column: usize,
}

/// A visitor that validates runbooks by traversing the AST
pub struct ValidationVisitor<'a> {
    /// Results collector
    result: &'a mut DoctorResult,
    /// Path to the current file being validated
    file_path: String,
    /// Map of action names to their types (built during traversal)
    action_types: HashMap<String, String>,
    /// Map of action names to their specifications
    action_specs: HashMap<String, CommandSpecification>,
    /// Addon specifications
    addon_specs: HashMap<String, Vec<(String, CommandSpecification)>>,
    /// Collected input references with their locations
    pub input_refs: Vec<LocatedInputRef>,
    /// Current context for tracking approximate location
    current_block_type: String,
    current_block_name: String,
}

impl<'a> ValidationVisitor<'a> {
    pub fn new(result: &'a mut DoctorResult, file_path: &str) -> Self {
        Self {
            result,
            file_path: file_path.to_string(),
            action_types: HashMap::new(),
            action_specs: HashMap::new(),
            addon_specs: get_addon_specifications(),
            input_refs: Vec::new(),
            current_block_type: String::new(),
            current_block_name: String::new(),
        }
    }

    /// Validate an action output reference
    fn validate_action_reference(&mut self, parts: &[String]) {
        if parts.len() >= 3 && parts[0] == "action" {
            let action_name = &parts[1];
            let field_path = &parts[2..];

            // Check if action exists
            if let Some(action_type) = self.action_types.get(action_name) {
                if let Some(spec) = self.action_specs.get(action_name) {
                    if !field_path.is_empty() {
                        let requested_field = &field_path[0];

                        // Check if field exists in outputs
                        let output_names: Vec<String> = spec.outputs.iter()
                            .map(|o| o.name.clone())
                            .collect();

                        if !output_names.contains(requested_field) {
                            // Special handling for common mistakes
                            if action_type.contains("send_eth") && (requested_field == "from" || requested_field == "to") {
                                self.result.errors.push(DoctorError {
                                    message: format!(
                                        "Field '{}' does not exist on action '{}' ({}). The send_eth action only outputs: {}",
                                        requested_field, action_name, action_type, output_names.join(", ")
                                    ),
                                    file: self.file_path.clone(),
                                    line: None,
                                    column: None,
                                    context: Some(format!(
                                        "The 'from' and 'to' fields are inputs to send_eth, not outputs. Transaction details like sender/recipient addresses are not returned by this action."
                                    )),
                                    documentation_link: Some("https://docs.txtx.sh/addons/evm/actions#send-eth".to_string()),
                                });

                                self.result.suggestions.push(DoctorSuggestion {
                                    message: "To access transaction details, you would need to use a different action that queries transaction data.".to_string(),
                                    example: None,
                                });
                            } else {
                                self.result.errors.push(DoctorError {
                                    message: format!(
                                        "Field '{}' does not exist on action '{}' ({}). Available outputs: {}",
                                        requested_field, action_name, action_type, output_names.join(", ")
                                    ),
                                    file: self.file_path.clone(),
                                    line: None,
                                    column: None,
                                    context: None,
                                    documentation_link: Some(get_action_doc_link(action_type)),
                                });
                            }
                        }

                        // Check for nested access on non-object fields
                        if field_path.len() > 1 && output_names.contains(requested_field) {
                            self.result.errors.push(DoctorError {
                                message: format!(
                                    "Cannot access property '{}' on field '{}' - it is not an object",
                                    field_path[1], requested_field
                                ),
                                file: self.file_path.clone(),
                                line: None,
                                column: None,
                                context: Some(format!(
                                    "The field '{}' is a simple value, not an object with properties",
                                    requested_field
                                )),
                                documentation_link: None,
                            });
                        }
                    }
                }
            } else {
                self.result.errors.push(DoctorError {
                    message: format!("Reference to undefined action '{}'", action_name),
                    file: self.file_path.clone(),
                    line: None,
                    column: None,
                    context: Some("Make sure the action is defined before using it in outputs".to_string()),
                    documentation_link: None,
                });
            }
        }
    }
}

impl<'a> RunbookVisitor for ValidationVisitor<'a> {
    fn visit_runbook(&mut self, runbook: &Runbook) {
        // First pass: collect all action definitions
        for action in &runbook.actions {
            self.visit_action(action);
        }
        
        // Second pass: validate outputs
        for output in &runbook.outputs {
            self.visit_output(output);
        }
        
        // Visit other blocks
        for variable in &runbook.variables {
            self.visit_variable(variable);
        }
        
        for signer in &runbook.signers {
            self.visit_signer(signer);
        }
    }
    
    fn visit_action(&mut self, action: &ActionBlock) {
        // Update context
        self.current_block_type = "action".to_string();
        self.current_block_name = action.name.clone();
        
        // Record action type
        self.action_types.insert(action.name.clone(), action.action_type.clone());
        
        // Get specification for this action
        let parts: Vec<&str> = action.action_type.split("::").collect();
        if parts.len() == 2 {
            let addon_name = parts[0];
            let action_name = parts[1];
            
            if let Some(addon_actions) = self.addon_specs.get(addon_name) {
                if let Some((_, spec)) = addon_actions.iter()
                    .find(|(matcher, _)| matcher == &action_name) {
                    self.action_specs.insert(action.name.clone(), spec.clone());
                }
            }
        }
        
        // Visit action attributes
        self.visit_attributes(&action.attributes);
    }
    
    fn visit_output(&mut self, output: &OutputBlock) {
        // Update context
        self.current_block_type = "output".to_string();
        self.current_block_name = output.name.clone();
        
        // Visit all attributes (including value)
        self.visit_attributes(&output.attributes);
    }
    
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Reference(parts) => {
                // Check if it's an input reference
                if parts.len() >= 2 && parts[0] == "input" {
                    // For now, just collect the name. We'll find the location later
                    // using text search when we know which inputs are undefined
                    self.input_refs.push(LocatedInputRef {
                        name: parts.join("."),
                        line: 0, // Will be filled in later
                        column: 0,
                    });
                }
                
                self.validate_action_reference(parts);
            }
            Expression::Array(items) => {
                for item in items {
                    self.visit_expression(item);
                }
            }
            Expression::Object(fields) => {
                for (_, value) in fields {
                    self.visit_expression(value);
                }
            }
            Expression::FunctionCall { name: _, args } => {
                for arg in args {
                    self.visit_expression(arg);
                }
            }
            _ => {} // Literals don't need validation
        }
    }
}

/// Run validation using the visitor pattern
pub fn validate_with_visitor(
    runbook: &Runbook,
    result: &mut DoctorResult,
    file_path: &str,
) -> Vec<LocatedInputRef> {
    let mut visitor = ValidationVisitor::new(result, file_path);
    visitor.visit_runbook(runbook);
    visitor.input_refs
}
