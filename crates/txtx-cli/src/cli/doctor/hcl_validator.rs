//! HCL-based validation for the doctor command using hcl-edit
//! 
//! This module uses hcl-edit's visitor pattern to perform comprehensive
//! validation of runbook files, replacing the Tree-sitter based approach.

use std::collections::{HashMap, HashSet, VecDeque};

use txtx_addon_kit::hcl::{
    visit::{Visit, visit_block, visit_expr, visit_body},
    structure::{Block, Body, BlockLabel},
    expr::{Expression, Traversal, TraversalOperator},
    Span,
};

use txtx_core::kit::types::commands::CommandSpecification;

use super::{DoctorResult, DoctorError, DoctorWarning, DoctorSuggestion, LocatedInputRef, get_addon_specifications, get_action_doc_link};

/// A visitor that validates HCL runbooks
pub struct HclValidationVisitor<'a> {
    /// Results collector
    result: &'a mut DoctorResult,
    /// Path to the current file being validated
    file_path: String,
    /// Source content for extracting line/column from spans
    source: &'a str,
    
    // === Collection Phase Data ===
    /// Map of action names to their types
    action_types: HashMap<String, String>,
    /// Map of action names to their specifications
    action_specs: HashMap<String, CommandSpecification>,
    /// Addon specifications
    addon_specs: HashMap<String, Vec<(String, CommandSpecification)>>,
    /// Track all variable definitions
    defined_variables: HashSet<String>,
    /// Track all signer definitions  
    defined_signers: HashMap<String, String>,
    /// Track all output definitions
    defined_outputs: HashSet<String>,
    /// Track flow definitions and their inputs
    flow_inputs: HashMap<String, Vec<String>>,
    
    // === Context Tracking ===
    /// Current block being processed
    current_block: Option<BlockContext>,
    /// Whether we're in validation phase (vs collection phase)
    is_validation_phase: bool,
    /// Collected input references
    pub input_refs: Vec<LocatedInputRef>,
}

#[derive(Clone, Debug)]
struct BlockContext {
    block_type: String,
    name: String,
    span: Option<std::ops::Range<usize>>,
}

impl<'a> HclValidationVisitor<'a> {
    pub fn new(result: &'a mut DoctorResult, file_path: &str, source: &'a str) -> Self {
        Self {
            result,
            file_path: file_path.to_string(),
            source,
            action_types: HashMap::new(),
            action_specs: HashMap::new(),
            addon_specs: get_addon_specifications(),
            defined_variables: HashSet::new(),
            defined_signers: HashMap::new(),
            defined_outputs: HashSet::new(),
            flow_inputs: HashMap::new(),
            current_block: None,
            is_validation_phase: false,
            input_refs: Vec::new(),
        }
    }
    
