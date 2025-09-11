//! Visitor-pattern based validation for the doctor command
//! 
//! This module uses the txtx-parser's visitor pattern to perform comprehensive
//! AST-based validation of runbook files.

use std::collections::{HashMap, HashSet};

use txtx_parser::{RunbookVisitor, Expression, ActionBlock, OutputBlock, Runbook, 
                  FlowBlock, ModuleBlock, RunbookBlock, SignerBlock, VariableDeclaration};
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
    /// Track which flow context we're in (if any)
    current_flow: Option<String>,
    /// Map of flow names to their defined inputs
    flow_inputs: HashMap<String, Vec<String>>,
    /// Track all variable definitions
    defined_variables: HashMap<String, String>,
    /// Track all signer definitions
    defined_signers: HashMap<String, String>,
    /// Track construct definition order for dependency validation
    definition_order: Vec<(String, String)>,
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
            current_flow: None,
            flow_inputs: HashMap::new(),
            defined_variables: HashMap::new(),
            defined_signers: HashMap::new(),
            definition_order: Vec::new(),
        }
    }

    /// Validate any reference based on its type
    fn validate_reference(&mut self, parts: &[String]) {
        if parts.is_empty() {
            return;
        }
        
        match parts[0].as_str() {
            "input" => {
                // CLI input reference - collect for later validation
                if parts.len() >= 2 {
                    self.input_refs.push(LocatedInputRef {
                        name: parts.join("."),
                        line: 0, // Will be filled in later
                        column: 0,
                    });
                }
            }
            "variable" => {
                self.validate_variable_reference(parts);
            }
            "flow" => {
                self.validate_flow_reference(parts);
            }
            "action" => {
                self.validate_action_reference(parts);
            }
            "signer" => {
                self.validate_signer_reference(parts);
            }
            "output" => {
                self.validate_output_reference(parts);
            }
            "module" => {
                // Module references are typically valid
            }
            _ => {
                // Could be a bare identifier or unknown reference
            }
        }
    }
    
    /// Validate a variable reference
    fn validate_variable_reference(&mut self, parts: &[String]) {
        if parts.len() >= 2 {
            let var_name = &parts[1];
            if !self.defined_variables.contains_key(var_name) {
                self.result.errors.push(DoctorError {
                    message: format!("Reference to undefined variable '{}'", var_name),
                    file: self.file_path.clone(),
                    line: None,
                    column: None,
                    context: Some("Variables must be defined before they can be referenced".to_string()),
                    documentation_link: None,
                });
            }
        }
    }
    
    /// Validate a flow input reference
    fn validate_flow_reference(&mut self, parts: &[String]) {
        if parts.len() < 2 {
            return;
        }
        
        // Flow references are valid everywhere because a flow is selected at runtime
        // and its attributes become available throughout the runbook execution
        
        let attribute_name = &parts[1];
        
        // If we have flow definitions, check if the attribute exists in at least one flow
        // (ideally should exist in all flows for consistency, but that's a warning not an error)
        if !self.flow_inputs.is_empty() {
            let mut found_in_any = false;
            let mut found_in_all = true;
            let mut flows_with_attr = Vec::new();
            let mut flows_without_attr = Vec::new();
            
            for (flow_name, attributes) in &self.flow_inputs {
                if attributes.contains(&attribute_name.to_string()) {
                    found_in_any = true;
                    flows_with_attr.push(flow_name.clone());
                } else {
                    found_in_all = false;
                    flows_without_attr.push(flow_name.clone());
                }
            }
            
            if !found_in_any {
                // Attribute doesn't exist in any flow
                self.result.errors.push(DoctorError {
                    message: format!(
                        "Flow attribute '{}' is not defined in any flow",
                        attribute_name
                    ),
                    file: self.file_path.clone(),
                    line: None,
                    column: None,
                    context: Some(format!(
                        "Available flow attributes: {}",
                        self.flow_inputs.values()
                            .flat_map(|attrs| attrs.iter())
                            .collect::<std::collections::HashSet<_>>()
                            .into_iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                    documentation_link: Some("https://docs.txtx.sh/language#flows".to_string()),
                });
            } else if !found_in_all && flows_without_attr.len() > 0 {
                // Attribute exists in some flows but not all - this is an ERROR
                // because the runbook will fail when executing the flow that's missing it
                self.result.errors.push(DoctorError {
                    message: format!(
                        "Flow attribute '{}' is missing in some flows",
                        attribute_name
                    ),
                    file: self.file_path.clone(),
                    line: None,
                    column: None,
                    context: Some(format!(
                        "The attribute '{}' exists in flows [{}] but is missing from flows [{}]. Since all flows are executed sequentially, this will cause a runtime error.",
                        attribute_name,
                        flows_with_attr.join(", "),
                        flows_without_attr.join(", ")
                    )),
                    documentation_link: Some("https://docs.txtx.sh/language#flows".to_string()),
                });
            }
        }
        // If no flows are defined, we can't validate - the default flow will be created at runtime
    }
    
    /// Validate a signer reference
    fn validate_signer_reference(&mut self, parts: &[String]) {
        if parts.len() >= 2 {
            let signer_name = &parts[1];
            if !self.defined_signers.contains_key(signer_name) {
                self.result.errors.push(DoctorError {
                    message: format!("Reference to undefined signer '{}'", signer_name),
                    file: self.file_path.clone(),
                    line: None,
                    column: None,
                    context: Some("Signers must be defined before they can be referenced".to_string()),
                    documentation_link: None,
                });
            }
        }
    }
    
    /// Validate an output reference (check for circular dependencies)
    fn validate_output_reference(&mut self, parts: &[String]) {
        if parts.len() >= 2 {
            let output_name = &parts[1];
            // In the current output context, referencing another output that comes later is invalid
            if self.current_block_type == "output" {
                // Check if the referenced output is defined before the current one
                let mut found = false;
                for (construct_type, construct_name) in &self.definition_order {
                    if construct_type == "output" && construct_name == output_name {
                        found = true;
                        break;
                    }
                    if construct_type == "output" && construct_name == &self.current_block_name {
                        // We've reached the current output without finding the referenced one
                        break;
                    }
                }
                
                if !found {
                    self.result.errors.push(DoctorError {
                        message: format!(
                            "Output '{}' references output '{}' which is not yet defined",
                            self.current_block_name, output_name
                        ),
                        file: self.file_path.clone(),
                        line: None,
                        column: None,
                        context: Some("Outputs can only reference previously defined outputs to avoid circular dependencies".to_string()),
                        documentation_link: None,
                    });
                }
            }
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
        // First pass: collect all definitions in order
        // This allows us to validate forward references
        
        // Visit modules first (they define metadata)
        for module in &runbook.modules {
            self.definition_order.push(("module".to_string(), module.name.clone()));
            self.visit_module(module);
        }
        
        // Visit flows to collect their inputs
        for flow in &runbook.flows {
            self.definition_order.push(("flow".to_string(), flow.name.clone()));
            // Collect flow inputs but don't validate yet
            let mut flow_inputs = Vec::new();
            for (key, _) in &flow.attributes {
                if key != "description" {
                    flow_inputs.push(key.clone());
                }
            }
            self.flow_inputs.insert(flow.name.clone(), flow_inputs);
        }
        
        // Visit variables
        for variable in &runbook.variables {
            self.definition_order.push(("variable".to_string(), variable.name.clone()));
            self.defined_variables.insert(variable.name.clone(), "defined".to_string());
            self.visit_variable(variable);
        }
        
        // Visit signers
        for signer in &runbook.signers {
            self.definition_order.push(("signer".to_string(), signer.name.clone()));
            self.defined_signers.insert(signer.name.clone(), signer.signer_type.clone());
            self.visit_signer(signer);
        }
        
        // Visit actions
        for action in &runbook.actions {
            self.definition_order.push(("action".to_string(), action.name.clone()));
            self.visit_action(action);
        }
        
        // Visit outputs
        for output in &runbook.outputs {
            self.definition_order.push(("output".to_string(), output.name.clone()));
            self.visit_output(output);
        }
        
        // Now visit flows with full context
        for flow in &runbook.flows {
            self.visit_flow(flow);
        }
        
        // Visit embedded runbooks
        for runbook_block in &runbook.runbook_blocks {
            self.visit_runbook_block(runbook_block);
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
    
    fn visit_flow(&mut self, flow: &FlowBlock) {
        // Set flow context
        let previous_flow = self.current_flow.clone();
        let previous_block_type = self.current_block_type.clone();
        let previous_block_name = self.current_block_name.clone();
        
        self.current_flow = Some(flow.name.clone());
        self.current_block_type = "flow".to_string();
        self.current_block_name = flow.name.clone();
        
        // Visit flow attributes (they can reference inputs and variables)
        self.visit_attributes(&flow.attributes);
        
        // Restore previous context
        self.current_flow = previous_flow;
        self.current_block_type = previous_block_type;
        self.current_block_name = previous_block_name;
    }
    
    fn visit_module(&mut self, module: &ModuleBlock) {
        let previous_block_type = self.current_block_type.clone();
        let previous_block_name = self.current_block_name.clone();
        
        self.current_block_type = "module".to_string();
        self.current_block_name = module.name.clone();
        
        self.visit_attributes(&module.attributes);
        
        self.current_block_type = previous_block_type;
        self.current_block_name = previous_block_name;
    }
    
    fn visit_variable(&mut self, variable: &VariableDeclaration) {
        let previous_block_type = self.current_block_type.clone();
        let previous_block_name = self.current_block_name.clone();
        
        self.current_block_type = "variable".to_string();
        self.current_block_name = variable.name.clone();
        
        self.visit_attributes(&variable.attributes);
        
        self.current_block_type = previous_block_type;
        self.current_block_name = previous_block_name;
    }
    
    fn visit_signer(&mut self, signer: &SignerBlock) {
        let previous_block_type = self.current_block_type.clone();
        let previous_block_name = self.current_block_name.clone();
        
        self.current_block_type = "signer".to_string();
        self.current_block_name = signer.name.clone();
        
        self.visit_attributes(&signer.attributes);
        
        self.current_block_type = previous_block_type;
        self.current_block_name = previous_block_name;
    }
    
    fn visit_runbook_block(&mut self, runbook_block: &RunbookBlock) {
        let previous_block_type = self.current_block_type.clone();
        let previous_block_name = self.current_block_name.clone();
        
        self.current_block_type = "runbook".to_string();
        self.current_block_name = runbook_block.name.clone();
        
        self.visit_attributes(&runbook_block.attributes);
        
        self.current_block_type = previous_block_type;
        self.current_block_name = previous_block_name;
    }
    
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Reference(parts) => {
                // Validate all types of references
                self.validate_reference(parts);
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
