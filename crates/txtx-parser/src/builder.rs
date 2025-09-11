use crate::ast::*;
use std::collections::HashMap;

// Implement conversions for common types to Expression
impl From<i32> for Expression {
    fn from(n: i32) -> Self {
        Expression::Number(n as f64)
    }
}

impl From<u32> for Expression {
    fn from(n: u32) -> Self {
        Expression::Number(n as f64)
    }
}

impl From<f64> for Expression {
    fn from(n: f64) -> Self {
        Expression::Number(n)
    }
}

impl From<bool> for Expression {
    fn from(b: bool) -> Self {
        Expression::Bool(b)
    }
}

impl From<&str> for Expression {
    fn from(s: &str) -> Self {
        Expression::String(s.to_string())
    }
}

impl From<String> for Expression {
    fn from(s: String) -> Self {
        Expression::String(s)
    }
}

/// Builder for constructing runbooks programmatically
pub struct RunbookBuilder {
    runbook: Runbook,
}

impl RunbookBuilder {
    pub fn new() -> Self {
        Self { runbook: Runbook::new() }
    }

    pub fn addon(self, network: impl Into<String>) -> AddonBuilder {
        AddonBuilder::new(self, network.into())
    }

    pub fn signer(self, name: impl Into<String>, signer_type: impl Into<String>) -> SignerBuilder {
        SignerBuilder::new(self, name.into(), signer_type.into())
    }

    pub fn action(self, name: impl Into<String>, action_type: impl Into<String>) -> ActionBuilder {
        ActionBuilder::new(self, name.into(), action_type.into())
    }

    pub fn output(self, name: impl Into<String>) -> OutputBuilder {
        OutputBuilder::new(self, name.into())
    }

    pub fn flow(self, name: impl Into<String>) -> FlowBuilder {
        FlowBuilder::new(self, name.into())
    }

    pub fn module(self, name: impl Into<String>) -> ModuleBuilder {
        ModuleBuilder::new(self, name.into())
    }

    pub fn runbook_block(self, name: impl Into<String>) -> RunbookBlockBuilder {
        RunbookBlockBuilder::new(self, name.into())
    }

    pub fn build(self) -> Runbook {
        self.runbook
    }
}

/// Builder for addon blocks
pub struct AddonBuilder {
    parent: RunbookBuilder,
    addon: AddonBlock,
}

impl AddonBuilder {
    fn new(parent: RunbookBuilder, network: String) -> Self {
        Self { parent, addon: AddonBlock { network, attributes: HashMap::new() } }
    }

    pub fn chain_id(self, value: Expression) -> Self {
        self.attr("chain_id", value)
    }

    pub fn rpc_url(self, value: Expression) -> Self {
        self.attr("rpc_api_url", value)
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.addon.attributes.insert(key.to_string(), value);
        self
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.addons.push(self.addon);
        self.parent
    }
}

/// Builder for signer blocks
pub struct SignerBuilder {
    parent: RunbookBuilder,
    signer: SignerBlock,
}

impl SignerBuilder {
    fn new(parent: RunbookBuilder, name: String, signer_type: String) -> Self {
        Self { parent, signer: SignerBlock { name, signer_type, attributes: HashMap::new() } }
    }

    pub fn secret_key(self, value: Expression) -> Self {
        self.attr("secret_key", value)
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.signer.attributes.insert(key.to_string(), value);
        self
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.signers.push(self.signer);
        self.parent
    }
}

/// Builder for action blocks
pub struct ActionBuilder {
    parent: RunbookBuilder,
    action: ActionBlock,
}

impl ActionBuilder {
    fn new(parent: RunbookBuilder, name: String, action_type: String) -> Self {
        Self { parent, action: ActionBlock { name, action_type, attributes: HashMap::new() } }
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.action.attributes.insert(key.to_string(), value);
        self
    }

    pub fn signer(self, signer_name: &str) -> Self {
        self.attr("signer", Expression::signer_ref(signer_name))
    }

    pub fn recipient_address(self, value: Expression) -> Self {
        self.attr("recipient_address", value)
    }

    pub fn amount(self, value: Expression) -> Self {
        self.attr("amount", value)
    }

    pub fn confirmations(self, value: impl Into<Expression>) -> Self {
        self.attr("confirmations", value.into())
    }

    // Additional methods for common action attributes
    pub fn address(self, value: Expression) -> Self {
        self.attr("address", value)
    }

    pub fn contract_abi(self, value: Expression) -> Self {
        self.attr("contract_abi", value)
    }

    pub fn contract_bytecode(self, value: Expression) -> Self {
        self.attr("contract_bytecode", value)
    }

    pub fn constructor_args(self, value: Expression) -> Self {
        self.attr("constructor_args", value)
    }

    pub fn contract_address(self, value: Expression) -> Self {
        self.attr("contract_address", value)
    }

    pub fn function_name(self, value: Expression) -> Self {
        self.attr("function_name", value)
    }

    pub fn function_args(self, value: Expression) -> Self {
        self.attr("function_args", value)
    }

