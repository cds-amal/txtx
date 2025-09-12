//! The main LSP backend implementation using tower-lsp.
//!
//! This module implements the Language Server Protocol for txtx, providing
//! completions, diagnostics, hover, and other IDE features.

use crate::document::DocumentStore;
use crate::symbols::{SymbolInfo, SymbolKind as TxtxSymbolKind, SymbolTable};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tracing::{debug, error, info};

/// The main LSP backend for txtx
pub struct TxtxLspBackend {
    /// LSP client for sending notifications and requests
    client: Client,
    /// Document store for managing open documents
    documents: Arc<RwLock<DocumentStore>>,
    /// Symbol table for tracking definitions and references
    symbols: Arc<SymbolTable>,
}

impl TxtxLspBackend {
    /// Creates a new LSP backend
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(DocumentStore::new())),
            symbols: Arc::new(SymbolTable::new()),
        }
    }

    /// Analyzes a document and updates the symbol table
    async fn analyze_document(&self, uri: &Url) -> Result<()> {
        let documents = self.documents.read().await;
        let document = documents
            .get(uri)
            .ok_or_else(|| anyhow::anyhow!("Document not found"))?;

        // Clear existing symbols for this file
        self.symbols.remove_file_symbols(uri);

        // TODO: Parse document and extract symbols
        // For now, we'll use a simple regex-based approach for testing
        self.extract_symbols_basic(uri, &document.text()).await?;

        Ok(())
    }

    /// Extract symbols using the txtx parser
    async fn extract_symbols_basic(&self, uri: &Url, text: &str) -> Result<()> {
        // Parse the document using txtx-parser
        match txtx_parser::parse(text) {
            Ok(runbook) => {
                // Extract actions
                for action in &runbook.actions {
                    // Find the line number for this action (simplified for now)
                    let line_num = self.find_line_for_symbol(text, "action", &action.name);
                    
                    self.symbols.add_symbol(SymbolInfo {
                        name: action.name.clone(),
                        kind: TxtxSymbolKind::Action,
                        definition_location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: line_num, character: 0 },
                                end: Position { line: line_num, character: 100 }, // Approximate
                            },
                        },
                        documentation: None,
                        action_type: Some(action.action_type.clone()),
                        signer_type: None,
                        can_rename: true,
                    });
                }

                // Extract variables
                for variable in &runbook.variables {
                    let line_num = self.find_line_for_symbol(text, "variable", &variable.name);
                    
                    self.symbols.add_symbol(SymbolInfo {
                        name: variable.name.clone(),
                        kind: TxtxSymbolKind::Variable,
                        definition_location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: line_num, character: 0 },
                                end: Position { line: line_num, character: 100 },
                            },
                        },
                        documentation: None,
                        action_type: None,
                        signer_type: None,
                        can_rename: true,
                    });
                }

                // Extract signers
                for signer in &runbook.signers {
                    let line_num = self.find_line_for_symbol(text, "signer", &signer.name);
                    
                    self.symbols.add_symbol(SymbolInfo {
                        name: signer.name.clone(),
                        kind: TxtxSymbolKind::Signer,
                        definition_location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: line_num, character: 0 },
                                end: Position { line: line_num, character: 100 },
                            },
                        },
                        documentation: None,
                        action_type: None,
                        signer_type: Some(signer.signer_type.clone()),
                        can_rename: true,
                    });
                }

                // Extract outputs
                for output in &runbook.outputs {
                    let line_num = self.find_line_for_symbol(text, "output", &output.name);
                    
                    self.symbols.add_symbol(SymbolInfo {
                        name: output.name.clone(),
                        kind: TxtxSymbolKind::Output,
                        definition_location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: line_num, character: 0 },
                                end: Position { line: line_num, character: 100 },
                            },
                        },
                        documentation: None,
                        action_type: None,
                        signer_type: None,
                        can_rename: true,
                    });
                }

                // Extract flows
                for flow in &runbook.flows {
                    let line_num = self.find_line_for_symbol(text, "flow", &flow.name);
                    
                    self.symbols.add_symbol(SymbolInfo {
                        name: flow.name.clone(),
                        kind: TxtxSymbolKind::Flow,
                        definition_location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: line_num, character: 0 },
                                end: Position { line: line_num, character: 100 },
                            },
                        },
                        documentation: None,
                        action_type: None,
                        signer_type: None,
                        can_rename: true,
                    });
                }

                Ok(())
            }
            Err(e) => {
                // Log the parse error but don't fail - we'll show diagnostics instead
                debug!("Failed to parse document: {}", e);
                Ok(())
            }
        }
    }

    /// Helper to find the line number for a symbol (simplified implementation)
    fn find_line_for_symbol(&self, text: &str, keyword: &str, name: &str) -> u32 {
        let pattern = format!(r#"{}\s+"{}""#, keyword, name);
        for (line_num, line) in text.lines().enumerate() {
            if line.contains(&pattern) || (line.contains(keyword) && line.contains(&format!(r#""{}""#, name))) {
                return line_num as u32;
            }
        }
        0
    }

    /// Publishes diagnostics for a document
    async fn publish_diagnostics(&self, uri: Url) -> Result<()> {
        let diagnostics = {
            let documents = self.documents.read().await;
            if let Some(doc) = documents.get(&uri) {
                let text = doc.text().to_string();
                drop(documents); // Release the lock
                self.validate_document(&text).await
            } else {
                vec![]
            }
        };

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;

        Ok(())
    }

    /// Validate a document and return diagnostics
    async fn validate_document(&self, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // First, try to parse the document
        match txtx_parser::parse(text) {
            Ok(_runbook) => {
                // Parsing succeeded - run semantic validation
                // TODO: Integrate full doctor validation here
                // For now, we'll add basic checks
            }
            Err(e) => {
                // Parsing failed - add a diagnostic
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 1 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("parse_error".to_string())),
                    source: Some("txtx".to_string()),
                    message: format!("Parse error: {}", e),
                    related_information: None,
                    tags: None,
                    code_description: None,
                    data: None,
                });
            }
        }

        diagnostics
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for TxtxLspBackend {
    async fn initialize(&self, _params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        info!("Initializing txtx LSP server");

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("LSP server initialized");
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        info!("Shutting down LSP server");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        debug!("Document opened: {}", params.text_document.uri);

        let mut documents = self.documents.write().await;
        documents.open(
            params.text_document.uri.clone(),
            params.text_document.version,
            params.text_document.text,
            params.text_document.language_id,
        );

        // Analyze and publish diagnostics
        drop(documents); // Release write lock
        if let Err(e) = self.analyze_document(&params.text_document.uri).await {
            error!("Failed to analyze document: {}", e);
        }
        if let Err(e) = self.publish_diagnostics(params.text_document.uri).await {
            error!("Failed to publish diagnostics: {}", e);
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("Document changed: {}", params.text_document.uri);

        let mut documents = self.documents.write().await;
        if let Err(e) = documents.update(
            &params.text_document.uri,
            params.text_document.version,
            params.content_changes,
        ) {
            error!("Failed to update document: {}", e);
            return;
        }

        // Re-analyze and publish diagnostics
        drop(documents); // Release write lock
        if let Err(e) = self.analyze_document(&params.text_document.uri).await {
            error!("Failed to analyze document: {}", e);
        }
        if let Err(e) = self.publish_diagnostics(params.text_document.uri.clone()).await {
            error!("Failed to publish diagnostics: {}", e);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        debug!("Document closed: {}", params.text_document.uri);

        let mut documents = self.documents.write().await;
        documents.close(&params.text_document.uri);

        // Clear symbols for this file
        self.symbols.remove_file_symbols(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> jsonrpc::Result<Option<CompletionResponse>> {
        debug!("Completion requested at {:?}", params.text_document_position);

        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Get the current line text to understand context
        let documents = self.documents.read().await;
        let line_text = if let Some(doc) = documents.get(uri) {
            doc.line_text(position.line as usize).unwrap_or_default()
        } else {
            String::new()
        };
        drop(documents);

        let mut items = Vec::new();

        // Check if we're typing an action type (after action name)
        if line_text.contains("action") && line_text.contains("\"") {
            // Suggest common action types
            items.extend(vec![
                CompletionItem {
                    label: "evm::deploy".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Deploy an EVM contract".to_string()),
                    ..Default::default()
                },
                CompletionItem {
                    label: "evm::call".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Call an EVM contract function".to_string()),
                    ..Default::default()
                },
                CompletionItem {
                    label: "stacks::call_contract".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Call a Stacks contract".to_string()),
                    ..Default::default()
                },
                CompletionItem {
                    label: "bitcoin::transfer".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Transfer Bitcoin".to_string()),
                    ..Default::default()
                },
            ]);
        }
        // Check if we're typing a signer type
        else if line_text.contains("signer") && line_text.contains("\"") {
            items.extend(vec![
                CompletionItem {
                    label: "evm::secret_key".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("EVM private key signer".to_string()),
                    ..Default::default()
                },
                CompletionItem {
                    label: "stacks::secret_key".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Stacks private key signer".to_string()),
                    ..Default::default()
                },
            ]);
        }
        // Check if we're in a reference context (e.g., after "action.")
        else if line_text.contains("action.") || line_text.contains("variable.") || line_text.contains("signer.") {
            // Get all defined symbols of the appropriate type
            let prefix = if line_text.contains("action.") {
                "action"
            } else if line_text.contains("variable.") {
                "variable"
            } else {
                "signer"
            };

            let symbols = self.symbols.all_symbols();
            for symbol in symbols {
                let matches = match prefix {
                    "action" => matches!(symbol.kind, TxtxSymbolKind::Action),
                    "variable" => matches!(symbol.kind, TxtxSymbolKind::Variable),
                    "signer" => matches!(symbol.kind, TxtxSymbolKind::Signer),
                    _ => false,
                };

                if matches {
                    items.push(CompletionItem {
                        label: symbol.name.clone(),
                        kind: Some(CompletionItemKind::REFERENCE),
                        detail: symbol.action_type.clone().or(symbol.signer_type.clone()),
                        ..Default::default()
                    });
                }
            }
        }
        // Default: suggest top-level keywords
        else {
            items.extend(vec![
                CompletionItem {
                    label: "action".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define an action".to_string()),
                    insert_text: Some("action \"$1\" \"$2\" {\n\t$0\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "variable".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define a variable".to_string()),
                    insert_text: Some("variable \"$1\" {\n\tvalue = $0\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "signer".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define a signer".to_string()),
                    insert_text: Some("signer \"$1\" \"$2\" {\n\t$0\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "output".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define an output".to_string()),
                    insert_text: Some("output \"$1\" {\n\tvalue = $0\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "flow".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Define a flow".to_string()),
                    insert_text: Some("flow \"$1\" {\n\t$0\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                CompletionItem {
                    label: "addon".to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Configure an addon".to_string()),
                    insert_text: Some("addon \"$1\" {\n\t$0\n}".to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
            ]);
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> jsonrpc::Result<Option<Hover>> {
        debug!("Hover requested at {:?}", params.text_document_position_params);

        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(symbol) = self.symbols.symbol_at_position(uri, position) {
            let content = format!(
                "**{}**: {}\n\nKind: {:?}",
                symbol.name,
                symbol.documentation.as_deref().unwrap_or("No documentation"),
                symbol.kind
            );

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: content,
                }),
                range: None,
            }));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        debug!("Go to definition requested at {:?}", params.text_document_position_params);

        // TODO: Find symbol under cursor and return its definition location
        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> jsonrpc::Result<Option<Vec<Location>>> {
        debug!("Find references requested at {:?}", params.text_document_position);

        // TODO: Find all references to the symbol under cursor
        Ok(None)
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> jsonrpc::Result<Option<PrepareRenameResponse>> {
        debug!("Prepare rename at {:?}", params);

        // TODO: Check if symbol can be renamed
        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> jsonrpc::Result<Option<WorkspaceEdit>> {
        debug!("Rename to '{}' at {:?}", params.new_name, params.text_document_position);

        // TODO: Perform rename operation
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<DocumentSymbolResponse>> {
        debug!("Document symbols requested for {}", params.text_document.uri);

        let symbols = self.symbols.symbols_in_file(&params.text_document.uri);

        let document_symbols: Vec<DocumentSymbol> = symbols
            .into_iter()
            .map(|sym| {
                #[allow(deprecated)]
                DocumentSymbol {
                    name: sym.name.clone(),
                    detail: sym.action_type.clone(),
                    kind: match sym.kind {
                        TxtxSymbolKind::Action => SymbolKind::FUNCTION,
                        TxtxSymbolKind::Variable => SymbolKind::VARIABLE,
                        TxtxSymbolKind::Signer => SymbolKind::KEY,
                        TxtxSymbolKind::Flow => SymbolKind::NAMESPACE,
                        TxtxSymbolKind::Module => SymbolKind::MODULE,
                        TxtxSymbolKind::Output => SymbolKind::PROPERTY,
                        _ => SymbolKind::VARIABLE,
                    },
                    range: sym.definition_location.range,
                    selection_range: sym.definition_location.range,
                    children: None,
                    tags: None,
                    deprecated: None,
                }
            })
            .collect();

        Ok(Some(DocumentSymbolResponse::Nested(document_symbols)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Document, DocumentStore};
    use crate::symbols::{SymbolInfo, SymbolKind as TxtxSymbolKind};

    #[tokio::test]
    async fn test_document_management() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let doc = Document::new(uri.clone(), 1, "test content".to_string(), "txtx".to_string());
        assert_eq!(doc.version, 1);
        assert_eq!(doc.text(), "test content");
        
        // Test document update via document store
        let mut store = DocumentStore::new();
        store.open(uri.clone(), 1, "test content".to_string(), "txtx".to_string());
        
        store.update(&uri, 2, vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "new content".to_string(),
        }]).unwrap();
        
        let doc = store.get(&uri).unwrap();
        assert_eq!(doc.version, 2);
        assert_eq!(doc.text(), "new content");
    }

    #[tokio::test]
    async fn test_symbol_extraction_from_text() {
        // Test the regex extraction logic directly
        let text = r#"
action "deploy" "evm::deploy" {
    value = 1000
}

variable "gas_price" {
    value = 30000000000
}
"#;
        
        let action_re = regex::Regex::new(r#"action\s+"([^"]+)"\s+"([^"]+)""#).unwrap();
        let variable_re = regex::Regex::new(r#"variable\s+"([^"]+)""#).unwrap();
        
        let mut symbols = Vec::new();
        
        for (_line_num, line) in text.lines().enumerate() {
            if let Some(captures) = action_re.captures(line) {
                let name = captures.get(1).unwrap().as_str();
                let action_type = captures.get(2).unwrap().as_str();
                assert_eq!(name, "deploy");
                assert_eq!(action_type, "evm::deploy");
                symbols.push(name);
            }
            
            if let Some(captures) = variable_re.captures(line) {
                let name = captures.get(1).unwrap().as_str();
                assert_eq!(name, "gas_price");
                symbols.push(name);
            }
        }
        
        assert_eq!(symbols.len(), 2);
    }

    #[tokio::test]
    async fn test_symbol_table() {
        let table = SymbolTable::new();
        let uri = Url::parse("file:///test.tx").unwrap();
        
        let symbol = SymbolInfo {
            name: "test_action".to_string(),
            kind: TxtxSymbolKind::Action,
            action_type: Some("evm::deploy".to_string()),
            signer_type: None,
            definition_location: Location {
                uri: uri.clone(),
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: 10 },
                },
            },
            documentation: None,
            can_rename: true,
        };
        
        table.add_symbol(symbol.clone());
        
        let symbols = table.symbols_in_file(&uri);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "test_action");
    }
}