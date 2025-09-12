//! txtx Language Server Protocol implementation.
//!
//! This crate provides a Language Server Protocol (LSP) server for the txtx
//! runbook language, offering IDE features like completions, diagnostics,
//! hover information, and refactoring support.

pub mod backend_sync;

// Re-export main types
pub use backend_sync::TxtxLspBackend;
