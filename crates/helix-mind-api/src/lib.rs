pub mod layer1;
pub mod layer2;
pub mod layer3;
pub mod server;
pub mod health;
pub mod middleware;

// Generated proto code — exposed as crate::proto
pub mod proto {
    tonic::include_proto!("helix_mind");
}

pub use server::HelixMindServiceImpl;
pub use server::serve;