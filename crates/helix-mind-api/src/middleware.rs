use tonic::service::Interceptor;
use tonic::{Request, Status};
use tracing::warn;

#[derive(Clone)]
pub struct ValidationLayer;

impl ValidationLayer {
    pub fn new() -> Self {
        Self
    }
}

impl Interceptor for ValidationLayer {
    fn call(&self, mut request: Request<()>) -> Result<Request<()>, Status> {
        // 1. Validate request metadata
        // TODO: Validate auth token if needed

        // 2. Log request
        let method = request.method().map(|m| m.path()).unwrap_or("unknown");
        tracing::debug!("API request: {}", method);

        // 3. Check system load
        // TODO: Check system load, return unavailable if too high

        Ok(request)
    }
}
