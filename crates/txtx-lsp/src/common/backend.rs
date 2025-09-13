use crate::lsp_types::MessageType;
use crate::state::{build_state, EditorState, WorkspaceState};
use crate::utils::get_runbook_location;
use lsp_types::{
    CompletionItem, CompletionParams, DocumentSymbol, DocumentSymbolParams, GotoDefinitionParams,
    Hover, HoverParams, InitializeParams, InitializeResult, Location, SignatureHelp,
    SignatureHelpParams,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use txtx_addon_kit::helpers::fs::{FileAccessor, FileLocation};
use txtx_addon_kit::types::diagnostics::{Diagnostic as TxtxDiagnostic, DiagnosticLevel};

use super::requests::capabilities::{get_capabilities, InitializationOptions};

#[derive(Debug, Clone)]
pub enum EditorStateInput {
    Owned(EditorState),
    RwLock(Arc<RwLock<EditorState>>),
}

impl EditorStateInput {
    pub fn try_read<F, R>(&self, closure: F) -> Result<R, String>
    where
        F: FnOnce(&EditorState) -> R,
    {
        match self {
            EditorStateInput::Owned(editor_state) => Ok(closure(editor_state)),
            EditorStateInput::RwLock(editor_state_lock) => match editor_state_lock.try_read() {
                Ok(editor_state) => Ok(closure(&editor_state)),
                Err(_) => Err("failed to read editor_state".to_string()),
            },
        }
    }

    pub fn try_write<F, R>(&mut self, closure: F) -> Result<R, String>
    where
        F: FnOnce(&mut EditorState) -> R,
    {
        match self {
            EditorStateInput::Owned(editor_state) => Ok(closure(editor_state)),
            EditorStateInput::RwLock(editor_state_lock) => match editor_state_lock.try_write() {
                Ok(mut editor_state) => Ok(closure(&mut editor_state)),
                Err(_) => Err("failed to write editor_state".to_string()),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LspNotification {
    ManifestOpened(FileLocation),
    ManifestSaved(FileLocation),
    RunbookOpened(FileLocation),
    RunbookSaved(FileLocation),
    RunbookChanged(FileLocation, String),
    RunbookClosed(FileLocation),
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LspNotificationResponse {
    pub aggregated_diagnostics: Vec<(FileLocation, Vec<TxtxDiagnostic>)>,
    pub notification: Option<(MessageType, String)>,
}

impl LspNotificationResponse {
    pub fn error(message: &str) -> LspNotificationResponse {
        LspNotificationResponse {
            aggregated_diagnostics: vec![],
            notification: Some((MessageType::ERROR, format!("Internal error: {}", message))),
        }
    }
}

pub async fn process_notification(
    command: LspNotification,
    editor_state: &mut EditorStateInput,
    file_accessor: Option<&dyn FileAccessor>,
) -> Result<LspNotificationResponse, String> {
    match command {
        LspNotification::ManifestOpened(manifest_location) => {
            // Only build the initial protocal state if it does not exist
            if editor_state.try_read(|es| es.workspaces.contains_key(&manifest_location))? {
                return Ok(LspNotificationResponse::default());
            }

            // With this manifest_location, let's initialize our state.
            let mut protocol_state = WorkspaceState::new();
            match build_state(&manifest_location, &mut protocol_state, file_accessor).await {
                Ok(_) => {
                    editor_state
                        .try_write(|es| es.index_workspace(manifest_location, protocol_state))?;
                    let (aggregated_diagnostics, notification) =
                        editor_state.try_read(|es| es.get_aggregated_diagnostics())?;
                    Ok(LspNotificationResponse { aggregated_diagnostics, notification })
                }
                Err(e) => Ok(LspNotificationResponse::error(&e)),
            }
        }
        LspNotification::ManifestSaved(manifest_location) => {
            // We will rebuild the entire state, without to try any optimizations for now
            let mut workspace_state = WorkspaceState::new();
            match build_state(&manifest_location, &mut workspace_state, file_accessor).await {
                Ok(_) => {
                    editor_state
                        .try_write(|es| es.index_workspace(manifest_location, workspace_state))?;
                    let (aggregated_diagnostics, notification) =
                        editor_state.try_read(|es| es.get_aggregated_diagnostics())?;
                    Ok(LspNotificationResponse { aggregated_diagnostics, notification })
                }
                Err(e) => Ok(LspNotificationResponse::error(&e)),
            }
        }
        LspNotification::RunbookOpened(runbook_location) => {
            // Find manifest by searching upward (stops at .git directory)
            let manifest_location = if file_accessor.is_none() {
                // Use our upward search when no file accessor (local filesystem)
                use crate::utils::manifest_finder::find_manifest_for_tx_file;
                match find_manifest_for_tx_file(&runbook_location) {
                    Some(location) => location,
                    None => {
                        // Return error diagnostic that will be shown to the user
                        let error_msg = format!(
                            "No txtx.yml found for {}. Searched up to project root (.git directory).",
                            runbook_location.get_file_name().unwrap_or_default()
                        );
                        
                        // Create a diagnostic error for the file
                        let diagnostic = TxtxDiagnostic::error_from_string(error_msg.clone());
                        
                        let diagnostics = vec![(runbook_location, vec![diagnostic])];
                        return Ok(LspNotificationResponse {
                            aggregated_diagnostics: diagnostics,
                            notification: Some((MessageType::ERROR, error_msg)),
                        });
                    }
                }
            } else {
                // Use existing method when file accessor is available
                runbook_location.get_workspace_manifest_location(file_accessor).await?
            };

            // store the contract in the active_contracts map
            if !editor_state.try_read(|es| es.active_runbooks.contains_key(&runbook_location))? {
                let contract_source = match file_accessor {
                    None => runbook_location.read_content_as_utf8(),
                    Some(file_accessor) => {
                        file_accessor.read_file(runbook_location.to_string()).await
                    }
                }?;

                // let metadata = editor_state.try_read(|es| {
                //     es.runbooks_lookup
                //         .get(&runbook_location)
                // })?;

                // if the contract isn't in lookup yet, fallback on manifest, to be improved in #668
                // let metadata = match metadata {
                //     Some(metadata) => metadata,
                //     None => {
                //         match file_accessor {
                //             None => WorkspaceManifest::from_location(&manifest_location),
                //             Some(file_accessor) => {
                //                 WorkspaceManifest::from_file_accessor(
                //                     &manifest_location,
                //                     file_accessor,
                //                 )
                //                 .await
                //             }
                //         }?
                //         .get_runbook_metadata_from_location(&runbook_location)
                //         .ok_or(format!(
                //             "No txtx.yml is associated to the runbook {}",
                //             &runbook_location.get_file_name().unwrap_or_default()
                //         ))?
                //     }
                // };

                editor_state.try_write(|es| {
                    es.insert_active_runbook(runbook_location.clone(), contract_source.as_str())
                })?;
            }

            // Only build the initial protocal state if it does not exist
            if editor_state.try_read(|es| es.workspaces.contains_key(&manifest_location))? {
                return Ok(LspNotificationResponse::default());
            }

            let mut protocol_state = WorkspaceState::new();
            match build_state(&manifest_location, &mut protocol_state, file_accessor).await {
                Ok(_) => {
                    editor_state
                        .try_write(|es| es.index_workspace(manifest_location, protocol_state))?;
                    let (aggregated_diagnostics, notification) =
                        editor_state.try_read(|es| es.get_aggregated_diagnostics())?;
                    Ok(LspNotificationResponse { aggregated_diagnostics, notification })
                }
                Err(e) => Ok(LspNotificationResponse::error(&e)),
            }
        }
        LspNotification::RunbookSaved(runbook_location) => {
            
            let manifest_location = match editor_state
                .try_write(|es| es.clear_workspace_associated_with_runbook(&runbook_location))?
            {
                Some(manifest_location) => manifest_location,
                None => {
                    // Try our upward search for manifest
                    use crate::utils::manifest_finder::find_manifest_for_tx_file;
                    match find_manifest_for_tx_file(&runbook_location) {
                        Some(location) => location,
                        None => runbook_location.get_workspace_manifest_location(file_accessor).await?,
                    }
                }
            };

            // TODO(): introduce partial analysis #604
            let mut workspace_state = WorkspaceState::new();
            match build_state(&manifest_location, &mut workspace_state, file_accessor).await {
                Ok(_) => {
                    editor_state.try_write(|es| {
                        es.index_workspace(manifest_location, workspace_state);
                        if let Some(_contract) = es.active_runbooks.get_mut(&runbook_location) {
                            // contract.update_definitions();
                        };
                    })?;

                    let (aggregated_diagnostics, notification) =
                        editor_state.try_read(|es| es.get_aggregated_diagnostics())?;
                    Ok(LspNotificationResponse { aggregated_diagnostics, notification })
                }
                Err(e) => Ok(LspNotificationResponse::error(&e)),
            }
        }
        LspNotification::RunbookChanged(runbook_location, contract_source) => {
            match editor_state.try_write(|es| {
                es.update_active_contract(&runbook_location, &contract_source, false)
            })? {
                Ok(_result) => Ok(LspNotificationResponse::default()),
                Err(err) => Ok(LspNotificationResponse::error(&err)),
            }
        }
        LspNotification::RunbookClosed(runbook_location) => {
            editor_state.try_write(|es| es.active_runbooks.remove_entry(&runbook_location))?;
            Ok(LspNotificationResponse::default())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LspRequest {
    Completion(CompletionParams),
    SignatureHelp(SignatureHelpParams),
    Definition(GotoDefinitionParams),
    Hover(HoverParams),
    DocumentSymbol(DocumentSymbolParams),
    Initialize(InitializeParams),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum LspRequestResponse {
    CompletionItems(Vec<CompletionItem>),
    SignatureHelp(Option<SignatureHelp>),
    Definition(Option<Location>),
    DocumentSymbol(Vec<DocumentSymbol>),
    Hover(Option<Hover>),
    Initialize(InitializeResult),
}

pub fn process_request(
    command: LspRequest,
    editor_state: &EditorStateInput,
) -> Result<LspRequestResponse, String> {
    match command {
        LspRequest::Completion(params) => {
            let file_url = params.text_document_position.text_document.uri;
            let position = params.text_document_position.position;

            let runbook_location = match get_runbook_location(&file_url) {
                Some(runbook_location) => runbook_location,
                None => return Ok(LspRequestResponse::CompletionItems(vec![])),
            };

            let completion_items = match editor_state
                .try_read(|es| es.get_completion_items_for_runbook(&runbook_location, &position))
            {
                Ok(result) => result,
                Err(_) => return Ok(LspRequestResponse::CompletionItems(vec![])),
            };

            Ok(LspRequestResponse::CompletionItems(completion_items))
        }

        LspRequest::Definition(params) => {
            let file_url = params.text_document_position_params.text_document.uri;
            let runbook_location = match get_runbook_location(&file_url) {
                Some(runbook_location) => runbook_location,
                None => return Ok(LspRequestResponse::Definition(None)),
            };
            let position = params.text_document_position_params.position;
            let location = editor_state
                .try_read(|es| es.get_definition_location(&runbook_location, &position))
                .unwrap_or_default();
            Ok(LspRequestResponse::Definition(location))
        }

        LspRequest::SignatureHelp(params) => {
            let file_url = params.text_document_position_params.text_document.uri;
            let runbook_location = match get_runbook_location(&file_url) {
                Some(runbook_location) => runbook_location,
                None => return Ok(LspRequestResponse::SignatureHelp(None)),
            };
            let position = params.text_document_position_params.position;

            // if the developer selects a specific signature
            // it can be retrieved in the context and kept selected
            let active_signature = params
                .context
                .and_then(|c| c.active_signature_help)
                .and_then(|s| s.active_signature);

            let signature = editor_state
                .try_read(|es| {
                    es.get_signature_help(&runbook_location, &position, active_signature)
                })
                .unwrap_or_default();
            Ok(LspRequestResponse::SignatureHelp(signature))
        }

        LspRequest::DocumentSymbol(params) => {
            let file_url = params.text_document.uri;
            let runbook_location = match get_runbook_location(&file_url) {
                Some(runbook_location) => runbook_location,
                None => return Ok(LspRequestResponse::DocumentSymbol(vec![])),
            };
            let document_symbols = editor_state
                .try_read(|es| es.get_document_symbols_for_runbook(&runbook_location))
                .unwrap_or_default();
            Ok(LspRequestResponse::DocumentSymbol(document_symbols))
        }

        LspRequest::Hover(params) => {
            let file_url = params.text_document_position_params.text_document.uri;
            
            #[cfg(debug_assertions)]
            eprintln!("LSP: Hover request for URL: {}", file_url);
            
            let runbook_location = match get_runbook_location(&file_url) {
                Some(runbook_location) => runbook_location,
                None => {
                    #[cfg(debug_assertions)]
                    eprintln!("LSP: Not a runbook file (no .tx extension?)");
                    return Ok(LspRequestResponse::Hover(None));
                }
            };
            
            #[cfg(debug_assertions)]
            eprintln!("LSP: Runbook location: {:?}", runbook_location);
            
            let position = params.text_document_position_params.position;
            
            #[cfg(debug_assertions)]
            eprintln!("LSP: Hover position: {:?}", position);
            
            let hover_data = editor_state
                .try_read(|es| es.get_hover_data(&runbook_location, &position))
                .unwrap_or_else(|e| {
                    #[cfg(debug_assertions)]
                    eprintln!("LSP: Error getting hover data: {}", e);
                    None
                });
                
            #[cfg(debug_assertions)]
            eprintln!("LSP: Hover response: {}", if hover_data.is_some() { "Some" } else { "None" });
            
            Ok(LspRequestResponse::Hover(hover_data))
        }
        _ => Err(format!("Unexpected command: {:?}", &command)),
    }
}

// lsp requests are not supposed to mut the editor_state (only the notifications do)
// this is to ensure there is no concurrency between notifications and requests to
// acquire write lock on the editor state in a wasm context
// except for the Initialize request, which is the first interaction between the client and the server
// and can therefore safely acquire write lock on the editor state
pub fn process_mutating_request(
    command: LspRequest,
    editor_state: &mut EditorStateInput,
) -> Result<LspRequestResponse, String> {
    match command {
        LspRequest::Initialize(params) => {
            let initialization_options = params
                .initialization_options
                .and_then(|o| serde_json::from_str(o.as_str()?).ok())
                .unwrap_or(InitializationOptions::default());

            // Store the settings
            editor_state.try_write(|es| es.settings = initialization_options.clone())?;
            
            // If we have a root URI, scan for txtx.yml files
            if let Some(root_uri) = params.root_uri {
                if let Some(root_location) = FileLocation::try_parse(&root_uri.to_string(), None) {
                    // Look for txtx.yml in the root
                    let mut manifest_location = root_location.clone();
                    if manifest_location.append_path("txtx.yml").is_ok() {
                        // Try to read to check if it exists
                        if manifest_location.read_content().is_ok() {
                            // Found a manifest, initialize the workspace
                            // Note: We can't do async operations here, so we'll just mark it for later
                            editor_state.try_write(|es| {
                                es.pending_workspace_roots.push(manifest_location);
                            })?;
                        }
                    }
                }
            }
            
            Ok(LspRequestResponse::Initialize(InitializeResult {
                server_info: None,
                capabilities: get_capabilities(&initialization_options),
            }))
        }
        _ => Err(format!("Unexpected command: {:?}, should not not mutate state", &command)),
    }
}
