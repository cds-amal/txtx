use std::collections::HashMap;
use txtx_addon_kit::types::functions::FunctionSpecification;
use txtx_addon_kit::types::commands::{CommandSpecification, PreCommandSpecification};
use txtx_addon_kit::Addon;

/// Get all available addons
fn get_available_addons() -> Vec<Box<dyn Addon>> {
    let mut addons: Vec<Box<dyn Addon>> = vec![];
    
    // Add all available network addons
    addons.push(Box::new(txtx_addon_network_bitcoin::BitcoinNetworkAddon::new()));
    addons.push(Box::new(txtx_addon_network_evm::EvmNetworkAddon::new()));
    // Stacks addon might not be available in LSP context
    // addons.push(Box::new(txtx_addon_network_stacks::StacksNetworkAddon::new()));
    addons.push(Box::new(txtx_addon_network_svm::SvmNetworkAddon::new()));
    addons.push(Box::new(txtx_addon_network_ovm::OvmNetworkAddon::new()));
    
    #[cfg(debug_assertions)]
    eprintln!("LSP: Loaded {} addons", addons.len());
    
    for addon in &addons {
        let functions = addon.get_functions();
        #[cfg(debug_assertions)]
        eprintln!("LSP: Addon '{}' has {} functions", addon.get_namespace(), functions.len());
    }
    
    addons
}

/// Get all available function specifications from addons
pub fn get_function_specifications() -> HashMap<String, Vec<FunctionSpecification>> {
    let mut specs = HashMap::new();
    
    // Get all available addons
    let addons = get_available_addons();
    
    for addon in addons {
        let namespace = addon.get_namespace();
        let functions = addon.get_functions();
        
        if !functions.is_empty() {
            specs.insert(namespace.to_string(), functions);
        }
    }
    
    specs
}

/// Get all available action specifications from addons  
pub fn get_action_specifications() -> HashMap<String, Vec<(String, CommandSpecification)>> {
    let mut specs = HashMap::new();
    
    // Get all available addons
    let addons = get_available_addons();
    
    for addon in addons {
        let namespace = addon.get_namespace();
        let actions = addon.get_actions();
        
        let mut action_specs = Vec::new();
        for action in actions {
            if let PreCommandSpecification::Atomic(spec) = action {
                action_specs.push((spec.matcher.clone(), spec));
            }
        }
        
        if !action_specs.is_empty() {
            specs.insert(namespace.to_string(), action_specs);
        }
    }
    
    specs
}

/// Get specification for a specific function (e.g. "evm::get_contract_from_foundry_project")
pub fn get_function_spec(full_name: &str) -> Option<FunctionSpecification> {
    let parts: Vec<&str> = full_name.split("::").collect();
    if parts.len() != 2 {
        #[cfg(debug_assertions)]
        eprintln!("LSP: Invalid function name format: '{}'", full_name);
        return None;
    }
    
    let namespace = parts[0];
    let function_name = parts[1];
    
    #[cfg(debug_assertions)]
    eprintln!("LSP: Looking up function '{}' in namespace '{}'", function_name, namespace);
    
    let all_specs = get_function_specifications();
    
    #[cfg(debug_assertions)]
    eprintln!("LSP: Available namespaces: {:?}", all_specs.keys().collect::<Vec<_>>());
    
    let namespace_functions = all_specs.get(namespace)?;
    
    #[cfg(debug_assertions)]
    eprintln!("LSP: Found {} functions in namespace '{}'", namespace_functions.len(), namespace);
    
    let result = namespace_functions.iter()
        .find(|f| f.name == function_name)
        .cloned();
        
    #[cfg(debug_assertions)]
    if result.is_some() {
        eprintln!("LSP: Found function spec for '{}'", full_name);
    } else {
        eprintln!("LSP: No function spec found for '{}'", full_name);
        eprintln!("LSP: Available functions in {}: {:?}", namespace, 
            namespace_functions.iter().map(|f| &f.name).collect::<Vec<_>>());
    }
    
    result
}

/// Get specification for a specific action (e.g. "evm::deploy_contract")
pub fn get_action_spec(full_name: &str) -> Option<CommandSpecification> {
    let parts: Vec<&str> = full_name.split("::").collect();
    if parts.len() != 2 {
        return None;
    }
    
    let namespace = parts[0];
    let action_name = parts[1];
    
    let all_specs = get_action_specifications();
    let namespace_actions = all_specs.get(namespace)?;
    
    namespace_actions.iter()
        .find(|(matcher, _)| matcher == action_name)
        .map(|(_, spec)| spec.clone())
}