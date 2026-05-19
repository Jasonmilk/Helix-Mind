use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct ValidationLayer;

impl ValidationLayer {
    pub fn new() -> Self {
        Self
    }
}

impl Interceptor for ValidationLayer {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // 1. Validate request metadata
        // TODO: Validate auth token if needed

        // 2. Log request
        let _method = "unknown".to_string();

        // 3. Check system load
        // TODO: Check system load, return unavailable if too high

        Ok(request)
    }
}
