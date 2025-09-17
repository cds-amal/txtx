//! Hover information handler
//!
//! Provides hover information for functions, actions, and input references

use super::{Handler, TextDocumentHandler};
use crate::cli::lsp::{
    functions::{get_action_hover, get_function_hover, get_signer_hover},
    workspace::SharedWorkspaceState,
};
use lsp_types::{*, Url};

pub struct HoverHandler {
    workspace: SharedWorkspaceState,
}

impl HoverHandler {
    pub fn new(workspace: SharedWorkspaceState) -> Self {
        Self { workspace }
    }

    /// Extract environment from file path
    /// For "config.aws.prod.tx", returns Some("prod")
    /// For "main.tx", returns None
    fn extract_environment_from_uri(uri: &lsp_types::Url) -> Option<String> {
        if let Ok(path) = uri.to_file_path() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_str) = file_name.to_str() {
                    // Must end with .tx
                    if !file_str.ends_with(".tx") {
                        return None;
                    }

                    // Remove .tx extension
                    let without_ext = &file_str[..file_str.len() - 3];

                    // Split by dots
                    let parts: Vec<&str> = without_ext.split('.').collect();

                    // If there are at least 2 parts, the last one is the environment
                    if parts.len() >= 2 {
                        return Some(parts[parts.len() - 1].to_string());
                    }
                }
            }
        }
        None
    }

    /// Dump the current txtx state for debugging
    fn dump_txtx_state(
        &self,
        uri: &Url,
    ) -> Option<Hover> {
        let workspace = self.workspace.read();
        
        // Get the current environment from VS Code's workspace state (environment selector)
        // Falls back to file-based detection, then to "global" if neither is available
        let current_env = workspace.get_current_environment()
            .or_else(|| Self::extract_environment_from_uri(uri))
            .unwrap_or_else(|| "global".to_string());
        
        let mut debug_text = String::from("# 🔍 txtx State Dump\n\n");
        
        // Add current file info
        debug_text.push_str(&format!("**Current file**: `{}`\n", uri.path()));
        debug_text.push_str(&format!("**Selected environment**: `{}`\n", current_env));
        
        // Add environment detection info
        if let Some(file_env) = Self::extract_environment_from_uri(uri) {
            if file_env != current_env {
                debug_text.push_str(&format!("**File-based environment**: `{}` (overridden by selector)\n", file_env));
            }
        }
        debug_text.push_str("\n");
        
        // Get manifest info
        if let Some(manifest) = workspace.get_manifest_for_document(uri) {
            debug_text.push_str("## Manifest Information\n\n");
            debug_text.push_str(&format!("**Manifest URI**: `{}`\n\n", manifest.uri));
            
            // List all environments
            debug_text.push_str("## Environments\n\n");
            let mut env_names: Vec<_> = manifest.environments.keys().cloned().collect();
            env_names.sort();
            
            for env_name in &env_names {
                if let Some(env_vars) = manifest.environments.get(env_name) {
                    debug_text.push_str(&format!("### {} ({} variables)\n", env_name, env_vars.len()));
                    
                    // Sort variables by key
                    let mut vars: Vec<_> = env_vars.iter().collect();
                    vars.sort_by_key(|(k, _)| k.as_str());
                    
                    if vars.is_empty() {
                        debug_text.push_str("*(no variables)*\n");
                    } else {
                        // Show first few variables as a sample
                        debug_text.push_str("```yaml\n");
                        for (idx, (key, value)) in vars.iter().enumerate() {
                            if idx < 10 {
                                // Truncate long values for display
                                let display_value = if value.len() > 50 {
                                    format!("{}...", &value[..47])
                                } else {
                                    value.to_string()
                                };
                                debug_text.push_str(&format!("{}: \"{}\"\n", key, display_value));
                            } else if idx == 10 {
                                debug_text.push_str(&format!("# ... and {} more variables\n", vars.len() - 10));
                                break;
                            }
                        }
                        debug_text.push_str("```\n");
                    }
                    debug_text.push('\n');
                }
            }
            
            // Show effective inputs for current environment
            debug_text.push_str(&format!("## Effective Inputs for '{}'\n\n", current_env));
            debug_text.push_str("*Resolution order: CLI inputs > environment-specific > global*\n\n");
            
            let mut effective_inputs: std::collections::HashMap<String, (String, String)> = std::collections::HashMap::new();
            
            // First add global inputs (lowest precedence)
            if let Some(global_vars) = manifest.environments.get("global") {
                for (key, value) in global_vars {
                    effective_inputs.insert(key.clone(), (value.clone(), "global".to_string()));
                }
            }
            
            // Then override with environment-specific inputs (higher precedence)
            if current_env != "global" {
                if let Some(env_vars) = manifest.environments.get(&current_env) {
                    for (key, value) in env_vars {
                        effective_inputs.insert(key.clone(), (value.clone(), current_env.clone()));
                    }
                }
            }
            
            // Note: CLI inputs would have highest precedence but aren't available in LSP context
            
            // Sort and display effective inputs
            let mut effective_vars: Vec<_> = effective_inputs.iter().collect();
            effective_vars.sort_by_key(|(k, _)| k.as_str());
            
            debug_text.push_str(&format!("**Total resolved inputs**: {}\n\n", effective_vars.len()));
            
            if effective_vars.is_empty() {
                debug_text.push_str("*(no inputs available)*\n");
            } else {
                debug_text.push_str("```yaml\n");
                for (idx, (key, (value, source))) in effective_vars.iter().enumerate() {
                    if idx < 20 {
                        // Truncate long values for display
                        let display_value = if value.len() > 50 {
                            format!("{}...", &value[..47])
                        } else {
                            value.to_string()
                        };
                        
                        if source == &current_env {
                            debug_text.push_str(&format!("{}: \"{}\"  # from {}\n", key, display_value, source));
                        } else {
                            debug_text.push_str(&format!("{}: \"{}\"  # inherited from {}\n", key, display_value, source));
                        }
                    } else if idx == 20 {
                        debug_text.push_str(&format!("# ... and {} more inputs\n", effective_vars.len() - 20));
                        break;
                    }
                }
                debug_text.push_str("```\n");
            }
            
            // Show summary statistics
            debug_text.push_str("\n## Summary\n\n");
            let global_count = manifest.environments.get("global").map_or(0, |e| e.len());
            let env_count = if current_env != "global" {
                manifest.environments.get(&current_env).map_or(0, |e| e.len())
            } else {
                0
            };
            
            debug_text.push_str(&format!("- **Global inputs**: {}\n", global_count));
            if current_env != "global" {
                debug_text.push_str(&format!("- **{} inputs**: {} (overrides)\n", current_env, env_count));
            }
            debug_text.push_str(&format!("- **Total effective inputs**: {}\n", effective_vars.len()));
            
            // List all available environments
            debug_text.push_str(&format!("\n**Available environments**: {}\n", 
                env_names.join(", ")));
            
        } else {
            debug_text.push_str("## ⚠️ No manifest found\n\n");
            debug_text.push_str("Could not find a `txtx.yml` file in the workspace.\n");
        }
        
        // Add workspace info
        debug_text.push_str("\n## Workspace Information\n\n");
        debug_text.push_str(&format!("**VS Code environment selector**: {}\n", 
            workspace.get_current_environment().unwrap_or_else(|| "not set".to_string())));
        debug_text.push_str(&format!("**Documents loaded**: {}\n", 
            workspace.documents().len()));
        
        // Add debugging tips
        debug_text.push_str("\n---\n");
        debug_text.push_str("💡 **Tip**: Use `input.dump_txtx_state` in any `.tx` file to see this debug info.\n");
        debug_text.push_str("💡 **Tip**: Use the VS Code environment selector to switch environments.\n");
        
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: debug_text,
            }),
            range: None,
        })
    }

    /// Dump detailed information about a specific variable across all environments
    fn dump_variable_details(
        &self,
        uri: &Url,
        variable_name: &str,
    ) -> Option<Hover> {
        let workspace = self.workspace.read();

        // Get the current environment
        let current_env = workspace.get_current_environment()
            .or_else(|| Self::extract_environment_from_uri(uri))
            .unwrap_or_else(|| "global".to_string());

        let mut debug_text = format!("# 🔍 Variable Details: `{}`\n\n", variable_name);

        // Add current environment info
        debug_text.push_str(&format!("**Current environment**: `{}`\n\n", current_env));

        // Get manifest info
        if let Some(manifest) = workspace.get_manifest_for_document(uri) {
            // Collect all environments and their values for this variable
            let mut env_values: Vec<(String, Option<String>)> = Vec::new();
            let mut env_names: Vec<String> = manifest.environments.keys().cloned().collect();
            env_names.sort();

            for env_name in &env_names {
                if let Some(env_vars) = manifest.environments.get(env_name) {
                    let value = env_vars.get(variable_name).cloned();
                    env_values.push((env_name.clone(), value));
                }
            }

            // Show definition in each environment
            debug_text.push_str("## Variable Definitions by Environment\n\n");

            let global_value = manifest.environments.get("global")
                .and_then(|vars| vars.get(variable_name))
                .cloned();

            for (env_name, value) in &env_values {
                if let Some(val) = value {
                    debug_text.push_str(&format!("### `{}`\n", env_name));

                    // Show the actual value
                    let display_value = if val.len() > 100 {
                        format!("{}...", &val[..97])
                    } else {
                        val.clone()
                    };
                    debug_text.push_str(&format!("**Value**: `{}`\n", display_value));

                    // Indicate if it's an override
                    if env_name != "global" && global_value.is_some() && global_value.as_ref() != Some(val) {
                        debug_text.push_str("*⚡ Overrides global value*\n");
                    }

                    debug_text.push_str("\n");
                }
            }

            // Show environments that don't define this variable but inherit it
            debug_text.push_str("## Environment Resolution\n\n");

            for env_name in &env_names {
                debug_text.push_str(&format!("### `{}`", env_name));

                // Mark current environment
                if env_name == &current_env {
                    debug_text.push_str(" *(current)*");
                }
                debug_text.push_str("\n");

                // Check if defined locally
                let local_value = manifest.environments.get(env_name)
                    .and_then(|vars| vars.get(variable_name));

                if let Some(val) = local_value {
                    let display_value = if val.len() > 100 {
                        format!("{}...", &val[..97])
                    } else {
                        val.clone()
                    };
                    debug_text.push_str(&format!("- **Defined locally**: `{}`\n", display_value));
                } else if env_name != "global" {
                    // Check if inherited from global
                    if let Some(ref global_val) = global_value {
                        let display_value = if global_val.len() > 100 {
                            format!("{}...", &global_val[..97])
                        } else {
                            global_val.clone()
                        };
                        debug_text.push_str(&format!("- **Inherited from global**: `{}`\n", display_value));
                    } else {
                        debug_text.push_str("- **Not defined** (variable not available)\n");
                    }
                } else {
                    debug_text.push_str("- **Not defined** (variable not available)\n");
                }

                // Show the resolved value (what would actually be used)
                let resolved_value = if let Some(val) = local_value {
                    Some(val.clone())
                } else if env_name != "global" {
                    global_value.clone()
                } else {
                    None
                };

                if let Some(resolved) = resolved_value {
                    let display_value = if resolved.len() > 100 {
                        format!("{}...", &resolved[..97])
                    } else {
                        resolved
                    };
                    debug_text.push_str(&format!("- **Resolved value**: `{}`\n", display_value));
                }

                debug_text.push_str("\n");
            }

            // Summary
            debug_text.push_str("## Summary\n\n");

            let defined_count = env_values.iter().filter(|(_, v)| v.is_some()).count();
            let total_envs = env_names.len();

            debug_text.push_str(&format!("- **Variable name**: `{}`\n", variable_name));
            debug_text.push_str(&format!("- **Defined in**: {} of {} environments\n", defined_count, total_envs));

            if let Some(ref global_val) = global_value {
                debug_text.push_str(&format!("- **Global value**: `{}`\n",
                    if global_val.len() > 50 {
                        format!("{}...", &global_val[..47])
                    } else {
                        global_val.clone()
                    }
                ));

                // Count overrides
                let override_count = env_values.iter()
                    .filter(|(name, val)| name != "global" && val.is_some() && val.as_ref() != Some(global_val))
                    .count();

                if override_count > 0 {
                    debug_text.push_str(&format!("- **Overridden in**: {} environment(s)\n", override_count));
                }
            } else {
                debug_text.push_str("- **Global value**: *not defined*\n");
            }

            // Check current environment resolution
            let current_resolved = manifest.environments.get(&current_env)
                .and_then(|vars| vars.get(variable_name))
                .or_else(|| {
                    if current_env != "global" {
                        global_value.as_ref()
                    } else {
                        None
                    }
                });

            if let Some(resolved) = current_resolved {
                debug_text.push_str(&format!("\n**Resolved in current environment (`{}`)**: `{}`\n",
                    current_env,
                    if resolved.len() > 50 {
                        format!("{}...", &resolved[..47])
                    } else {
                        resolved.clone()
                    }
                ));
            } else {
                debug_text.push_str(&format!("\n⚠️ **Not available in current environment (`{}`)**\n", current_env));
            }

        } else {
            debug_text.push_str("## ⚠️ No manifest found\n\n");
            debug_text.push_str("Could not find a `txtx.yml` file in the workspace.\n");
        }

        // Add tip
        debug_text.push_str("\n---\n");
        debug_text.push_str(&format!("💡 **Tip**: Use `input.dump_txtx_var_<name>` to see details for any variable.\n"));

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: debug_text,
            }),
            range: None,
        })
    }

    /// Handle hover request
    pub fn hover(&self, params: HoverParams) -> Option<Hover> {
        let (uri, content, position) =
            self.get_document_at_position(&params.text_document_position_params)?;

        // Debug: Log the hover position
        eprintln!("[HOVER DEBUG] Position: line {}, char {}", position.line, position.character);

        // Try to extract function/action reference
        if let Some(reference) = extract_function_or_action(&content, &position) {
            eprintln!("[HOVER DEBUG] Extracted function/action reference: '{}'", reference);
            
            // Check if it's a function
            if let Some(hover_text) = get_function_hover(&reference) {
                eprintln!("[HOVER DEBUG] Resolved as function");
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_text,
                    }),
                    range: None,
                });
            }

            // Check if it's an action
            if let Some(hover_text) = get_action_hover(&reference) {
                eprintln!("[HOVER DEBUG] Resolved as action");
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_text,
                    }),
                    range: None,
                });
            }

            // Check if it's a signer - now environment-aware
            // First check for static signers from addons
            if let Some(hover_text) = get_signer_hover(&reference) {
                eprintln!("[HOVER DEBUG] Resolved as signer from addon");
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_text,
                    }),
                    range: None,
                });
            }
            
            // If not found in static signers, check environment-specific signers
            // Get the current environment
            let workspace = self.workspace.read();
            let current_env = workspace.get_current_environment()
                .or_else(|| Self::extract_environment_from_uri(&uri))
                .unwrap_or_else(|| "global".to_string());
            
            eprintln!("[HOVER DEBUG] Checking for signer '{}' in environment '{}'", reference, current_env);
            
            // TODO: Here we need to parse signers from the environment-specific .tx files
            // For now, provide a generic message for environment-specific signers
            if reference.contains("::") {
                // Split the reference to get namespace and signer name
                let parts: Vec<&str> = reference.split("::").collect();
                if parts.len() == 2 {
                    let namespace = parts[0];
                    let signer_name = parts[1];
                    
                    // Provide a generic hover text for environment-specific signers
                    let hover_text = format!(
                        "### Signer: `{}`\n\n\
                        **Namespace**: `{}`\n\
                        **Environment**: `{}`\n\n\
                        This signer may be defined in an environment-specific file.\n\n\
                        💡 **Tip**: Check `*.{}.tx` files for environment-specific signer definitions.",
                        signer_name, namespace, current_env, current_env
                    );
                    
                    eprintln!("[HOVER DEBUG] Providing generic hover for environment signer");
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: hover_text,
                        }),
                        range: None,
                    });
                }
            }
            
            eprintln!("[HOVER DEBUG] Reference '{}' not resolved as function/action/signer", reference);
        }

        // Try input reference hover
        if let Some(var_ref) = extract_input_reference(&content, &position) {
            eprintln!("[HOVER DEBUG] Extracted input reference: 'input.{}'", var_ref);
            
            let workspace = self.workspace.read();

            // Special debug command: input.dump_txtx_state
            if var_ref == "dump_txtx_state" {
                eprintln!("[HOVER DEBUG] Resolved as special debug command: dump_txtx_state");
                return self.dump_txtx_state(&uri);
            }

            // Special debug command: input.dump_txtx_var_<variableName>
            if var_ref.starts_with("dump_txtx_var_") {
                let variable_name = &var_ref["dump_txtx_var_".len()..];
                eprintln!("[HOVER DEBUG] Resolved as special debug command: dump_txtx_var_{}", variable_name);
                return self.dump_variable_details(&uri, variable_name);
            }

            // Get the current environment from VS Code's workspace state (environment selector)
            // Falls back to file-based detection, then to "global" if neither is available
            let current_env = workspace.get_current_environment()
                .or_else(|| Self::extract_environment_from_uri(&uri))
                .unwrap_or_else(|| "global".to_string());

            eprintln!("[HOVER DEBUG] Current environment: '{}'", current_env);
            eprintln!("[HOVER DEBUG] VS Code selector: {:?}", workspace.get_current_environment());
            if let Some(file_env) = Self::extract_environment_from_uri(&uri) {
                eprintln!("[HOVER DEBUG] File-based environment: '{}'", file_env);
            }

            // Get manifest for the document
            let manifest = workspace.get_manifest_for_document(&uri)?;

            // Check if the input exists in the current environment (considering inheritance)
            let mut current_value = None;
            let mut inherited_from_global = false;

            // First check the current environment
            if let Some(env_vars) = manifest.environments.get(&current_env) {
                if let Some(value) = env_vars.get(&var_ref) {
                    current_value = Some(value.clone());
                    eprintln!("[HOVER DEBUG] Found in current env '{}': '{}'", current_env, value);
                }
            }

            // If not found and we're not in global, check global environment
            if current_value.is_none() && current_env != "global" {
                if let Some(global_vars) = manifest.environments.get("global") {
                    if let Some(value) = global_vars.get(&var_ref) {
                        current_value = Some(value.clone());
                        inherited_from_global = true;
                        eprintln!("[HOVER DEBUG] Inherited from global: '{}'", value);
                    }
                }
            }

            // Find all environments where this input is defined
            let mut defined_in: Vec<(String, String)> = Vec::new();

            for (env_name, env_vars) in &manifest.environments {
                if let Some(value) = env_vars.get(&var_ref) {
                    defined_in.push((env_name.clone(), value.clone()));
                }
            }

            eprintln!("[HOVER DEBUG] Input '{}' defined in {} environment(s)", var_ref, defined_in.len());

            let mut hover_text = format!("**Input**: `{}`\n\n", var_ref);

            if let Some(value) = current_value {
                // Input is available in current environment
                hover_text.push_str(&format!("**Current value**: `{}`\n", value));
                hover_text.push_str(&format!("**Environment**: `{}`", current_env));

                if inherited_from_global {
                    hover_text.push_str(" *(inherited from global)*");
                }
                hover_text.push_str("\n\n");

                // Show other environments where it's defined
                if defined_in.len() > 1 {
                    hover_text.push_str("**Also defined in:**\n");
                    for (env_name, value) in &defined_in {
                        if env_name != &current_env
                            && !(inherited_from_global && env_name == "global")
                        {
                            hover_text.push_str(&format!("- `{}`: `{}`\n", env_name, value));
                        }
                    }
                }
            } else if !defined_in.is_empty() {
                // Input not available in current environment but exists elsewhere
                hover_text.push_str(&format!(
                    "⚠️ **Not available** in environment `{}`\n\n",
                    current_env
                ));
                hover_text.push_str("**Available in:**\n");
                for (env_name, value) in &defined_in {
                    hover_text.push_str(&format!("- `{}`: `{}`\n", env_name, value));
                }
                hover_text.push_str(&format!(
                    "\n💡 Switch to one of these environments or add this input to `{}`",
                    current_env
                ));
            } else {
                // Input not found anywhere
                hover_text.push_str("⚠️ **Not defined** in any environment\n\n");
                hover_text.push_str(
                    "Add this input to your `txtx.yml` file:\n```yaml\nenvironments:\n  ",
                );
                hover_text.push_str(&current_env);
                hover_text.push_str(&format!(":\n    {}: \"<value>\"\n```", var_ref));
            }

            eprintln!("[HOVER DEBUG] Returning hover text for input '{}'", var_ref);
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_text,
                }),
                range: None,
            });
        }

        eprintln!("[HOVER DEBUG] No hover information found at position");
        None
    }
}

