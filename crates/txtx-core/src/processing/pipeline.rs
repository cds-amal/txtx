//! Pipeline and composition utilities for processors

use super::{RunbookProcessor, ProcessorError};
use crate::runbook::Runbook;
use error_stack::{Result, Report};
use std::marker::PhantomData;

/// A pipeline that runs two processors in sequence
pub struct Pipeline<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pipeline<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> RunbookProcessor for Pipeline<A, B>
where
    A: RunbookProcessor,
    B: RunbookProcessor,
    B::Context: From<A::Output>,
{
    type Output = B::Output;
    type Error = ProcessorError;
    type Context = A::Context;
    
    fn process(
        &self,
        runbook: &Runbook,
        context: Self::Context,
    ) -> Result<Self::Output, Self::Error> {
        let intermediate = self.first
            .process(runbook, context)
            .map_err(|e| Report::new(ProcessorError::Generic(format!("Pipeline first stage failed: {:?}", e))))?;
            
        let second_context = B::Context::from(intermediate);
        
        self.second
            .process(runbook, second_context)
            .map_err(|e| Report::new(ProcessorError::Generic(format!("Pipeline second stage failed: {:?}", e))))
    }
    
    fn name(&self) -> &str {
        "Pipeline"
    }
}

/// A processor that runs multiple processors in parallel and collects results
pub struct Parallel<P, O> {
    processors: Vec<P>,
    _output: PhantomData<O>,
}

impl<P, O> Parallel<P, O> {
    pub fn new(processors: Vec<P>) -> Self {
        Self {
            processors,
            _output: PhantomData,
        }
    }
}

/// A processor that conditionally runs one of two processors
pub struct Conditional<P1, P2, F> {
    condition: F,
    if_true: P1,
    if_false: P2,
}

impl<P1, P2, F> Conditional<P1, P2, F> {
    pub fn new(condition: F, if_true: P1, if_false: P2) -> Self {
        Self {
            condition,
            if_true,
            if_false,
        }
    }
}

impl<P1, P2, F, C> RunbookProcessor for Conditional<P1, P2, F>
where
    P1: RunbookProcessor<Context = C>,
    P2: RunbookProcessor<Context = C, Output = P1::Output, Error = P1::Error>,
    F: Fn(&Runbook, &C) -> bool + Send + Sync,
    C: Clone,
{
    type Output = P1::Output;
    type Error = P1::Error;
    type Context = C;
    
    fn process(
        &self,
        runbook: &Runbook,
        context: Self::Context,
    ) -> Result<Self::Output, Self::Error> {
        if (self.condition)(runbook, &context) {
            self.if_true.process(runbook, context)
        } else {
            self.if_false.process(runbook, context)
        }
    }
    
    fn name(&self) -> &str {
        "Conditional"
    }
}