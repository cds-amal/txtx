//! Unified processing framework for runbooks
//! 
//! This module provides a generic trait-based system for processing runbooks.
//! All operations on runbooks (validation, execution, planning, analysis) are
//! implemented as processors.

use crate::runbook::Runbook;
use error_stack::{Result, Report};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::manifest::WorkspaceManifest;
use indexmap::IndexMap;

pub mod context;
pub mod pipeline;
pub mod validators;

pub use context::{ProcessingContext, ValidationContext};
pub use pipeline::Pipeline;

/// Core trait for all runbook processors
/// 
/// A processor takes a parsed runbook and produces some output.
/// This could be validation results, execution results, a plan, etc.
pub trait RunbookProcessor: Send + Sync {
    /// The type of output this processor produces
    type Output: Send;
    
    /// The type of error this processor can produce
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// The type of context this processor requires
    type Context: Send;
    
    /// Process a runbook with the given context
    fn process(
        &self,
        runbook: &Runbook,
        context: Self::Context,
    ) -> Result<Self::Output, Self::Error>;
    
    /// Get a human-readable name for this processor
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

/// Factory trait for creating processors
pub trait ProcessorFactory: Send + Sync {
    /// The type of processor this factory creates
    type Processor: RunbookProcessor;
    
    /// Create a new processor instance
    fn create(&self) -> Result<Self::Processor, Report<std::io::Error>>;
}

/// A processor that can be boxed for dynamic dispatch
pub trait DynProcessor: Send + Sync {
    /// Process a runbook and return a serializable result
    fn process_dyn(
        &self,
        runbook: &Runbook,
        context: &ProcessingContext,
    ) -> Result<serde_json::Value, Report<ProcessorError>>;
    
    /// Get the processor name
    fn name(&self) -> &str;
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("Processor error: {0}")]
    Generic(String),
    
    #[error("Validation failed")]
    ValidationFailed,
    
    #[error("Execution failed")]
    ExecutionFailed,
    
    #[error("Context error: {0}")]
    ContextError(String),
}

/// Result type for validation processors
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
}

/// Diagnostic information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: Option<Location>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            success: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, message: String) {
        self.success = false;
        self.errors.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message,
            location: None,
            suggestion: None,
        });
    }
    
    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message,
            location: None,
            suggestion: None,
        });
    }
}