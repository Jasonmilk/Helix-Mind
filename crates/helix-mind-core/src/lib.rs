pub mod config;
pub mod error;
pub mod graph;
pub mod tracing;

pub use config::Config;
pub use error::MindError;
pub use graph::*;
// graph 模块已经公开了 DeepColdStub 和 AuditEntry

use sha2::{Sha256, Digest};

pub fn sha256_digest(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub mod audit;