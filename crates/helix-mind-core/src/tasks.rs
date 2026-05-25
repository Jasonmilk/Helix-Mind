//! Task DAG and scratchpad node types.
//!
//! Helix-Mind models tasks and personal notes as DAG nodes so that they
//! benefit from the same on-demand loading, spiral topology, and energy-aware
//! traversal as all other graph assets. This avoids the "notebook bloat"
//! problem where a monolithic scratchpad wastes context tokens.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── TaskNode ────────────────────────────────────────────────────────────

/// A task or project tracked in the personal task DAG.
///
/// Tasks form a strict DAG with `DependsOn` hard edges and `SpiralRefines`
/// edges for epistemological refinement (e.g. "I now understand this task
/// differently than when I created it").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    /// Unique task identifier.
    pub task_id: Uuid,
    /// Granularity of the task.
    pub task_type: TaskType,
    /// Current lifecycle status.
    pub status: TaskStatus,
    /// Priority 1 (lowest) to 10 (highest).
    pub priority: u8,
    /// Estimated number of cognitive cycles needed.
    pub estimated_effort: u64,
    /// Actual cognitive cycles consumed so far.
    pub actual_effort: u64,
    /// Condensed context at task creation time (max 5 L3 memories).
    pub context_snapshot: TaskContextSnapshot,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Project,
    Subtask,
    Milestone,
    Action,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Planning,
    InProgress,
    Blocked,
    Completed,
    Abandoned,
}

/// Lightweight snapshot of the cognitive context when a task was created.
///
/// This avoids loading the full L3 history every time the task is inspected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContextSnapshot {
    /// CIDs of related L2 concept nodes.
    pub related_l2_concepts: Vec<String>,
    /// Up to 5 L3 memory UUIDs that capture the task's origin.
    pub key_l3_memories: Vec<Uuid>,
    /// Related scratchpad note UUIDs.
    pub scratchpad_notes: Vec<Uuid>,
}

// ── Scratchpad (Notepad) Nodes ──────────────────────────────────────────

/// A condensed factual note (≤200 tokens).
///
/// Notes are DAG citizens: they reference L2 knowledge and derive from L3
/// conversations via edges. This enables on-demand retrieval instead of
/// loading an entire monolithic notebook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteNode {
    pub node_id: Uuid,
    /// The condensed fact, kept under 200 tokens.
    pub content: String,
}

/// A contextual reminder that fires when its trigger condition is met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderNode {
    pub node_id: Uuid,
    /// Natural-language trigger, e.g. "next time the user mentions project X".
    pub trigger_condition: String,
}

/// A self-generated agenda item in Helix's internal to-do list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaNode {
    pub node_id: Uuid,
    pub priority: u8,
    pub action: String,
    pub created_by: AgendaCreator,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgendaCreator {
    User,
    HelixSelf,
}
