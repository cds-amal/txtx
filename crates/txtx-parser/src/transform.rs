//! AST transformation utilities

use crate::ast::*;
use std::collections::HashMap;

/// Trait for transforming runbooks
pub trait RunbookTransform {
    fn transform_runbook(&mut self, runbook: &mut Runbook) {
        // Transform addons
        for addon in &mut runbook.addons {
            self.transform_addon(addon);
        }
        
        // Transform signers
        for signer in &mut runbook.signers {
            self.transform_signer(signer);
        }
        
        // Transform actions
        for action in &mut runbook.actions {
            self.transform_action(action);
        }
        
        // Transform outputs
        for output in &mut runbook.outputs {
            self.transform_output(output);
        }
        
        // Transform variables
        for var in &mut runbook.variables {
            self.transform_variable(var);
        }
    }
    
    fn transform_addon(&mut self, addon: &mut AddonBlock) {
        let mut new_attrs = HashMap::new();
        for (key, expr) in addon.attributes.drain() {
            let new_expr = self.transform_expression(expr);
            new_attrs.insert(key, new_expr);
        }
        addon.attributes = new_attrs;
    }
    
    fn transform_signer(&mut self, signer: &mut SignerBlock) {
        let mut new_attrs = HashMap::new();
        for (key, expr) in signer.attributes.drain() {
            let new_expr = self.transform_expression(expr);
            new_attrs.insert(key, new_expr);
        }
        signer.attributes = new_attrs;
    }
    
    fn transform_action(&mut self, action: &mut ActionBlock) {
        let mut new_attrs = HashMap::new();
        for (key, expr) in action.attributes.drain() {
            let new_expr = self.transform_expression(expr);
            new_attrs.insert(key, new_expr);
        }
        action.attributes = new_attrs;
    }
    
    fn transform_output(&mut self, output: &mut OutputBlock) {
        let mut new_attrs = HashMap::new();
        for (key, expr) in output.attributes.drain() {
            let new_expr = self.transform_expression(expr);
            new_attrs.insert(key, new_expr);
        }
        output.attributes = new_attrs;
    }
    
    fn transform_variable(&mut self, var: &mut VariableDeclaration) {
        let mut new_attrs = HashMap::new();
        for (key, expr) in var.attributes.drain() {
            let new_expr = self.transform_expression(expr);
            new_attrs.insert(key, new_expr);
        }
        var.attributes = new_attrs;
    }
    
    fn transform_expression(&mut self, expr: Expression) -> Expression {
        match expr {
            Expression::Array(items) => {
                Expression::Array(items.into_iter()
                    .map(|e| self.transform_expression(e))
                    .collect())
            }
            Expression::Object(mut fields) => {
                let mut new_fields = HashMap::new();
                for (key, value) in fields.drain() {
                    new_fields.insert(key, self.transform_expression(value));
                }
                Expression::Object(new_fields)
            }
            Expression::FunctionCall { name, args } => {
                Expression::FunctionCall {
                    name,
                    args: args.into_iter()
                        .map(|e| self.transform_expression(e))
                        .collect(),
                }
            }
            _ => expr,
        }
    }
}

/// Transform that replaces input references with specific values
pub struct InputSubstitution {
    pub values: HashMap<String, Expression>,
}

impl InputSubstitution {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    pub fn add_input(&mut self, name: String, value: Expression) {
        self.values.insert(name, value);
    }
}

impl RunbookTransform for InputSubstitution {
    fn transform_expression(&mut self, expr: Expression) -> Expression {
        match expr {
            Expression::Reference(ref parts) if parts.len() >= 2 && parts[0] == "input" => {
                let input_name = &parts[1];
                if let Some(value) = self.values.get(input_name) {
                    value.clone()
                } else {
                    expr
                }
            }
            Expression::Array(items) => {
                Expression::Array(items.into_iter()
                    .map(|e| self.transform_expression(e))
                    .collect())
            }
            Expression::Object(mut fields) => {
                let mut new_fields = HashMap::new();
                for (key, value) in fields.drain() {
                    new_fields.insert(key, self.transform_expression(value));
                }
                Expression::Object(new_fields)
            }
            Expression::FunctionCall { name, args } => {
                Expression::FunctionCall {
                    name,
                    args: args.into_iter()
                        .map(|e| self.transform_expression(e))
                        .collect(),
                }
            }
            _ => expr,
        }
    }
}