    /// Convert a span to line/column position
    fn span_to_position(&self, span: &std::ops::Range<usize>) -> (usize, usize) {
        let start = span.start;
        let mut line = 1;
        let mut col = 1;
        
        for (i, ch) in self.source.char_indices() {
            if i >= start {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        
        (line, col)
    }
    
    /// Process a block based on its type
    fn process_block(&mut self, block: &Block) {
        let block_type = block.ident.value().as_str();
        
        // Set current context with span
        let span = block.span();
        self.current_block = Some(BlockContext {
            block_type: block_type.to_string(),
            name: String::new(), // Will be filled based on block type
            span,
        });
        
        match block_type {
            "addon" => {
                // Addon blocks don't need tracking for validation
            }
            "signer" => {
                if let Some(BlockLabel::String(name)) = block.labels.get(0) {
                    if let Some(ctx) = &mut self.current_block {
                        ctx.name = name.value().to_string();
                    }
                    
                    if !self.is_validation_phase {
                        // Collection phase: record signer
                        if let Some(BlockLabel::String(signer_type)) = block.labels.get(1) {
                            self.defined_signers.insert(
                                name.value().to_string(), 
                                signer_type.value().to_string()
                            );
                        }
                    }
                }
            }
            "action" => {
                if let Some(BlockLabel::String(name)) = block.labels.get(0) {
                    if let Some(ctx) = &mut self.current_block {
                        ctx.name = name.value().to_string();
                    }
                    
                    if !self.is_validation_phase {
                        // Collection phase: record action and its type
                        if let Some(BlockLabel::String(action_type)) = block.labels.get(1) {
                            let name_str = name.value().to_string();
                            let type_str = action_type.value().to_string();
                            
                            self.action_types.insert(name_str.clone(), type_str.clone());
                            
                            // Get specification for this action
                            if let Some((namespace, action_name)) = type_str.split_once("::") {
                                if let Some(addon_actions) = self.addon_specs.get(namespace) {
                                    if let Some((_, spec)) = addon_actions.iter()
                                        .find(|(matcher, _)| matcher == action_name) {
                                        self.action_specs.insert(name_str, spec.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            "output" => {
                if let Some(BlockLabel::String(name)) = block.labels.get(0) {
                    if let Some(ctx) = &mut self.current_block {
                        ctx.name = name.value().to_string();
                    }
                    
                    if !self.is_validation_phase {
                        self.defined_outputs.insert(name.value().to_string());
                    }
                }
            }
            "variable" => {
                if let Some(BlockLabel::String(name)) = block.labels.get(0) {
                    if let Some(ctx) = &mut self.current_block {
                        ctx.name = name.value().to_string();
                    }
                    
                    if !self.is_validation_phase {
                        self.defined_variables.insert(name.value().to_string());
                    }
                }
            }
            "flow" => {
                if let Some(BlockLabel::String(name)) = block.labels.get(0) {
                    if let Some(ctx) = &mut self.current_block {
                        ctx.name = name.value().to_string();
                    }
                    
                    if !self.is_validation_phase {
                        // Collect flow inputs
                        let mut inputs = Vec::new();
                        for attr in block.body.attributes() {
                            if attr.key.as_str() != "description" {
                                inputs.push(attr.key.to_string());
                            }
                        }
                        self.flow_inputs.insert(name.value().to_string(), inputs);
                    }
                }
            }
            _ => {}
        }
    }
    
    /// Validate a traversal expression
    fn validate_traversal(&mut self, traversal: &Traversal) {
        if !self.is_validation_phase {
            return;
        }
        
        // Get the root variable
        let Some(root) = traversal.expr.as_variable() else {
            return;
        };
        
        // Build the full path
        let mut parts = VecDeque::new();
        parts.push_back(root.to_string());
        
        for op in traversal.operators.iter() {
            if let TraversalOperator::GetAttr(attr) = op.value() {
                parts.push_back(attr.to_string());
            }
        }
        
        if parts.is_empty() {
            return;
        }
        
        let (line, col) = self.current_block
            .as_ref()
            .and_then(|ctx| ctx.span.as_ref())
            .map(|span| self.span_to_position(span))
            .unwrap_or((0, 0));
        
        match parts[0].as_str() {
            "input" => {
                // Collect input reference for later validation
                let parts_vec: Vec<String> = parts.into_iter().collect();
                let (line, col) = self.current_block
                    .as_ref()
                    .and_then(|ctx| ctx.span.as_ref())
                    .map(|span| self.span_to_position(span))
                    .unwrap_or((0, 0));
                
                self.input_refs.push(LocatedInputRef {
                    name: parts_vec.join("."),
                    line,
                    column: col,
                });
            }
            "action" => {
                if parts.len() >= 2 {
                    let action_name = &parts[1];
                    if !self.action_types.contains_key(action_name) {
                        self.result.errors.push(DoctorError {
                            message: format!("Reference to undefined action '{}'", action_name),
                            file: self.file_path.clone(),
                            line: if line > 0 { Some(line) } else { None },
                            column: if col > 0 { Some(col) } else { None },
                            context: Some("Make sure the action is defined before using it".to_string()),
                            documentation_link: None,
                        });
                    } else if parts.len() >= 3 {
                        // Validate field access
                        self.validate_action_field_access(action_name, &parts[2], line, col);
                    }
                }
            }
            "signer" => {
                if parts.len() >= 2 {
                    let signer_name = &parts[1];
                    if !self.defined_signers.contains_key(signer_name) {
                        self.result.errors.push(DoctorError {
                            message: format!("Reference to undefined signer '{}'", signer_name),
                            file: self.file_path.clone(),
                            line: if line > 0 { Some(line) } else { None },
                            column: if col > 0 { Some(col) } else { None },
                            context: Some("Signers must be defined before they can be referenced".to_string()),
                            documentation_link: None,
                        });
                    }
                }
            }
            "variable" => {
                if parts.len() >= 2 {
                    let var_name = &parts[1];
                    if !self.defined_variables.contains(var_name) {
                        self.result.errors.push(DoctorError {
                            message: format!("Reference to undefined variable '{}'", var_name),
                            file: self.file_path.clone(),
                            line: if line > 0 { Some(line) } else { None },
                            column: if col > 0 { Some(col) } else { None },
                            context: Some("Variables must be defined before they can be referenced".to_string()),
                            documentation_link: None,
                        });
                    }
                }
            }
            "output" => {
                // Output references need ordering validation
                if parts.len() >= 2 {
                    let output_name = &parts[1];
                    // In output context, check for circular dependencies
                    if let Some(ctx) = &self.current_block {
                        if ctx.block_type == "output" && !self.defined_outputs.contains(output_name) {
                            self.result.errors.push(DoctorError {
                                message: format!("Output '{}' references undefined output '{}'", ctx.name, output_name),
                                file: self.file_path.clone(),
                                line: if line > 0 { Some(line) } else { None },
                                column: if col > 0 { Some(col) } else { None },
                                context: Some("Outputs can only reference previously defined outputs".to_string()),
                                documentation_link: None,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    /// Validate action field access
    fn validate_action_field_access(&mut self, action_name: &str, field: &str, line: usize, col: usize) {
        if let Some(spec) = self.action_specs.get(action_name) {
            let output_names: Vec<String> = spec.outputs.iter()
                .map(|o| o.name.clone())
                .collect();
            
            if !output_names.contains(&field.to_string()) {
                let action_type = self.action_types.get(action_name).unwrap();
                self.result.errors.push(DoctorError {
                    message: format!(
                        "Field '{}' does not exist on action '{}' ({}). Available outputs: {}",
                        field, action_name, action_type, output_names.join(", ")
                    ),
                    file: self.file_path.clone(),
                    line: if line > 0 { Some(line) } else { None },
                    column: if col > 0 { Some(col) } else { None },
                    context: None,
                    documentation_link: Some(get_action_doc_link(action_type)),
                });
            }
        }
    }
    
    /// Run two-pass validation on the body
    pub fn validate(&mut self, body: &Body) {
        // Pass 1: Collection phase
        self.is_validation_phase = false;
        self.visit_body(body);
        
        // Pass 2: Validation phase
        self.is_validation_phase = true;
        self.visit_body(body);
    }
}

impl<'a> Visit for HclValidationVisitor<'a> {
    fn visit_block(&mut self, block: &Block) {
        self.process_block(block);
        
        // Continue visiting the block's contents
        visit_block(self, block);
    }
    
    fn visit_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Traversal(traversal) => {
                self.validate_traversal(traversal);
            }
            _ => {}
        }
        
        // Continue visiting nested expressions
        visit_expr(self, expr);
    }
}

/// Run HCL-based validation on a runbook
pub fn validate_with_hcl(
    content: &str,
    result: &mut DoctorResult,
    file_path: &str,
) -> Result<Vec<LocatedInputRef>, String> {
    // Parse the content as HCL
    let body: Body = content.parse()
        .map_err(|e| format!("Failed to parse runbook: {}", e))?;
    
    // Create and run the validator
    let mut visitor = HclValidationVisitor::new(result, file_path, content);
    visitor.validate(&body);
    
    Ok(visitor.input_refs)
}