    pub fn recipients(self, value: Expression) -> Self {
        self.attr("recipients", value)
    }

    pub fn amounts(self, value: Expression) -> Self {
        self.attr("amounts", value)
    }

    pub fn from_block(self, value: Expression) -> Self {
        self.attr("from_block", value)
    }

    pub fn to_block(self, value: Expression) -> Self {
        self.attr("to_block", value)
    }

    pub fn topics(self, value: Expression) -> Self {
        self.attr("topics", value)
    }

    pub fn gas_limit(self, value: Expression) -> Self {
        self.attr("gas_limit", value)
    }

    pub fn gas_price(self, value: Expression) -> Self {
        self.attr("gas_price", value)
    }

    pub fn expected_revert(self, value: Expression) -> Self {
        self.attr("expected_revert", value)
    }

    pub fn value(self, value: Expression) -> Self {
        self.attr("value", value)
    }

    pub fn from(self, value: Expression) -> Self {
        self.attr("from", value)
    }

    pub fn to(self, value: Expression) -> Self {
        self.attr("to", value)
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.actions.push(self.action);
        self.parent
    }
}

/// Builder for output blocks
pub struct OutputBuilder {
    parent: RunbookBuilder,
    output: OutputBlock,
}

impl OutputBuilder {
    fn new(parent: RunbookBuilder, name: String) -> Self {
        Self { parent, output: OutputBlock { name, attributes: HashMap::new() } }
    }

    pub fn value(self, expr: Expression) -> Self {
        self.attr("value", expr)
    }

    pub fn description(self, desc: &str) -> Self {
        self.attr("description", Expression::String(desc.to_string()))
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.output.attributes.insert(key.to_string(), value);
        self
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.outputs.push(self.output);
        self.parent
    }
}

/// Builder for flow blocks
pub struct FlowBuilder {
    parent: RunbookBuilder,
    flow: FlowBlock,
}

impl FlowBuilder {
    fn new(parent: RunbookBuilder, name: String) -> Self {
        Self { parent, flow: FlowBlock { name, attributes: HashMap::new() } }
    }

    pub fn description(self, desc: &str) -> Self {
        self.attr("description", Expression::String(desc.to_string()))
    }

    pub fn chain_id(self, value: Expression) -> Self {
        self.attr("chain_id", value)
    }

    pub fn rpc_url(self, value: Expression) -> Self {
        self.attr("rpc_url", value)
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.flow.attributes.insert(key.to_string(), value);
        self
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.flows.push(self.flow);
        self.parent
    }
}

/// Builder for module blocks
pub struct ModuleBuilder {
    parent: RunbookBuilder,
    module: ModuleBlock,
}

impl ModuleBuilder {
    fn new(parent: RunbookBuilder, name: String) -> Self {
        Self { parent, module: ModuleBlock { name, attributes: HashMap::new() } }
    }

    pub fn name(self, name: &str) -> Self {
        self.attr("name", Expression::String(name.to_string()))
    }

    pub fn description(self, desc: &str) -> Self {
        self.attr("description", Expression::String(desc.to_string()))
    }

    pub fn version(self, version: &str) -> Self {
        self.attr("version", Expression::String(version.to_string()))
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.module.attributes.insert(key.to_string(), value);
        self
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.modules.push(self.module);
        self.parent
    }
}

/// Builder for runbook blocks
pub struct RunbookBlockBuilder {
    parent: RunbookBuilder,
    runbook_block: RunbookBlock,
}

impl RunbookBlockBuilder {
    fn new(parent: RunbookBuilder, name: String) -> Self {
        Self { parent, runbook_block: RunbookBlock { name, attributes: HashMap::new() } }
    }

    pub fn location(self, path: &str) -> Self {
        self.attr("location", Expression::String(path.to_string()))
    }

    pub fn inputs(self, inputs: Expression) -> Self {
        self.attr("inputs", inputs)
    }

    pub fn attr(mut self, key: &str, value: Expression) -> Self {
        self.runbook_block.attributes.insert(key.to_string(), value);
        self
    }

    pub fn done(mut self) -> RunbookBuilder {
        self.parent.runbook.runbook_blocks.push(self.runbook_block);
        self.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_api() {
        let runbook = RunbookBuilder::new()
            .addon("evm")
            .chain_id(Expression::Number(31337.0))
            .rpc_url(Expression::String("http://localhost:8545".to_string()))
            .done()
            .signer("test_signer", "evm::secret_key")
            .secret_key(Expression::input_ref("private_key"))
            .done()
            .action("transfer", "evm::send_eth")
            .signer("test_signer")
            .amount(Expression::number(1000000000000000000.0))
            .done()
            .output("tx_hash")
            .value(Expression::action_ref("transfer", "tx_hash"))
            .done()
            .build();

        assert_eq!(runbook.addons.len(), 1);
        assert_eq!(runbook.signers.len(), 1);
        assert_eq!(runbook.actions.len(), 1);
        assert_eq!(runbook.outputs.len(), 1);
    }
}
