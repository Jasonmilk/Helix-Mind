//! Intent envelope and response types for internal message routing.
//!
//! All inter-module communication within Helix-Mind uses `IntentEnvelope`
//! as the unified data envelope. The `intent_type` field determines the
//! routing target; the `payload` carries the operation-specific parameters.
//! This design follows the "Piped Composability" axiom (#04).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unified intent envelope for all internal message routing.
///
/// This is the single channel contract defined by the CommonIntents protocol
/// stack. Modules only depend on this envelope and the `IntentSender` trait—
/// they never know the concrete type of the receiver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentEnvelope {
    /// Globally unique trace id for end-to-end observability.
    pub trace_id: Uuid,
    /// Identifier of the sending instance (e.g. "anaphase_01").
    pub source_id: String,
    /// Optional identifier of the intended receiver. `None` means broadcast.
    pub target_id: Option<String>,
    /// Intent type name, e.g. "CIS_QueryRequest", "CIB_Corroborate".
    pub intent_type: String,
    /// Unified JSON payload. Structure depends on `intent_type`.
    pub payload: serde_json::Value,
}

impl IntentEnvelope {
    /// Validate required fields before routing.
    pub fn validate(&self) -> Result<(), crate::error::MindError> {
        if self.trace_id.is_nil() {
            return Err(crate::error::MindError::Validation(
                "trace_id must not be nil".into(),
            ));
        }
        if self.source_id.is_empty() {
            return Err(crate::error::MindError::Validation(
                "source_id must not be empty".into(),
            ));
        }
        if self.intent_type.is_empty() {
            return Err(crate::error::MindError::Validation(
                "intent_type must not be empty".into(),
            ));
        }
        Ok(())
    }
}

/// Unified response envelope returned by all intent handlers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResponse {
    /// Echoes the trace_id from the request.
    pub trace_id: Uuid,
    /// Whether the intent was processed successfully.
    pub success: bool,
    /// Handler-specific result data.
    pub data: serde_json::Value,
    /// Error description if `success` is false.
    pub error: Option<String>,
}

impl IntentResponse {
    pub fn ok(trace_id: Uuid, data: serde_json::Value) -> Self {
        Self {
            trace_id,
            success: true,
            data,
            error: None,
        }
    }

    pub fn err(trace_id: Uuid, error: impl Into<String>) -> Self {
        Self {
            trace_id,
            success: false,
            data: serde_json::Value::Null,
            error: Some(error.into()),
        }
    }
}
