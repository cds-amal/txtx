//! Render AST back to txtx source code

use crate::ast::*;
use std::collections::HashMap;
use std::fmt::Write;

pub struct RunbookRenderer {
    indent: usize,
    indent_str: String,
}

impl RunbookRenderer {
    pub fn new() -> Self {
        Self { indent: 0, indent_str: "    ".to_string() }
    }

    pub fn render(&mut self, runbook: &Runbook) -> String {
        let mut out = String::new();

        // Render addons
        for addon in &runbook.addons {
            self.render_addon(&mut out, addon);
            out.push('\n');
        }

        // Render signers
        for signer in &runbook.signers {
            self.render_signer(&mut out, signer);
            out.push('\n');
        }

        // Render actions
        for action in &runbook.actions {
            self.render_action(&mut out, action);
            out.push('\n');
        }

        // Render outputs
        for output in &runbook.outputs {
            self.render_output(&mut out, output);
            out.push('\n');
        }

        // Render variables
        for variable in &runbook.variables {
            self.render_variable(&mut out, variable);
            out.push('\n');
        }

        out
    }

    fn indent(&self) -> String {
        self.indent_str.repeat(self.indent)
    }

    fn render_addon(&mut self, out: &mut String, addon: &AddonBlock) {
        write!(out, "{}addon \"{}\" {{\n", self.indent(), addon.network).unwrap();
        self.indent += 1;
        self.render_attributes(out, &addon.attributes);
        self.indent -= 1;
        write!(out, "{}}}\n", self.indent()).unwrap();
    }

    fn render_signer(&mut self, out: &mut String, signer: &SignerBlock) {
        write!(out, "{}signer \"{}\" \"{}\" {{\n", self.indent(), signer.name, signer.signer_type)
            .unwrap();
        self.indent += 1;
        self.render_attributes(out, &signer.attributes);
        self.indent -= 1;
        write!(out, "{}}}\n", self.indent()).unwrap();
    }

    fn render_action(&mut self, out: &mut String, action: &ActionBlock) {
        write!(out, "{}action \"{}\" \"{}\" {{\n", self.indent(), action.name, action.action_type)
            .unwrap();
        self.indent += 1;
        self.render_attributes(out, &action.attributes);
        self.indent -= 1;
        write!(out, "{}}}\n", self.indent()).unwrap();
    }

    fn render_output(&mut self, out: &mut String, output: &OutputBlock) {
        write!(out, "{}output \"{}\" {{\n", self.indent(), output.name).unwrap();
        self.indent += 1;
        self.render_attributes(out, &output.attributes);
        self.indent -= 1;
        write!(out, "{}}}\n", self.indent()).unwrap();
    }

    fn render_variable(&mut self, out: &mut String, variable: &VariableDeclaration) {
        write!(out, "{}variable \"{}\" {{\n", self.indent(), variable.name).unwrap();
        self.indent += 1;
        self.render_attributes(out, &variable.attributes);
        self.indent -= 1;
        write!(out, "{}}}\n", self.indent()).unwrap();
    }

    fn render_attributes(&mut self, out: &mut String, attrs: &HashMap<String, Expression>) {
        for (key, value) in attrs {
            write!(out, "{}{} = ", self.indent(), key).unwrap();
            self.render_expression(out, value);
            out.push('\n');
        }
    }

    fn render_expression(&mut self, out: &mut String, expr: &Expression) {
        match expr {
            Expression::String(s) => write!(out, "\"{}\"", s).unwrap(),
            Expression::Number(n) => {
                if n.fract() == 0.0 {
                    write!(out, "{}", *n as i64).unwrap();
                } else {
                    write!(out, "{}", n).unwrap();
                }
            }
            Expression::Bool(b) => write!(out, "{}", b).unwrap(),
            Expression::Reference(parts) => write!(out, "{}", parts.join(".")).unwrap(),
            Expression::Array(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    self.render_expression(out, item);
                }
                out.push(']');
            }
            Expression::Object(fields) => {
                out.push('{');
                let mut first = true;
                for (key, value) in fields {
                    if !first {
                        out.push_str(", ");
                    }
                    first = false;
                    write!(out, "{} = ", key).unwrap();
                    self.render_expression(out, value);
                }
                out.push('}');
            }
            Expression::FunctionCall { name, args } => {
                write!(out, "{}(", name).unwrap();
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    self.render_expression(out, arg);
                }
                out.push(')');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::RunbookBuilder;

    #[test]
    fn test_renderer() {
        let runbook = RunbookBuilder::new()
            .addon("evm")
            .chain_id(Expression::Number(1.0))
            .rpc_url(Expression::String("http://localhost:8545".to_string()))
            .done()
            .signer("test_signer", "evm::secret_key")
            .secret_key(Expression::input_ref("private_key"))
            .done()
            .action("transfer", "evm::send_eth")
            .signer("test_signer")
            .recipient_address(Expression::input_ref("recipient"))
            .amount(Expression::Number(1000000000000000000.0))
            .done()
            .output("tx_hash")
            .value(Expression::action_ref("transfer", "tx_hash"))
            .done()
            .build();

        let mut renderer = RunbookRenderer::new();
        let output = renderer.render(&runbook);

        println!("{}", output);
        assert!(output.contains("addon \"evm\""));
        assert!(output.contains("chain_id = 1"));
    }
}
