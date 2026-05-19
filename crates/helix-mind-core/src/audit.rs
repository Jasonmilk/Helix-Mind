use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub actor: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    GeneLockReloaded,
    LifecyclePhaseChanged,
    FederationNodeReceived,
    FederationNodeMerged,
    DigitalCremationTriggered,
    HumanViewSynced,
    ReincarnationTriggered,
    PrivacyAccessGranted,
    EmergencyDuskTriggered,
}

impl AuditEntry {
    pub fn new(event_type: AuditEventType, actor: &str, details: &str) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            actor: actor.to_string(),
            details: details.to_string(),
        }
    }
}
