use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("grpc.health.v1");
}

pub use proto::health_server::HealthServer;
pub use proto::HealthCheckRequest;
pub use proto::HealthCheckResponse;
pub use proto::health_check_response::ServingStatus;

pub struct HealthServiceImpl;

#[tonic::async_trait]
impl proto::health_server::Health for HealthServiceImpl {
    type WatchStream = tokio_stream::wrappers::ReceiverStream<Result<HealthCheckResponse, Status>>;

    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: ServingStatus::Serving as i32,
        }))
    }

    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        Err(Status::unimplemented("watch not implemented"))
    }
}