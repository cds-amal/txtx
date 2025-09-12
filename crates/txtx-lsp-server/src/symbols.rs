//! Symbol table and symbol resolution for txtx documents.
//!
//! This module provides symbol tracking, resolution, and cross-reference
//! capabilities for the LSP server.

use anyhow::Result;
use dashmap::DashMap;
use tower_lsp::lsp_types::{Location, Position, Range, Url};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a type of symbol in txtx
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    /// A flow block defining an execution context
    Flow,
    /// A module block with metadata
    Module,
    /// A variable definition
    Variable,
    /// An action that performs operations
    Action,
    /// A signer for authentication
    Signer,
    /// An output value
    Output,
    /// An addon configuration
    Addon,
    /// An input reference (external)
    Input,
    /// A runbook block (embedded)
    Runbook,
}

/// Information about a symbol definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    /// The symbol's name
    pub name: String,
    /// The kind of symbol
    pub kind: SymbolKind,
    /// The location where the symbol is defined
    pub definition_location: Location,
    /// Optional documentation
    pub documentation: Option<String>,
    /// For actions: the action type (e.g., "evm::deploy")
    pub action_type: Option<String>,
    /// For signers: the signer type (e.g., "evm::secret_key")
    pub signer_type: Option<String>,
    /// Whether this symbol can be renamed
    pub can_rename: bool,
}

/// A reference to a symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolReference {
    /// The location of the reference
    pub location: Location,
    /// The kind of reference (read, write, etc.)
    pub reference_kind: ReferenceKind,
}

/// The kind of reference to a symbol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceKind {
    /// Reading the symbol's value
    Read,
    /// Writing/modifying the symbol
    Write,
    /// Using the symbol in a declaration
    Declaration,
}

