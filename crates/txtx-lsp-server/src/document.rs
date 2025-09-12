//! Document management for the LSP server.
//! 
//! This module handles storing and managing open documents, tracking their
//! versions, and providing efficient text manipulation operations.

use anyhow::Result;
use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};
use ropey::Rope;
use std::collections::HashMap;

/// Represents a single document in the workspace
#[derive(Debug, Clone)]
pub struct Document {
    /// The document's URI
    pub uri: Url,
    /// The document's version number
    pub version: i32,
    /// The document's content as a rope for efficient manipulation
    content: Rope,
    /// The document's language ID (should be "txtx")
    pub language_id: String,
}

impl Document {
    /// Creates a new document
    pub fn new(uri: Url, version: i32, text: String, language_id: String) -> Self {
        Self {
            uri,
            version,
            content: Rope::from_str(&text),
            language_id,
        }
    }

    /// Gets the full text content of the document
    pub fn text(&self) -> String {
        self.content.to_string()
    }

    /// Gets a line from the document (0-indexed)
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.content.len_lines() {
            Some(self.content.line(line_idx).to_string())
        } else {
            None
        }
    }

    /// Gets the total number of lines in the document
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get text of a specific line
    pub fn line_text(&self, line: usize) -> Option<String> {
        if line < self.content.len_lines() {
            Some(self.content.line(line).to_string())
        } else {
            None
        }
    }

    /// Converts an LSP position to a rope byte offset
    pub fn position_to_offset(&self, position: Position) -> Option<usize> {
        let line_idx = position.line as usize;
        if line_idx >= self.content.len_lines() {
            return None;
        }

        let line_start_offset = self.content.line_to_byte(line_idx);
        let line = self.content.line(line_idx);
        
        // Convert UTF-16 code units (LSP) to byte offset
        let mut utf16_col = 0;
        let mut byte_offset = 0;
        
        for ch in line.chars() {
            if utf16_col >= position.character as usize {
                break;
            }
            utf16_col += ch.len_utf16();
            byte_offset += ch.len_utf8();
        }
        
        Some(line_start_offset + byte_offset)
    }

    /// Converts a rope byte offset to an LSP position
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let line_idx = self.content.byte_to_line(offset.min(self.content.len_bytes()));
        let line_start_offset = self.content.line_to_byte(line_idx);
        let column_offset = offset - line_start_offset;
        
        // Convert byte offset to UTF-16 code units (LSP)
        let line = self.content.line(line_idx);
        let mut utf16_col = 0;
        let mut byte_count = 0;
        
        for ch in line.chars() {
            if byte_count >= column_offset {
                break;
            }
            byte_count += ch.len_utf8();
            utf16_col += ch.len_utf16();
        }
        
        Position {
            line: line_idx as u32,
            character: utf16_col as u32,
        }
    }

    /// Gets text within a range
    pub fn text_in_range(&self, range: Range) -> Option<String> {
        let start = self.position_to_offset(range.start)?;
        let end = self.position_to_offset(range.end)?;
        
        if start <= end && end <= self.content.len_bytes() {
            Some(self.content.byte_slice(start..end).to_string())
        } else {
            None
        }
    }

    /// Applies a change to the document
    pub fn apply_change(&mut self, change: &TextDocumentContentChangeEvent) {
        if let Some(range) = change.range {
            // Incremental change
            if let (Some(start), Some(end)) = (
                self.position_to_offset(range.start),
                self.position_to_offset(range.end),
            ) {
                self.content.remove(start..end);
                self.content.insert(start, &change.text);
            }
        } else {
            // Full document change
            self.content = Rope::from_str(&change.text);
        }
    }
}

/// Manages all open documents in the workspace
#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: HashMap<Url, Document>,
}

impl DocumentStore {
    /// Creates a new document store
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens a document
    pub fn open(&mut self, uri: Url, version: i32, text: String, language_id: String) {
        let document = Document::new(uri.clone(), version, text, language_id);
        self.documents.insert(uri, document);
    }

    /// Closes a document
    pub fn close(&mut self, uri: &Url) -> Option<Document> {
        self.documents.remove(uri)
    }

    /// Gets a document by URI
    pub fn get(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }

    /// Gets a mutable reference to a document by URI
    pub fn get_mut(&mut self, uri: &Url) -> Option<&mut Document> {
        self.documents.get_mut(uri)
    }

    /// Updates a document with changes
    pub fn update(&mut self, uri: &Url, version: i32, changes: Vec<TextDocumentContentChangeEvent>) -> Result<()> {
        let document = self.get_mut(uri)
            .ok_or_else(|| anyhow::anyhow!("Document not found: {}", uri))?;
        
        document.version = version;
        for change in changes {
            document.apply_change(&change);
        }
        
        Ok(())
    }

    /// Returns an iterator over all documents
    pub fn iter(&self) -> impl Iterator<Item = (&Url, &Document)> {
        self.documents.iter()
    }

