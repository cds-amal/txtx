//! Helper functions for getting addon specifications and documentation links

use crate::kit::types::commands::CommandSpecification;
use std::collections::HashMap;

/// Get addon specifications for validation
/// Returns a minimal set of addon namespaces and actions for HCL validation
pub fn get_addon_specifications() -> HashMap<String, Vec<(String, CommandSpecification)>> {
    let mut specifications = HashMap::new();

    // For now, just register the namespaces with empty action lists
    // This will at least fix the "Unknown addon namespace" errors
    // The actual action validation can be added later when we have access to real addon instances

    specifications.insert("evm".to_string(), vec![]);
    specifications.insert("bitcoin".to_string(), vec![]);
    specifications.insert("stacks".to_string(), vec![]);
    specifications.insert("svm".to_string(), vec![]);
    specifications.insert("ovm".to_string(), vec![]);
    specifications.insert("telegram".to_string(), vec![]);
    specifications.insert("sp1".to_string(), vec![]);
    specifications.insert("std".to_string(), vec![]);

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
