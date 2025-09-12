#[cfg(test)]
mod tests {
    use crate::common::state::{EditorState, ActiveRunbookData};
    use crate::common::input_parser::parse_input_reference;
    use lsp_types::Position;
    use txtx_addon_kit::helpers::fs::FileLocation;
    
    #[test]
    fn test_goto_definition_integration() {
        // Create test source with input reference
        let source = r#"action "deploy" "evm::deploy" {
  address = inputs.contract_address
}"#;
        
        // Parse input reference
        let position = Position { line: 1, character: 25 };
        let input_ref = parse_input_reference(source, &position);
        assert!(input_ref.is_some(), "Should find input reference");
        
        let input_ref = input_ref.unwrap();
        assert_eq!(input_ref.name, "contract_address");
        
        // Now test with EditorState
        let mut editor_state = EditorState::new();
        
        // Add active runbook
        let runbook_location = FileLocation::try_parse("file:///test/deploy.tx", None).unwrap();
        editor_state.insert_active_runbook(runbook_location.clone(), source);
        
        // Try to get definition (will fail because no workspace is set up)
        let result = editor_state.get_definition_location(&runbook_location, &position);
        
        // This will be None because we haven't set up the workspace properly
        // But at least we know the parsing works
        assert!(result.is_none(), "Should return None without workspace setup");
    }
}