    /// Returns the number of open documents
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Returns true if no documents are open
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let doc = Document::new(uri.clone(), 1, "hello\nworld".to_string(), "txtx".to_string());
        
        assert_eq!(doc.version, 1);
        assert_eq!(doc.language_id, "txtx");
        assert_eq!(doc.text(), "hello\nworld");
        assert_eq!(doc.line_count(), 2);
    }

    #[test]
    fn test_document_line_access() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let doc = Document::new(uri, 1, "line 1\nline 2\nline 3".to_string(), "txtx".to_string());
        
        assert_eq!(doc.line(0), Some("line 1\n".to_string()));
        assert_eq!(doc.line(1), Some("line 2\n".to_string()));
        assert_eq!(doc.line(2), Some("line 3".to_string()));
        assert_eq!(doc.line(3), None);
    }

    #[test]
    fn test_position_offset_conversion() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let doc = Document::new(uri, 1, "hello\nworld\n123".to_string(), "txtx".to_string());
        
        // Test position to offset
        assert_eq!(doc.position_to_offset(Position { line: 0, character: 0 }), Some(0));
        assert_eq!(doc.position_to_offset(Position { line: 0, character: 5 }), Some(5));
        assert_eq!(doc.position_to_offset(Position { line: 1, character: 0 }), Some(6));
        assert_eq!(doc.position_to_offset(Position { line: 1, character: 5 }), Some(11));
        
        // Test offset to position
        assert_eq!(doc.offset_to_position(0), Position { line: 0, character: 0 });
        assert_eq!(doc.offset_to_position(5), Position { line: 0, character: 5 });
        assert_eq!(doc.offset_to_position(6), Position { line: 1, character: 0 });
        assert_eq!(doc.offset_to_position(11), Position { line: 1, character: 5 });
    }

    #[test]
    fn test_text_in_range() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let doc = Document::new(uri, 1, "hello world\nfoo bar".to_string(), "txtx".to_string());
        
        let range = Range {
            start: Position { line: 0, character: 6 },
            end: Position { line: 1, character: 3 },
        };
        
        assert_eq!(doc.text_in_range(range), Some("world\nfoo".to_string()));
    }

    #[test]
    fn test_apply_incremental_change() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let mut doc = Document::new(uri, 1, "hello world".to_string(), "txtx".to_string());
        
        let change = TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position { line: 0, character: 6 },
                end: Position { line: 0, character: 11 },
            }),
            range_length: None,
            text: "rust".to_string(),
        };
        
        doc.apply_change(&change);
        assert_eq!(doc.text(), "hello rust");
    }

    #[test]
    fn test_apply_full_change() {
        let uri = Url::parse("file:///test.tx").unwrap();
        let mut doc = Document::new(uri, 1, "hello world".to_string(), "txtx".to_string());
        
        let change = TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "completely new text".to_string(),
        };
        
        doc.apply_change(&change);
        assert_eq!(doc.text(), "completely new text");
    }

    #[test]
    fn test_document_store_operations() {
        let mut store = DocumentStore::new();
        let uri1 = Url::parse("file:///test1.tx").unwrap();
        let uri2 = Url::parse("file:///test2.tx").unwrap();
        
        // Test opening documents
        store.open(uri1.clone(), 1, "doc1".to_string(), "txtx".to_string());
        store.open(uri2.clone(), 1, "doc2".to_string(), "txtx".to_string());
        
        assert_eq!(store.len(), 2);
        assert!(!store.is_empty());
        
        // Test getting documents
        assert!(store.get(&uri1).is_some());
        assert_eq!(store.get(&uri1).unwrap().text(), "doc1");
        
        // Test updating documents
        let changes = vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "updated doc1".to_string(),
        }];
        
        store.update(&uri1, 2, changes).unwrap();
        assert_eq!(store.get(&uri1).unwrap().version, 2);
        assert_eq!(store.get(&uri1).unwrap().text(), "updated doc1");
        
        // Test closing documents
        store.close(&uri1);
        assert_eq!(store.len(), 1);
        assert!(store.get(&uri1).is_none());
    }

    #[test]
    fn test_utf16_position_handling() {
        let uri = Url::parse("file:///test.tx").unwrap();
        // String with emoji: "hello 👋 world"
        let doc = Document::new(uri, 1, "hello 👋 world".to_string(), "txtx".to_string());
        
        // The emoji takes 2 UTF-16 code units
        // Position after emoji should account for this
        let pos_after_emoji = Position { line: 0, character: 8 }; // 6 (hello ) + 2 (emoji)
        let offset = doc.position_to_offset(pos_after_emoji).unwrap();
        
        // The byte offset should be at the space after the emoji
        let text_from_offset = &doc.text()[offset..];
        assert_eq!(text_from_offset, " world");
    }
}