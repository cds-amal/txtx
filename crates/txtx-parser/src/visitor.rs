//! Visitor pattern for traversing txtx AST

use crate::ast::*;

/// Trait for visiting AST nodes
pub trait RunbookVisitor {
    fn visit_runbook(&mut self, runbook: &Runbook) {
        for addon in &runbook.addons {
            self.visit_addon(addon);
        }
        for signer in &runbook.signers {
            self.visit_signer(signer);
        }
        for action in &runbook.actions {
            self.visit_action(action);
        }
        for output in &runbook.outputs {
            self.visit_output(output);
        }
        for variable in &runbook.variables {
            self.visit_variable(variable);
        }
    }

    fn visit_addon(&mut self, addon: &AddonBlock) {
        self.visit_attributes(&addon.attributes);
    }

    fn visit_signer(&mut self, signer: &SignerBlock) {
        self.visit_attributes(&signer.attributes);
    }

    fn visit_action(&mut self, action: &ActionBlock) {
        self.visit_attributes(&action.attributes);
    }

    fn visit_output(&mut self, output: &OutputBlock) {
        self.visit_attributes(&output.attributes);
    }

    fn visit_variable(&mut self, variable: &VariableDeclaration) {
        self.visit_attributes(&variable.attributes);
    }

    fn visit_attributes(&mut self, attrs: &std::collections::HashMap<String, Expression>) {
        for (key, expr) in attrs {
            self.visit_attribute(key, expr);
        }
    }

    fn visit_attribute(&mut self, _key: &str, expr: &Expression) {
        self.visit_expression(expr);
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
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
            Expression::FunctionCall { args, .. } => {
                for arg in args {
                    self.visit_expression(arg);
                }
            }
            _ => {}
        }
    }
}