/// Symbol table for a workspace
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Maps symbol names to their information
    symbols: Arc<DashMap<String, SymbolInfo>>,
    /// Maps symbol names to all their references
    references: Arc<DashMap<String, Vec<SymbolReference>>>,
    /// Maps file URIs to symbols defined in that file
    file_symbols: Arc<DashMap<Url, Vec<String>>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    /// Creates a new empty symbol table
    pub fn new() -> Self {
        Self {
            symbols: Arc::new(DashMap::new()),
            references: Arc::new(DashMap::new()),
            file_symbols: Arc::new(DashMap::new()),
        }
    }

    /// Adds a symbol definition
    pub fn add_symbol(&self, symbol: SymbolInfo) {
        let name = symbol.name.clone();
        let uri = symbol.definition_location.uri.clone();
        
        // Add to symbols map
        self.symbols.insert(name.clone(), symbol);
        
        // Track which file this symbol is in
        self.file_symbols
            .entry(uri)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Adds a reference to a symbol
    pub fn add_reference(&self, symbol_name: &str, reference: SymbolReference) {
        self.references
            .entry(symbol_name.to_string())
            .or_insert_with(Vec::new)
            .push(reference);
    }

    /// Gets symbol information by name
    pub fn get_symbol(&self, name: &str) -> Option<SymbolInfo> {
        self.symbols.get(name).map(|entry| entry.clone())
    }

    /// Gets all references to a symbol
    pub fn get_references(&self, name: &str) -> Vec<SymbolReference> {
        self.references
            .get(name)
            .map(|entry| entry.clone())
            .unwrap_or_default()
    }

    /// Finds the symbol at a given position
    pub fn symbol_at_position(&self, uri: &Url, position: Position) -> Option<SymbolInfo> {
        // Check all symbols to see if any contain this position
        for entry in self.symbols.iter() {
            let symbol = entry.value();
            if symbol.definition_location.uri == *uri {
                if range_contains_position(&symbol.definition_location.range, &position) {
                    return Some(symbol.clone());
                }
            }
        }
        
        None
    }

    /// Finds all symbols in a file
    pub fn symbols_in_file(&self, uri: &Url) -> Vec<SymbolInfo> {
        self.file_symbols
            .get(uri)
            .map(|entry| {
                entry
                    .iter()
                    .filter_map(|name| self.get_symbol(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Removes all symbols from a file
    pub fn remove_file_symbols(&self, uri: &Url) {
        if let Some(entry) = self.file_symbols.remove(uri) {
            for symbol_name in entry.1 {
                self.symbols.remove(&symbol_name);
                self.references.remove(&symbol_name);
            }
        }
    }

    /// Get all symbols across all files
    pub fn all_symbols(&self) -> Vec<SymbolInfo> {
        self.symbols
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Checks if a symbol name already exists
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Validates if a name can be used for renaming
    pub fn validate_rename(&self, old_name: &str, new_name: &str) -> Result<()> {
        // Check if the symbol exists and can be renamed
        let symbol = self.get_symbol(old_name)
            .ok_or_else(|| anyhow::anyhow!("Symbol '{}' not found", old_name))?;
        
        if !symbol.can_rename {
            return Err(anyhow::anyhow!("Symbol '{}' cannot be renamed", old_name));
        }
        
        // Check if new name is valid
        if !is_valid_identifier(new_name) {
            return Err(anyhow::anyhow!("'{}' is not a valid identifier", new_name));
        }
        
        // Check for conflicts
        if old_name != new_name && self.contains(new_name) {
            return Err(anyhow::anyhow!("Symbol '{}' already exists", new_name));
        }
        
        Ok(())
    }

    /// Clears all symbols
    pub fn clear(&self) {
        self.symbols.clear();
        self.references.clear();
        self.file_symbols.clear();
    }
}

/// Checks if a range contains a position
fn range_contains_position(range: &Range, position: &Position) -> bool {
    (range.start.line < position.line || 
     (range.start.line == position.line && range.start.character <= position.character))
    &&
    (range.end.line > position.line ||
     (range.end.line == position.line && range.end.character >= position.character))
}

/// Validates if a string is a valid txtx identifier
fn is_valid_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    let mut chars = name.chars();
    
    // First character must be letter or underscore
    if let Some(first) = chars.next() {
        if !first.is_ascii_alphabetic() && first != '_' {
            return false;
        }
    }
    
    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_location(uri: &str, start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Location {
        Location {
            uri: Url::parse(uri).unwrap(),
            range: Range {
                start: Position { line: start_line, character: start_char },
                end: Position { line: end_line, character: end_char },
            },
        }
    }

    #[test]
    fn test_symbol_table_add_and_get() {
        let table = SymbolTable::new();
        
        let symbol = SymbolInfo {
            name: "deploy".to_string(),
            kind: SymbolKind::Action,
            definition_location: create_test_location("file:///test.tx", 5, 0, 5, 10),
            documentation: Some("Deploys a contract".to_string()),
            action_type: Some("evm::deploy".to_string()),
            signer_type: None,
            can_rename: true,
        };
        
        table.add_symbol(symbol.clone());
        
        let retrieved = table.get_symbol("deploy").unwrap();
        assert_eq!(retrieved.name, "deploy");
        assert_eq!(retrieved.kind, SymbolKind::Action);
        assert_eq!(retrieved.action_type, Some("evm::deploy".to_string()));
    }

    #[test]
    fn test_symbol_references() {
        let table = SymbolTable::new();
        
        let symbol = SymbolInfo {
            name: "my_var".to_string(),
            kind: SymbolKind::Variable,
            definition_location: create_test_location("file:///test.tx", 1, 0, 1, 10),
            documentation: None,
            action_type: None,
            signer_type: None,
            can_rename: true,
        };
        
        table.add_symbol(symbol);
        
        // Add some references
        table.add_reference("my_var", SymbolReference {
            location: create_test_location("file:///test.tx", 5, 10, 5, 16),
            reference_kind: ReferenceKind::Read,
        });
        
        table.add_reference("my_var", SymbolReference {
            location: create_test_location("file:///test.tx", 10, 5, 10, 11),
            reference_kind: ReferenceKind::Read,
        });
        
        let refs = table.get_references("my_var");
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].reference_kind, ReferenceKind::Read);
    }

    #[test]
    fn test_symbol_at_position() {
        let table = SymbolTable::new();
        let uri = Url::parse("file:///test.tx").unwrap();
        
        let symbol = SymbolInfo {
            name: "deploy".to_string(),
            kind: SymbolKind::Action,
            definition_location: Location {
                uri: uri.clone(),
                range: Range {
                    start: Position { line: 5, character: 10 },
                    end: Position { line: 5, character: 20 },
                },
            },
            documentation: None,
            action_type: Some("evm::deploy".to_string()),
            signer_type: None,
            can_rename: true,
        };
        
        table.add_symbol(symbol);
        
        // Position within the symbol
        let found = table.symbol_at_position(&uri, Position { line: 5, character: 15 });
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "deploy");
        
        // Position outside the symbol
        let not_found = table.symbol_at_position(&uri, Position { line: 6, character: 0 });
        assert!(not_found.is_none());
    }

    #[test]
    fn test_file_symbols() {
        let table = SymbolTable::new();
        let uri = Url::parse("file:///test.tx").unwrap();
        
        // Add multiple symbols to the same file
        table.add_symbol(SymbolInfo {
            name: "var1".to_string(),
            kind: SymbolKind::Variable,
            definition_location: Location { uri: uri.clone(), range: Default::default() },
            documentation: None,
            action_type: None,
            signer_type: None,
            can_rename: true,
        });
        
        table.add_symbol(SymbolInfo {
            name: "action1".to_string(),
            kind: SymbolKind::Action,
            definition_location: Location { uri: uri.clone(), range: Default::default() },
            documentation: None,
            action_type: Some("evm::deploy".to_string()),
            signer_type: None,
            can_rename: true,
        });
        
        let symbols = table.symbols_in_file(&uri);
        assert_eq!(symbols.len(), 2);
        
        // Remove file symbols
        table.remove_file_symbols(&uri);
        assert_eq!(table.symbols_in_file(&uri).len(), 0);
        assert!(table.get_symbol("var1").is_none());
        assert!(table.get_symbol("action1").is_none());
    }

    #[test]
    fn test_validate_rename() {
        let table = SymbolTable::new();
        
        // Add a renameable symbol
        table.add_symbol(SymbolInfo {
            name: "old_name".to_string(),
            kind: SymbolKind::Variable,
            definition_location: create_test_location("file:///test.tx", 1, 0, 1, 10),
            documentation: None,
            action_type: None,
            signer_type: None,
            can_rename: true,
        });
        
        // Add a non-renameable symbol
        table.add_symbol(SymbolInfo {
            name: "input_var".to_string(),
            kind: SymbolKind::Input,
            definition_location: create_test_location("file:///test.tx", 2, 0, 2, 10),
            documentation: None,
            action_type: None,
            signer_type: None,
            can_rename: false,
        });
        
        // Valid rename
        assert!(table.validate_rename("old_name", "new_name").is_ok());
        
        // Cannot rename non-renameable symbol
        assert!(table.validate_rename("input_var", "new_input").is_err());
        
        // Cannot use existing name
        assert!(table.validate_rename("old_name", "input_var").is_err());
        
        // Invalid identifier
        assert!(table.validate_rename("old_name", "123invalid").is_err());
        assert!(table.validate_rename("old_name", "invalid-name").is_err());
    }

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("valid_name"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("name123"));
        assert!(is_valid_identifier("CamelCase"));
        
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123start"));
        assert!(!is_valid_identifier("invalid-name"));
        assert!(!is_valid_identifier("invalid.name"));
        assert!(!is_valid_identifier("invalid name"));
    }

    #[test]
    fn test_range_contains_position() {
        let range = Range {
            start: Position { line: 5, character: 10 },
            end: Position { line: 7, character: 20 },
        };
        
        // Inside range
        assert!(range_contains_position(&range, &Position { line: 6, character: 0 }));
        assert!(range_contains_position(&range, &Position { line: 5, character: 10 })); // Start inclusive
        assert!(range_contains_position(&range, &Position { line: 7, character: 20 })); // End inclusive
        
        // Outside range
        assert!(!range_contains_position(&range, &Position { line: 4, character: 0 }));
        assert!(!range_contains_position(&range, &Position { line: 8, character: 0 }));
        assert!(!range_contains_position(&range, &Position { line: 5, character: 9 }));
    }
}