impl Handler for HoverHandler {
    fn workspace(&self) -> &SharedWorkspaceState {
        &self.workspace
    }
}

impl TextDocumentHandler for HoverHandler {}

// Helper function to check if a position is within a comment
fn is_in_comment(content: &str, position: &Position) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    if let Some(line) = lines.get(position.line as usize) {
        // Check for line comments starting with //
        if let Some(comment_start) = line.find("//") {
            if position.character >= comment_start as u32 {
                return true;
            }
        }
        
        // Check for line comments starting with #
        if let Some(comment_start) = line.find('#') {
            // Make sure it's not inside a string
            // Simple heuristic: count quotes before the #
            let before_hash = &line[..comment_start];
            let quote_count = before_hash.chars().filter(|c| *c == '"').count();
            
            // If even number of quotes, we're likely not in a string
            if quote_count % 2 == 0 && position.character >= comment_start as u32 {
                return true;
            }
        }
        
        // TODO: Handle block comments /* */ if HCL supports them
    }
    false
}

fn extract_function_or_action(content: &str, position: &Position) -> Option<String> {
    // Skip if position is in a comment
    if is_in_comment(content, position) {
        return None;
    }
    
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Simple heuristic: look for namespace::name pattern
    let re = regex::Regex::new(r"\b(\w+)::([\w_]+)\b").ok()?;

    for capture in re.captures_iter(line) {
        let full_match = capture.get(0)?;
        let start = full_match.start() as u32;
        let end = full_match.end() as u32;

        if position.character >= start && position.character <= end {
            return Some(full_match.as_str().to_string());
        }
    }

    None
}

