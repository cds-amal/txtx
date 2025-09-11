#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashSet;

    /// Custom visitor to collect flow information
    struct FlowCollector {
        pub flow_names: HashSet<String>,
        pub flow_attributes: Vec<(String, String, String)>, // (flow_name, key, value_type)
    }

    impl FlowCollector {
        fn new() -> Self {
            Self {
                flow_names: HashSet::new(),
                flow_attributes: Vec::new(),
            }
        }
    }

    impl RunbookVisitor for FlowCollector {
        fn visit_flow(&mut self, flow: &FlowBlock) {
            self.flow_names.insert(flow.name.clone());
            
            for (key, expr) in &flow.attributes {
                let value_type = match expr {
                    Expression::String(_) => "string",
                    Expression::Number(_) => "number",
                    Expression::Bool(_) => "bool",
                    Expression::Reference(_) => "reference",
                    Expression::Array(_) => "array",
                    Expression::Object(_) => "object",
                    Expression::FunctionCall { .. } => "function",
                };
                self.flow_attributes.push((flow.name.clone(), key.clone(), value_type.to_string()));
            }
            
            // Call default implementation to continue traversal
            self.visit_attributes(&flow.attributes);
        }
    }

    #[test]
    fn test_flow_parsing() {
        let source = r#"
flow "mainnet" {
    description = "Production environment"
    chain_id = 1
    rpc_url = "https://mainnet.infura.io"
}

flow "testnet" {
    description = "Test environment"
    chain_id = 5
    rpc_url = "https://goerli.infura.io"
}

addon "evm" {
    chain_id = input.chain_id
}
"#;

        let runbook = parse(source).expect("Failed to parse");
        
        // Verify flows were parsed
        assert_eq!(runbook.flows.len(), 2);
        assert_eq!(runbook.flows[0].name, "mainnet");
        assert_eq!(runbook.flows[1].name, "testnet");
        
        // Verify flow attributes
        let mainnet_flow = &runbook.flows[0];
        assert_eq!(mainnet_flow.attributes.len(), 3);
        assert!(mainnet_flow.attributes.contains_key("description"));
        assert!(mainnet_flow.attributes.contains_key("chain_id"));
        assert!(mainnet_flow.attributes.contains_key("rpc_url"));
    }

    #[test]
    fn test_flow_visitor_traversal() {
        let source = r#"
flow "mainnet" {
    description = "Production"
    chain_id = 1
}

flow "testnet" {
    description = "Testing"
    chain_id = 5
}
"#;

        let runbook = parse(source).expect("Failed to parse");
        
        let mut collector = FlowCollector::new();
        collector.visit_runbook(&runbook);
        
        // Verify visitor found all flows
        assert_eq!(collector.flow_names.len(), 2);
        assert!(collector.flow_names.contains("mainnet"));
        assert!(collector.flow_names.contains("testnet"));
        
        // Verify visitor collected attributes
        assert_eq!(collector.flow_attributes.len(), 4); // 2 flows × 2 attributes each
        
        let mainnet_attrs: Vec<_> = collector.flow_attributes.iter()
            .filter(|(flow, _, _)| flow == "mainnet")
            .collect();
        assert_eq!(mainnet_attrs.len(), 2);
    }

    #[test]
    fn test_flow_builder() {
        let runbook = RunbookBuilder::new()
            .flow("mainnet")
                .description("Production environment")
                .chain_id(Expression::number(1))
                .rpc_url(Expression::string("https://mainnet.infura.io"))
                .done()
            .flow("testnet")
                .description("Test environment")
                .chain_id(Expression::number(5))
                .rpc_url(Expression::string("https://goerli.infura.io"))
                .done()
            .build();
        
        assert_eq!(runbook.flows.len(), 2);
        assert_eq!(runbook.flows[0].name, "mainnet");
        assert_eq!(runbook.flows[1].name, "testnet");
    }

    #[test]
    fn test_flow_transform() {
        let mut runbook = RunbookBuilder::new()
            .flow("mainnet")
                .chain_id(Expression::input_ref("chain_id"))
                .done()
            .build();
        
        // Custom transform to replace input references
        struct InputReplacer {
            replacement: Expression,
        }
        
        impl RunbookTransform for InputReplacer {
            fn transform_expression(&mut self, expr: Expression) -> Expression {
                match expr {
                    Expression::Reference(parts) if parts.get(0) == Some(&"input".to_string()) => {
                        self.replacement.clone()
                    }
                    _ => expr,
                }
            }
        }
        
        let mut replacer = InputReplacer {
            replacement: Expression::number(1),
        };
        
        replacer.transform_runbook(&mut runbook);
        
        // Verify transform was applied to flow
        let chain_id = runbook.flows[0].attributes.get("chain_id").unwrap();
        assert_eq!(chain_id, &Expression::number(1));
    }

    #[test]
    fn test_flow_renderer() {
        let runbook = RunbookBuilder::new()
            .flow("mainnet")
                .description("Production")
                .chain_id(Expression::number(1))
                .done()
            .build();
        
        let mut renderer = RunbookRenderer::new();
        let output = renderer.render(&runbook);
        
        assert!(output.contains("flow \"mainnet\""));
        assert!(output.contains("description = \"Production\""));
        assert!(output.contains("chain_id = 1"));
    }

    #[test]
    fn test_complete_flow_support() {
        // Test that all components work together
        let source = r#"
module "metadata" {
    name = "Test Runbook"
    version = "1.0.0"
}

flow "mainnet" {
    description = "Production environment"
    chain_id = 1
    rpc_url = "https://mainnet.infura.io"
}

flow "testnet" {
    description = "Test environment"  
    chain_id = 5
    rpc_url = "https://goerli.infura.io"
}

addon "evm" {
    chain_id = input.chain_id
    rpc_url = input.rpc_url
}

action "deploy" "evm::deploy" {
    bytecode = "0x123"
}

runbook "child" {
    location = "./child.tx"
}
"#;
        
        // Parse
        let mut runbook = parse(source).expect("Failed to parse");
        
        // Verify all block types present
        assert_eq!(runbook.modules.len(), 1);
        assert_eq!(runbook.flows.len(), 2);
        assert_eq!(runbook.addons.len(), 1);
        assert_eq!(runbook.actions.len(), 1);
        assert_eq!(runbook.runbook_blocks.len(), 1);
        
        // Visit and collect
        let mut collector = FlowCollector::new();
        collector.visit_runbook(&runbook);
        assert_eq!(collector.flow_names.len(), 2);
        
        // Transform
        struct FlowTransformer;
        impl RunbookTransform for FlowTransformer {
            fn transform_flow(&mut self, flow: &mut FlowBlock) {
                // Add a new attribute to each flow
                flow.attributes.insert(
                    "transformed".to_string(),
                    Expression::Bool(true)
                );
                // Call default to transform existing attributes
                let mut new_attrs = HashMap::new();
                for (key, expr) in flow.attributes.drain() {
                    let new_expr = self.transform_expression(expr);
                    new_attrs.insert(key, new_expr);
                }
                flow.attributes = new_attrs;
            }
        }
        
        let mut transformer = FlowTransformer;
        transformer.transform_runbook(&mut runbook);
        
        // Verify transformation
        for flow in &runbook.flows {
            assert!(flow.attributes.contains_key("transformed"));
        }
        
        // Render
        let mut renderer = RunbookRenderer::new();
        let rendered = renderer.render(&runbook);
        
        // Verify rendered output contains all block types
        assert!(rendered.contains("module \"metadata\""));
        assert!(rendered.contains("flow \"mainnet\""));
        assert!(rendered.contains("flow \"testnet\""));
        assert!(rendered.contains("addon \"evm\""));
        assert!(rendered.contains("action \"deploy\""));
        assert!(rendered.contains("runbook \"child\""));
        assert!(rendered.contains("transformed = true"));
    }
}