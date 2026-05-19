pub mod server;
pub mod layer1;
pub mod layer2;
pub mod layer3;
pub mod middleware;
pub mod health;

// Generated proto code
tonic::include_proto!("helix_mind");

pub use self::helix_mind::helix_mind_server::HelixMindServer;
pub use self::server::HelixMindServiceImpl;