fn extract_input_reference(content: &str, position: &Position) -> Option<String> {
    // Skip if position is in a comment
    if is_in_comment(content, position) {
        return None;
    }
    
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;

    // Look for input.variable_name pattern
    let re = regex::Regex::new(r"input\.(\w+)").ok()?;

    for capture in re.captures_iter(line) {
        if let Some(var_match) = capture.get(1) {
            let full_match = capture.get(0)?;
            let start = full_match.start() as u32;
            let end = full_match.end() as u32;

            // Check if cursor position is within the match bounds
            if position.character >= start && position.character < end {
                return Some(var_match.as_str().to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_comment() {
        // Test regular code - not in comment
        let content = "value = std::encode_hex(data)";
        let position = Position { line: 0, character: 15 };
        assert_eq!(is_in_comment(content, &position), false);

        // Test // comment
        let content = "// This is a comment";
        let position = Position { line: 0, character: 10 };
        assert_eq!(is_in_comment(content, &position), true);

        // Test # comment
        let content = "# This is a comment";
        let position = Position { line: 0, character: 10 };
        assert_eq!(is_in_comment(content, &position), true);

        // Test code before comment
        let content = "value = 5 // comment";
        let position = Position { line: 0, character: 5 };
        assert_eq!(is_in_comment(content, &position), false);

        // Test position in comment after code
        let content = "value = 5 // comment";
        let position = Position { line: 0, character: 15 };
        assert_eq!(is_in_comment(content, &position), true);
    }

    #[test]
    fn test_extract_function_reference() {
        let content = "value = std::encode_hex(data)";
        let position = Position { line: 0, character: 15 };

        // Debug: check if incorrectly detected as comment
        assert_eq!(is_in_comment(content, &position), false, "Should not be detected as comment");

        let result = extract_function_or_action(content, &position);
        assert_eq!(result, Some("std::encode_hex".to_string()));
    }

    #[test]
    fn test_extract_action_reference() {
        let content = "action \"deploy\" \"evm::deploy_contract\" {";
        let position = Position { line: 0, character: 20 };

        // Debug: check if incorrectly detected as comment
        assert_eq!(is_in_comment(content, &position), false, "Should not be detected as comment");

        let result = extract_function_or_action(content, &position);
        assert_eq!(result, Some("evm::deploy_contract".to_string()));
    }

    #[test]
    fn test_extract_input_reference() {
        let content = "value = input.api_key";
        let position = Position { line: 0, character: 15 };

        let result = extract_input_reference(content, &position);
        assert_eq!(result, Some("api_key".to_string()));
    }

    #[test]
    fn test_extract_input_dump_txtx_state() {
        let content = "debug = input.dump_txtx_state";
        
        // The string "input.dump_txtx_state" starts at position 8
        // Test hovering at 'i' of input (position 8)
        let position = Position { line: 0, character: 8 };
        let result = extract_input_reference(content, &position);
        assert_eq!(result, Some("dump_txtx_state".to_string()));
        
        // Test hovering at 'd' of dump (position 14)
        let position = Position { line: 0, character: 14 };
        let result = extract_input_reference(content, &position);
        assert_eq!(result, Some("dump_txtx_state".to_string()));
        
        // Test hovering in middle of "dump_txtx_state" (position 20)
        let position = Position { line: 0, character: 20 };
        let result = extract_input_reference(content, &position);
        assert_eq!(result, Some("dump_txtx_state".to_string()));
        
        // Test hovering at last character 'e' (position 28)
        let position = Position { line: 0, character: 28 };
        let result = extract_input_reference(content, &position);
        assert_eq!(result, Some("dump_txtx_state".to_string()));
        
        // Test hovering just after the match should return None
        let position = Position { line: 0, character: 29 };
        let result = extract_input_reference(content, &position);
        assert_eq!(result, None);
    }
}
