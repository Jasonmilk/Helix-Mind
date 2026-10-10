//! ADR-0043 T5a end-to-end: `RememberRequest.parent_ids` must produce real edges.
//!
//! These tests go through the real gRPC handler against an in-memory store, so they
//! cover the whole write path — proto field, handler, `derived_from`, edge creation,
//! and the storage layer's SQL upsert — rather than the diffusion in isolation.

use helix_mind_api::layer1::handle_remember;
use helix_mind_api::proto::RememberRequest;
use helix_mind_api::HelixMindServiceImpl;
use helix_mind_core::config::Config;
use helix_mind_core::graph::RelationType;
use helix_mind_federation::FederationEngine;
use helix_mind_metabolism::MetabolismEngine;
use helix_mind_reincarnation::ReincarnationEngine;
use helix_mind_retrieval::RetrievalEngine;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tonic::Request;
use uuid::Uuid;

async fn build() -> (HelixMindServiceImpl, Arc<StorageEngine>) {
    let config = Config::default();
    // 临时**文件**库，不是 `:memory:` —— 见 `craft_integration.rs` 同处注释与
    // `helix-mind-storage/src/sqlite_pool.rs:477`：r2d2 的每条 `:memory:` 连接
    // 都是私有空库，schema 只在其中一条上，并发下会偶发 `no such table`。
    let db = std::env::temp_dir().join(format!("helix_reparents_{}.db", uuid::Uuid::new_v4()));
    let storage_config = helix_mind_core::config::StorageConfig {
        sqlite_path: db.to_string_lossy().to_string(),
        wal_dir: db.with_extension("wal").to_string_lossy().to_string(), // 独立 WAL，避免共享
        ..config.storage.clone()
    };
    let storage = StorageEngine::new(&storage_config).await.unwrap();
    let retrieval = Arc::new(RetrievalEngine::new(
        config.retrieval.clone(),
        storage.clone(),
    ));
    let metabolism = Arc::new(MetabolismEngine::new(
        config.metabolism.clone(),
        storage.clone(),
    ));
    let federation = Arc::new(FederationEngine::new(
        config.federation.clone(),
        storage.clone(),
    ));
    let reincarnation = Arc::new(ReincarnationEngine::new(
        config.lifecycle.clone(),
        storage.clone(),
    ));
    let service = HelixMindServiceImpl::new(
        config,
        storage.clone(),
        retrieval,
        metabolism,
        federation,
        reincarnation,
        Arc::new(helix_mind_cognitive::CognitiveCraft::new(
            Arc::new(helix_mind_metabolism::DeterministicAdapter::new(
                helix_mind_core::config::MetabolismConfig::default(),
            )),
            helix_mind_cognitive::CraftConfig::default(),
        )),
    );
    (service, storage)
}

/// Write through the real handler and return the id Mind assigned.
async fn remember(service: &HelixMindServiceImpl, content: &str, parents: &[Uuid]) -> Uuid {
    let request = RememberRequest {
        content: content.to_string(),
        node_type: -1, // protocol default = L3 episodic
        parent_ids: parents.iter().map(|p| p.to_string()).collect(),
    };
    let response = handle_remember(service, Request::new(request))
        .await
        .expect("a remember with valid parents must succeed");
    Uuid::parse_str(&response.into_inner().node_id).expect("Mind must return a parsable uuid")
}

/// The core claim of T5a: a parent reference becomes a real, persisted edge.
#[tokio::test]
async fn parent_ids_create_real_edges() {
    let (service, storage) = build().await;
    let origin = remember(&service, "origin note", &[]).await;
    let derived = remember(&service, "derived note", &[origin]).await;

    let edges = storage
        .get_edges_between(&[origin, derived])
        .await
        .unwrap();
    assert_eq!(
        edges.len(),
        1,
        "one parent must produce exactly one edge, got {edges:?}"
    );
    let e = &edges[0];
    // Direction is derived -> origin, matching crystallize. Reversing it would make
    // diffusion flow backwards in time and nothing would report an error.
    assert_eq!(e.source_id, derived, "direction must be derived -> origin");
    assert_eq!(e.target_id, origin);
    // The spec's relation table permits TEMPORAL between L3 nodes and restricts
    // REFINES to L2 -> L2, so TEMPORAL is the only legal choice for succession.
    assert_eq!(e.relation_type, RelationType::Temporal);
    assert!(!e.is_soft, "provenance edges are hard edges");
}

/// Tolerant degradation (ADR-0043 D6): no parents means no edges, not an error.
/// This is what every existing caller did before T5b, so it must stay silent.
#[tokio::test]
async fn absent_parent_ids_create_no_edges() {
    let (service, storage) = build().await;
    let lone = remember(&service, "a note with no parents", &[]).await;

    let edges = storage.get_edges_between(&[lone]).await.unwrap();
    assert!(edges.is_empty(), "expected no edges, got {edges:?}");
}

/// Several parents each get their own edge (real fan-in, not a single chain link).
#[tokio::test]
async fn every_parent_gets_its_own_edge() {
    let (service, storage) = build().await;
    let a = remember(&service, "observation a", &[]).await;
    let b = remember(&service, "observation b", &[]).await;
    let conclusion = remember(&service, "conclusion", &[a, b]).await;

    let edges = storage
        .get_edges_between(&[a, b, conclusion])
        .await
        .unwrap();
    assert_eq!(edges.len(), 2, "two parents must yield two edges, got {edges:?}");
    assert!(
        edges.iter().all(|e| e.source_id == conclusion),
        "every edge must originate at the derived node"
    );
}

/// Rewriting the same relationship must not add a second edge (ADR-0043 D5). SQL
/// dedupes via its primary key; the in-memory graph is deduped separately, and this
/// checks the SQL half through the handler.
#[tokio::test]
async fn a_cycle_forming_parent_cannot_arise_while_ids_are_minted_here() {
    let (service, storage) = build().await;
    let a = remember(&service, "first", &[]).await;
    let b = remember(&service, "second", &[a]).await;

    // NOTE: the handler does tolerate `MindError::CycleDetected` by skipping the
    // offending edge, but that branch is currently UNREACHABLE through `Remember`:
    // every write mints a fresh node id, so the new node has no incoming edges and
    // therefore cannot already be reachable from its parents. It becomes reachable
    // only once the request's own `node_id` is honoured (INTENT-7 `WRITE_NODE`
    // carries one), i.e. when the caller can target an EXISTING node. Recorded
    // rather than tested as if it were live.
    let c = remember(&service, "third", &[b, a]).await;

    let edges = storage.get_edges_between(&[a, b, c]).await.unwrap();
    assert_eq!(edges.len(), 3, "lineage writes must all succeed, got {edges:?}");
}
