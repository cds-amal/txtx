//! Helper functions for getting addon specifications and documentation links

use std::collections::HashMap;
use crate::kit::types::commands::{CommandSpecification, PreCommandSpecification};

/// Get addon specifications for validation
/// This is a placeholder that should be populated based on available addons
pub fn get_addon_specifications() -> HashMap<String, Vec<(String, CommandSpecification)>> {
    let mut specifications = HashMap::new();
    
    // TODO: This should dynamically load specifications from registered addons
    // For now, we'll return an empty map and the actual loading will be done
    // by the consumers (CLI and LSP) who have access to the addon instances
    
    specifications
}

/// Get documentation link for an action
pub fn get_action_doc_link(namespace: &str, action: &str) -> Option<String> {
    match namespace {
        "bitcoin" => Some(format!("https://docs.txtx.sh/addons/bitcoin/actions#{}", action)),
        "evm" => Some(format!("https://docs.txtx.sh/addons/evm/actions#{}", action)),
        "stacks" => Some(format!("https://docs.txtx.sh/addons/stacks/actions#{}", action)),
        "svm" => Some(format!("https://docs.txtx.sh/addons/svm/actions#{}", action)),
        "ovm" => Some(format!("https://docs.txtx.sh/addons/ovm/actions#{}", action)),
        "telegram" => Some(format!("https://docs.txtx.sh/addons/telegram/actions#{}", action)),
        _ => None,
    }
}