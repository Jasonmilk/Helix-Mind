//! Intent router — the single routing entity for internal message dispatch.
//!
//! The `IntentRouter` is a compile-time-determined `HashMap<String, Handler>`
//! lookup table. It is NOT an event bus, NOT a mailbox, and NOT a reflection
//! registry. It performs O(1) routing with zero dynamic dispatch overhead
//! beyond the trait object.
//!
//! This is the ONLY additional entity introduced beyond the raw mpsc channel.
//! It follows the "Orchestrate, Don't Build" axiom (#01) and the "Strict
//! Contracts" iron law (#07).

//! Intent router — the single routing entity for internal message dispatch.

use crate::envelope::{IntentEnvelope, IntentResponse};
use crate::error::MindError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait IntentHandler: Send + Sync {
    /// Process a validated intent envelope.
    async fn handle(&self, envelope: &IntentEnvelope) -> Result<IntentResponse, MindError>;

    /// Return the list of intent types this handler is responsible for.
    fn supported_intents(&self) -> Vec<&'static str>;
}

/// Compile-time-determined routing table.
pub struct IntentRouter {
    routes: HashMap<String, Arc<dyn IntentHandler>>,
}

impl IntentRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Arc<dyn IntentHandler>) {
        for intent_type in handler.supported_intents() {
            self.routes
                .insert(intent_type.to_string(), handler.clone());
        }
    }

    pub async fn route(&self, envelope: &IntentEnvelope) -> Result<IntentResponse, MindError> {
        match self.routes.get(&envelope.intent_type) {
            Some(handler) => handler.handle(envelope).await,
            None => Err(MindError::SchemaMismatch {
                detail: format!("unregistered intent type: {}", envelope.intent_type),
            }),
        }
    }

    pub fn len(&self) -> usize {
        self.routes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }
}

impl Default for IntentRouter {
    fn default() -> Self {
        Self::new()
    }
}