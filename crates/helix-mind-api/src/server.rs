use helix_mind_core::config::Config;
use helix_mind_storage::StorageEngine;
use helix_mind_retrieval::RetrievalEngine;
use helix_mind_metabolism::MetabolismEngine;
use helix_mind_federation::FederationEngine;
use helix_mind_reincarnation::ReincarnationEngine;
use std::sync::Arc;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use super::*;

pub struct HelixMindServiceImpl {
    config: Config,
    storage: Arc<StorageEngine>,
    retrieval: Arc<RetrievalEngine>,
    metabolism: Arc<MetabolismEngine>,
    federation: Arc<FederationEngine>,
    reincarnation: Arc<ReincarnationEngine>,
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

    pub async fn start(self, addr: &str) -> Result<(), helix_mind_core::error::MindError> {
        let service = HelixMindServer::new(self)
            .max_decoding_message_size(1024 * 1024 * 10) // 10MB
            .layer(super::middleware::ValidationLayer::new());

        tonic::transport::Server::builder()
            .add_service(service)
            .serve(addr.parse()?)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl helix_mind_server::HelixMind for HelixMindServiceImpl {
    // Layer 1
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        if !self.config.api.layer1_enabled {
            return Err(Status::unavailable("Layer 1 API is disabled"));
        }
        super::layer1::handle_query(self, request).await
    }

    async fn remember(
        &self,
        request: Request<RememberRequest>,
    ) -> Result<Response<RememberResponse>, Status> {
        if !self.config.api.layer1_enabled {
            return Err(Status::unavailable("Layer 1 API is disabled"));
        }
        super::layer1::handle_remember(self, request).await
    }

    async fn forget(
        &self,
        request: Request<ForgetRequest>,
    ) -> Result<Response<ForgetResponse>, Status> {
        if !self.config.api.layer1_enabled {
            return Err(Status::unavailable("Layer 1 API is disabled"));
        }
        super::layer1::handle_forget(self, request).await
    }

    // Layer 2
    async fn advanced_query(
        &self,
        request: Request<AdvancedQueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        if !self.config.api.layer2_enabled {
            return Err(Status::unavailable("Layer 2 API is disabled"));
        }
        super::layer2::handle_advanced_query(self, request).await
    }

    // Layer 3
    async fn helix_query(
        &self,
        request: Request<HelixQueryRequest>,
    ) -> Result<Response<HelixQueryResult>, Status> {
        super::layer3::handle_helix_query(self, request).await
    }

    async fn helix_consolidate(
        &self,
        request: Request<HelixConsolidateRequest>,
    ) -> Result<Response<HelixConsolidateResult>, Status> {
        super::layer3::handle_helix_consolidate(self, request).await
    }

    async fn federated_dag_share(
        &self,
        request: Request<FederatedDAGShareRequest>,
    ) -> Result<Response<FederatedDAGShareResponse>, Status> {
        super::layer3::handle_federated_share(self, request).await
    }

    async fn trigger_reincarnation(
        &self,
        request: Request<TriggerReincarnationRequest>,
    ) -> Result<Response<TriggerReincarnationResponse>, Status> {
        super::layer3::handle_reincarnation(self, request).await
    }

    async fn reload_gene_lock(
        &self,
        request: Request<ReloadGeneLockRequest>,
    ) -> Result<Response<ReloadGeneLockResponse>, Status> {
        super::layer3::handle_reload_gene_lock(self, request).await
    }

    async fn sync_human_view(
        &self,
        request: Request<SyncHumanViewRequest>,
    ) -> Result<Response<SyncHumanViewResponse>, Status> {
        super::layer3::handle_sync_human_view(self, request).await
    }
}
