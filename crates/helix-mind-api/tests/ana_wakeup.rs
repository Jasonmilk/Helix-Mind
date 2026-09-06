//! ana_wakeup tests — P10d (ADR-0032): wake-up channel + peak-congestion guard.
//!
//! Deterministic, no heartbeat: alarms are L2 nodes (provenance
//! `alarm#{job_id}`); due judgement is relative to the service clock
//! (tests build due_at relative to now — same pattern as sleep_review).

use std::collections::HashMap;
use std::sync::Arc;

use chrono::{Duration, Utc};
use helix_mind_api::alarm::{ack_alarm, list_due_alarms};
use helix_mind_core::config::Config;
use helix_mind_core::graph::{Node, NodeContent, NodeType};
use helix_mind_storage::{StorageEngine, WritePriority};

fn alarm_node(job_id: &str, due_at: chrono::DateTime<Utc>, mode: &str, repeat_minutes: i64) -> Node {
    let mut n = Node::default();
    n.node_type = NodeType::L2;
    n.content = NodeContent::Structured(HashMap::from_iter(vec![
        ("due_at".to_string(), due_at.to_rfc3339()),
        ("mode".to_string(), mode.to_string()),
        ("action".to_string(), "hibernate".to_string()),
        ("repeat_minutes".to_string(), repeat_minutes.to_string()),
    ]));
    n.abstract_provenance = Some(format!("alarm#{job_id}"));
    n.notes = Some("pending".to_string());
    n
}

async fn make_storage() -> Arc<StorageEngine> {
    let mut config = Config::default();
    config.storage.sqlite_path = ":memory:".to_string();
    StorageEngine::new(&config.storage).await.unwrap()
}

#[tokio::test]
async fn punctual_alarm_due_at_or_after_now() {
    let storage = make_storage().await;
    let now = Utc::now();
    // Due in the past → claimed; due in the future → not yet.
    storage.write_node(alarm_node("past", now - Duration::minutes(5), "punctual", 0), WritePriority::Deferred).await.unwrap();
    storage.write_node(alarm_node("future", now + Duration::minutes(5), "punctual", 0), WritePriority::Deferred).await.unwrap();

    let due = list_due_alarms(&storage, 0).await.unwrap();
    assert_eq!(due.len(), 1, "only the past alarm is due");
    assert_eq!(due[0].job_id, "past");
    assert_eq!(due[0].mode, "punctual");
    assert_eq!(due[0].claim_id, "claim#past");
}

#[tokio::test]
async fn jittered_alarm_elastic_window() {
    let storage = make_storage().await;
    let now = Utc::now();
    // due_at = now + 30min; jitter window 60min → window opened (due - 60 <= now).
    storage.write_node(alarm_node("elastic", now + Duration::minutes(30), "jittered", 0), WritePriority::Deferred).await.unwrap();
    // Outside the window: due_at = now + 90min > now + 60min → not due.
    storage.write_node(alarm_node("far", now + Duration::minutes(90), "jittered", 0), WritePriority::Deferred).await.unwrap();

    let due = list_due_alarms(&storage, 60).await.unwrap();
    assert_eq!(due.len(), 1, "only the in-window alarm is due");
    assert_eq!(due[0].job_id, "elastic");
    assert_eq!(due[0].mode, "jittered");
}

#[tokio::test]
async fn jitter_off_means_punctual_for_all() {
    let storage = make_storage().await;
    let now = Utc::now();
    // jittered alarm 30min in the future with jitter_minutes = 0 → not due.
    storage.write_node(alarm_node("future", now + Duration::minutes(30), "jittered", 0), WritePriority::Deferred).await.unwrap();

    let due = list_due_alarms(&storage, 0).await.unwrap();
    assert!(due.is_empty(), "jitter off → strict due_at");
}

#[tokio::test]
async fn claiming_is_atomic_no_double_delivery() {
    let storage = make_storage().await;
    storage.write_node(alarm_node("only", Utc::now() - Duration::minutes(1), "punctual", 0), WritePriority::Deferred).await.unwrap();

    let first = list_due_alarms(&storage, 0).await.unwrap();
    let second = list_due_alarms(&storage, 0).await.unwrap();
    assert_eq!(first.len(), 1);
    assert!(second.is_empty(), "claimed alarm is not delivered twice");
}

#[tokio::test]
async fn ack_done_closes_and_renew_rolls_forward() {
    let storage = make_storage().await;
    let now = Utc::now();
    storage.write_node(alarm_node("once", now - Duration::minutes(1), "punctual", 0), WritePriority::Deferred).await.unwrap();
    storage.write_node(alarm_node("daily", now - Duration::minutes(1), "punctual", 1440), WritePriority::Deferred).await.unwrap();

    let due = list_due_alarms(&storage, 0).await.unwrap();
    assert_eq!(due.len(), 2);

    // done → closed forever.
    assert!(ack_alarm(&storage, "claim#once", "done").await.unwrap());
    // renewed → rolls from ORIGINAL due (+1440min), back to pending.
    assert!(ack_alarm(&storage, "claim#daily", "renewed").await.unwrap());

    // once: gone. daily: pending but due ~24h later → not due now.
    let due_again = list_due_alarms(&storage, 0).await.unwrap();
    assert!(due_again.is_empty(), "done closed, renewed not yet due");

    // Idempotent ack: done again → still success, no side effect.
    assert!(ack_alarm(&storage, "claim#once", "done").await.unwrap());
    // Unknown claim → false.
    assert!(!ack_alarm(&storage, "claim#nope", "done").await.unwrap());
}

#[tokio::test]
async fn late_alarm_is_still_delivered() {
    let storage = make_storage().await;
    // jittered alarm whose window fully passed — honest, still returned.
    storage.write_node(alarm_node("late", Utc::now() - Duration::hours(3), "jittered", 0), WritePriority::Deferred).await.unwrap();

    let due = list_due_alarms(&storage, 60).await.unwrap();
    assert_eq!(due.len(), 1, "late alarms are delivered, never dropped");
}
