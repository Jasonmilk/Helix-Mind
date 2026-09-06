//! helix_craft 集成测试（P10a，ADR-0031）。
//!
//! 验证：Anaphase 触发认知工艺编排 → 返回 synthesis + 确定性 trace_id
//! （`craft#{job_id}`，同 job 同输入字节级一致，非 UUID）；未知
//! process/mode 被拒绝（fail-closed）；traceparent 透传回显。

use helix_mind_api::proto::helix_mind_client::HelixMindClient;
use helix_mind_api::proto::helix_mind_server::HelixMindServer;
use helix_mind_api::proto::{HelixCraftRequest, ProcessStep};
use helix_mind_api::HelixMindServiceImpl;
use helix_mind_core::config::Config;
use helix_mind_federation::FederationEngine;
use helix_mind_metabolism::MetabolismEngine;
use helix_mind_reincarnation::ReincarnationEngine;
use helix_mind_retrieval::RetrievalEngine;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

const TRACEPARENT: &str = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";

async fn build_service() -> HelixMindServiceImpl {
    let mut config = Config::default();
    config.storage.sqlite_path = ":memory:".to_string();
    let storage = StorageEngine::new(&config.storage).await.unwrap();
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
    let cognitive = Arc::new(helix_mind_cognitive::CognitiveCraft::new(
        Arc::new(helix_mind_metabolism::DeterministicAdapter::new(
            helix_mind_core::config::MetabolismConfig::default(),
        )),
        helix_mind_cognitive::CraftConfig::default(),
    ));
    HelixMindServiceImpl::new(
        config,
        storage,
        retrieval,
        metabolism,
        federation,
        reincarnation,
        cognitive,
    )
}

/// 绑定随机端口启动 server，返回完整 endpoint 地址。
async fn spawn_server(service: HelixMindServiceImpl) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let incoming = TcpListenerStream::new(listener);
        Server::builder()
            .add_service(HelixMindServer::new(service))
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });
    format!("http://{}", addr)
}

fn craft_request(job_id: &str, traceparent: &str) -> HelixCraftRequest {
    HelixCraftRequest {
        query: "评估这个方案的架构风险".into(),
        steps: vec![
            ProcessStep {
                process: "structural".into(),
                mode: "skilled".into(),
            },
            ProcessStep {
                process: "critical".into(),
                mode: "anchored".into(),
            },
        ],
        global_constraints: "只做确定性分析，不臆造".into(),
        job_id: job_id.into(),
        energy_context: None,
        autonomy_level: 0, // Agent
        traceparent: traceparent.into(),
    }
}

#[tokio::test]
async fn craft_returns_deterministic_trace_and_synthesis() {
    let service = build_service().await;
    let addr = spawn_server(service).await;

    let mut client = HelixMindClient::connect(addr).await.unwrap();
    let resp = client
        .helix_craft(craft_request("job-42", TRACEPARENT))
        .await
        .unwrap()
        .into_inner();

    // Deterministic trace_id from job id, never a UUID.
    assert_eq!(resp.trace_id, "craft#job-42", "trace_id must be craft#{{job_id}}");
    // Orchestration runs: two steps + a Hegelian synthesis.
    assert_eq!(resp.steps.len(), 2, "two requested processes execute");
    assert!(!resp.synthesis.is_empty(), "convergence synthesis produced");
    // P3c: traceparent echoes verbatim (pass-through, never generated).
    assert_eq!(resp.traceparent, TRACEPARENT, "traceparent must echo verbatim");
    // P10b populates value_grade later; honest empty for now.
    assert!(resp.value_grade.is_empty(), "value_grade empty until P10b");
}

#[tokio::test]
async fn craft_is_deterministic_across_calls() {
    let service = build_service().await;
    let addr = spawn_server(service).await;

    let mut client = HelixMindClient::connect(addr).await.unwrap();
    let a = client
        .helix_craft(craft_request("job-7", ""))
        .await
        .unwrap()
        .into_inner();
    let b = client
        .helix_craft(craft_request("job-7", ""))
        .await
        .unwrap()
        .into_inner();

    assert_eq!(a.trace_id, b.trace_id, "same job → same trace_id");
    assert_eq!(a.synthesis, b.synthesis, "same input → byte-identical synthesis");
}

#[tokio::test]
async fn craft_rejects_unknown_process_fail_closed() {
    let service = build_service().await;
    let addr = spawn_server(service).await;

    let mut req = craft_request("job-x", "");
    req.steps[0].process = "telepathy".into();

    let mut client = HelixMindClient::connect(addr).await.unwrap();
    let err = client.helix_craft(req).await.unwrap_err();
    assert!(
        err.message().contains("Unknown process"),
        "fail-closed on unknown process, got: {}",
        err.message()
    );
}
