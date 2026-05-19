pub mod layer1;
pub mod layer2;
pub mod layer3;
pub mod server;
pub mod health;
pub mod middleware;

// Generated proto code
tonic::include_proto!("helix_mind");

pub use helix_mind::helix_mind_server::HelixMindServer;
pub use self::server::HelixMindServiceImpl;
