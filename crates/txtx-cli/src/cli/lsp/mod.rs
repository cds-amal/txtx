mod diagnostics;
mod functions;
mod workspace;
mod handlers;
mod utils;
mod validation;

use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    CompletionOptions, OneOf,
};
use std::error::Error;

use self::handlers::Handlers;
use self::workspace::SharedWorkspaceState;

/// Run the Language Server Protocol server
pub fn run_lsp() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Use stderr for logging so it doesn't interfere with LSP protocol on stdout
    eprintln!("Starting txtx Language Server");

    // Create the connection over stdin/stdout
    let (connection, io_threads) = Connection::stdio();
    
    // Wait for the initialize request
    let init_result = connection.initialize_start();
    let (initialize_id, initialize_params) = match init_result {
        Ok(params) => params,
        Err(e) => {
            eprintln!("Failed to receive initialize request: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    let initialize_params: InitializeParams = serde_json::from_value(initialize_params)?;
    
    eprintln!("Initialize params: {:?}", initialize_params.root_uri);
    
    // Build server capabilities
    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL
        )),
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".to_string()]),
            ..Default::default()
        }),

        ..Default::default()
    };
    
    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(lsp_types::ServerInfo {
            name: "txtx-language-server".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    };
    
    // Complete initialization
    connection.initialize_finish(initialize_id, serde_json::to_value(initialize_result)?)?;
    
    eprintln!("LSP server initialized successfully");
    
    // Create shared workspace state and handlers
    let workspace = SharedWorkspaceState::new();
    let handlers = Handlers::new(workspace);
    
    // Main message loop
    for message in &connection.receiver {
        match message {
            Message::Request(req) => {
                eprintln!("Received request: {}", req.method);
                
                // Handle shutdown request
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                
                // Route the request to appropriate handler
                let response = handle_request(req, &handlers);
                if let Some(resp) = response {
                    connection.sender.send(Message::Response(resp))?;
                }
            }
            Message::Notification(not) => {
                eprintln!("Received notification: {}", not.method);
                handle_notification(not, &handlers, &connection)?;
            }
            Message::Response(_) => {
                // We don't send requests, so we shouldn't get responses
                eprintln!("Unexpected response received");
            }
        }
    }
    
    // Join the IO threads
    io_threads.join()?;
    
    eprintln!("LSP server shutting down");
    Ok(())
}

fn handle_request(
    req: Request,
    handlers: &Handlers,
) -> Option<Response> {
    match req.method.as_str() {
        "textDocument/definition" => {
            let params: lsp_types::GotoDefinitionParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse definition params: {}", e);
                    return Some(Response::new_err(
                        req.id,
                        lsp_server::ErrorCode::InvalidParams as i32,
                        "Invalid parameters".to_string(),
                    ));
                }
            };
            
            let result = handlers.definition.goto_definition(params);
            Some(Response::new_ok(req.id, result))
        }
        "textDocument/hover" => {
            let params: lsp_types::HoverParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse hover params: {}", e);
                    return Some(Response::new_err(
                        req.id,
                        lsp_server::ErrorCode::InvalidParams as i32,
                        "Invalid parameters".to_string(),
                    ));
                }
            };
            
            let result = handlers.hover.hover(params);
            Some(Response::new_ok(req.id, result))
        }
        "textDocument/completion" => {
            let params: lsp_types::CompletionParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse completion params: {}", e);
                    return Some(Response::new_err(
                        req.id,
                        lsp_server::ErrorCode::InvalidParams as i32,
                        "Invalid parameters".to_string(),
                    ));
                }
            };
            
            let result = handlers.completion.completion(params);
            Some(Response::new_ok(req.id, result))
        }
        _ => {
            eprintln!("Unhandled request: {}", req.method);
            Some(Response::new_err(
                req.id,
                lsp_server::ErrorCode::MethodNotFound as i32,
                format!("Method not found: {}", req.method),
            ))
        }
    }
}

fn handle_notification(
    not: lsp_server::Notification,
    handlers: &Handlers,
    connection: &Connection,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match not.method.as_str() {
        "textDocument/didOpen" => {
            let params: lsp_types::DidOpenTextDocumentParams = serde_json::from_value(not.params)?;
            let uri = params.text_document.uri.clone();
            handlers.document_sync.did_open(params);
            
            // Send diagnostics
            let diagnostics = handlers.diagnostics.get_diagnostics(&uri);
            if !diagnostics.is_empty() {
                let params = lsp_types::PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version: None,
                };
                let notification = lsp_server::Notification {
                    method: "textDocument/publishDiagnostics".to_string(),
                    params: serde_json::to_value(params)?,
                };
                connection.sender.send(Message::Notification(notification))?;
            }
        }
        "textDocument/didChange" => {
            let params: lsp_types::DidChangeTextDocumentParams = serde_json::from_value(not.params)?;
            let uri = params.text_document.uri.clone();
            handlers.document_sync.did_change(params);
            
            // Send diagnostics
            let diagnostics = handlers.diagnostics.get_diagnostics(&uri);
            if !diagnostics.is_empty() {
                let params = lsp_types::PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version: None,
                };
                let notification = lsp_server::Notification {
                    method: "textDocument/publishDiagnostics".to_string(),
                    params: serde_json::to_value(params)?,
                };
                connection.sender.send(Message::Notification(notification))?;
            }
        }
        "textDocument/didSave" => {
            let _params: lsp_types::DidSaveTextDocumentParams = serde_json::from_value(not.params)?;
            // Currently a no-op, but could trigger validation
        }
        "textDocument/didClose" => {
            let params: lsp_types::DidCloseTextDocumentParams = serde_json::from_value(not.params)?;
            handlers.document_sync.did_close(params);
        }
        _ => {
            eprintln!("Unhandled notification: {}", not.method);
        }
    }
    Ok(())
}