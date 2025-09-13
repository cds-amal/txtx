//! Source location tracking for AST nodes

use serde::{Deserialize, Serialize};

/// Represents a position in the source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// Zero-based line number
    pub line: usize,
    /// Zero-based column number (in bytes)
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Represents a span in the source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Start position (inclusive)
    pub start: Position,
    /// End position (exclusive)
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a span from Tree-sitter positions
    pub fn from_tree_sitter(node: &tree_sitter::Node) -> Self {
        let start = node.start_position();
        let end = node.end_position();
        
        Self {
            start: Position::new(start.row, start.column),
            end: Position::new(end.row, end.column),
        }
    }
}

/// A wrapper that adds source location to any AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Located<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Located<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
    
    /// Create a located value from a Tree-sitter node
    pub fn from_node(value: T, node: &tree_sitter::Node) -> Self {
        Self::new(value, Span::from_tree_sitter(node))
    }
    
    /// Get the inner value
    pub fn inner(&self) -> &T {
        &self.value
    }
    
    /// Get the inner value mutably
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    /// Consume and return the inner value
    pub fn into_inner(self) -> T {
        self.value
    }
    
    /// Map the inner value while preserving location
    pub fn map<U, F>(self, f: F) -> Located<U>
    where
        F: FnOnce(T) -> U,
    {
        Located::new(f(self.value), self.span)
    }
}

/// Helper trait for types that have a source location
pub trait HasLocation {
    fn span(&self) -> Span;
    fn start_position(&self) -> Position {
        self.span().start
    }
    fn end_position(&self) -> Position {
        self.span().end
    }
}

impl<T> HasLocation for Located<T> {
    fn span(&self) -> Span {
        self.span
    }
}