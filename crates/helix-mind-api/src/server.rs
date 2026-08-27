use helix_mind_core::config::{ApiConfig, Transport};
use helix_mind_core::config::Config;
use helix_mind_storage::StorageEngine;
use helix_mind_retrieval::RetrievalEngine;
use helix_mind_metabolism::MetabolismEngine;
use helix_mind_federation::FederationEngine;
use helix_mind_reincarnation::ReincarnationEngine;
use crate::proto::helix_mind_server::HelixMindServer;
use crate::proto::*;          // 导入所有 gRPC 消息类型
use std::sync::Arc;
use tonic::{Request, Response, Status};
use std::net::SocketAddr;

pub struct HelixMindServiceImpl {
    pub config: Config,
    pub storage: Arc<StorageEngine>,
    pub retrieval: Arc<RetrievalEngine>,
    pub metabolism: Arc<MetabolismEngine>,
    pub federation: Arc<FederationEngine>,
    pub reincarnation: Arc<ReincarnationEngine>,
}

impl HelixMindServiceImpl {
    pub fn new(
        config: Config,
        storage: Arc<StorageEngine>,
        retrieval: Arc<RetrievalEngine>,
        metabolism: Arc<MetabolismEngine>,
        federation: Arc<FederationEngine>,
        reincarnation: Arc<ReincarnationEngine>,
    ) -> Self {
        Self {
            config,
            storage,
            retrieval,
            metabolism,
            federation,
            reincarnation,
        }
    }
}

/// 启动 gRPC 服务（ADR-0019 P3b：支持 Tcp / Unix 双传输模式）。
///
/// - `Transport::Tcp`：远程部署；`listen_addr` 解析为 SocketAddr，mTLS 预留。
/// - `Transport::Unix`：本地 UDS；`listen_addr` 视为 socket 路径，
///   SO_PEERCRED 白名单鉴权（`trusted_uids`，fail-closed）。
pub async fn serve(
    config: &ApiConfig,
    service: HelixMindServiceImpl,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = HelixMindServer::new(service);
    // Z3 (P0-Pre): health probe is core gRPC infrastructure (the "serving lamp"),
    // standard gRPC/tonic probe — belongs here, not deferred to P3.
    let health = crate::health::HealthServer::new(crate::health::HealthServiceImpl);

    match config.transport {
        Transport::Tcp => {
            let addr: SocketAddr = config.listen_addr.parse()?;
            tracing::info!("Starting gRPC server (TCP) on {}", addr);
            let validation =
                crate::middleware::ValidationLayer::new(config.max_system_load);
            tonic::transport::Server::builder()
                .layer(tonic::service::interceptor(validation))
                .add_service(server)
                .add_service(health)
                .serve(addr)
                .await?;
        }
        Transport::Unix => {
            let path = &config.listen_addr;
            let _ = std::fs::remove_file(path);
            let listener = tokio::net::UnixListener::bind(path)?;
            tracing::info!("Starting gRPC server (UDS) on {}", path);
            let incoming = tokio_stream::wrappers::UnixListenerStream::new(listener);
            // 身份层（fail-closed）→ 内容层（metadata 校验 / 日志 / 负载）
            let auth = crate::middleware::PeerCredAuth::new(config.trusted_uids.clone());
            let validation =
                crate::middleware::ValidationLayer::new(config.max_system_load);
            tonic::transport::Server::builder()
                .layer(tonic::service::interceptor(auth))
                .layer(tonic::service::interceptor(validation))
                .add_service(server)
                .add_service(health)
                .serve_with_incoming(incoming)
                .await?;
        }
    }
    Ok(())
}

#[tonic::async_trait]
impl crate::proto::helix_mind_server::HelixMind for HelixMindServiceImpl {
    async fn query(&self, request: Request<QueryRequest>) -> Result<Response<QueryResponse>, Status> {
        if !self.config.api.layer1_enabled {
            return Err(Status::unavailable("Layer 1 API is disabled"));
        }
        super::layer1::handle_query(self, request).await
    }
    async fn remember(&self, request: Request<RememberRequest>) -> Result<Response<RememberResponse>, Status> {
        if !self.config.api.layer1_enabled {
            return Err(Status::unavailable("Layer 1 API is disabled"));
        }
        super::layer1::handle_remember(self, request).await
    }
    async fn forget(&self, request: Request<ForgetRequest>) -> Result<Response<ForgetResponse>, Status> {
        if !self.config.api.layer1_enabled {
            return Err(Status::unavailable("Layer 1 API is disabled"));
        }
        super::layer1::handle_forget(self, request).await
    }
    async fn advanced_query(&self, request: Request<AdvancedQueryRequest>) -> Result<Response<QueryResponse>, Status> {
        if !self.config.api.layer2_enabled {
            return Err(Status::unavailable("Layer 2 API is disabled"));
        }
        super::layer2::handle_advanced_query(self, request).await
    }
    async fn helix_query(&self, request: Request<HelixQueryRequest>) -> Result<Response<HelixQueryResult>, Status> {
        super::layer3::handle_helix_query(self, request).await
    }
    async fn helix_consolidate(&self, request: Request<HelixConsolidateRequest>) -> Result<Response<HelixConsolidateResult>, Status> {
        super::layer3::handle_helix_consolidate(self, request).await
    }
    async fn federated_dag_share(&self, request: Request<FederatedDagShareRequest>) -> Result<Response<FederatedDagShareResponse>, Status> {
        super::layer3::handle_federated_share(self, request).await
    }
    async fn trigger_reincarnation(&self, request: Request<TriggerReincarnationRequest>) -> Result<Response<TriggerReincarnationResponse>, Status> {
        super::layer3::handle_reincarnation(self, request).await
    }
    async fn reload_gene_lock(&self, request: Request<ReloadGeneLockRequest>) -> Result<Response<ReloadGeneLockResponse>, Status> {
        super::layer3::handle_reload_gene_lock(self, request).await
    }
    async fn sync_human_view(&self, request: Request<SyncHumanViewRequest>) -> Result<Response<SyncHumanViewResponse>, Status> {
        super::layer3::handle_sync_human_view(self, request).await
    }
}