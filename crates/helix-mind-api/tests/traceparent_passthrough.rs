//! traceparent 透传集成测试（P3c，ADR-0020）。
//!
//! 验证：`HelixQueryRequest.traceparent` → `HelixQueryResult.traceparent`
//! 原样回传（透传不生成，Mind 不是 trace 根）；请求无 traceparent → 响应为空。

use helix_mind_api::proto::helix_mind_client::HelixMindClient;
use helix_mind_api::proto::helix_mind_server::HelixMindServer;
use helix_mind_api::proto::HelixQueryRequest;
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
    HelixMindServiceImpl::new(
        config,
        storage,
        retrieval,
        metabolism,
        federation,
        reincarnation,
        std::sync::Arc::new(helix_mind_cognitive::CognitiveCraft::new(
            std::sync::Arc::new(helix_mind_metabolism::DeterministicAdapter::new(helix_mind_core::config::MetabolismConfig::default())),
            helix_mind_cognitive::CraftConfig::default(),
        )),
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

fn make_request(traceparent: &str) -> HelixQueryRequest {
    HelixQueryRequest {
        query: "earth is round".into(),
        suggested_mode: 0, // Skilled
        energy_context: None,
        include_recessive: false,
        allow_imagination: false,
        autonomy_level: 0, // Agent
        traceparent: traceparent.into(),
    }
}

#[tokio::test]
async fn traceparent_echoed_back_verbatim() {
    let service = build_service().await;
    let addr = spawn_server(service).await;

    let mut client = HelixMindClient::connect(addr).await.unwrap();
    let resp = client
        .helix_query(make_request(TRACEPARENT))
        .await
        .unwrap()
        .into_inner();

    assert_eq!(resp.traceparent, TRACEPARENT, "traceparent must echo verbatim");
}

#[tokio::test]
async fn missing_traceparent_returns_empty() {
    let service = build_service().await;
    let addr = spawn_server(service).await;

    let mut client = HelixMindClient::connect(addr).await.unwrap();
    let resp = client
        .helix_query(make_request(""))
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        resp.traceparent, "",
        "Mind never generates a traceparent; empty in → empty out"
    );
}
