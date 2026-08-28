//! Rhizax 消费者预留接口（P4 M-12）。
//!
//! ECOSYSTEM §4.6：Rhizax 通过 Unix Domain Socket 提供三个极简接口，
//! 作为零信任传输面：
//! - `resolve_ipns(name) → CID`
//! - `get_block(cid) → data`
//! - `publish_block(data) → CID`
//!
//! **能力未就绪 = 功能不存在**（ADR-0018 同款铁律）：接口签名已预留，
//! 但所有调用返回"未就绪"错误，绝不提供假实现/stub 默认值。
//! Rhizax 自身（新立项）就绪后，由新 ADR 打开此模块的实现路径。

use helix_mind_core::error::MindError;

/// Rhizax 客户端占位（预留 UDS 路径等字段，未来实现填充）。
#[derive(Debug, Clone)]
pub struct RhizaxClient {
    /// 预留：UDS socket 路径（ECOSYSTEM §4.6：权限 0600，commonintents 组）。
    pub uds_path: Option<String>,
}

impl RhizaxClient {
    /// 构造一个未接入的 Rhizax 客户端（`uds_path = None`，表示能力未就绪）。
    pub fn unprovisioned() -> Self {
        Self { uds_path: None }
    }

    /// 解析 IPNS 名称到当前 CID。
    pub async fn resolve_ipns(&self, name: &str) -> Result<String, MindError> {
        let _ = name;
        Err(MindError::Federation(
            "rhizax capability not ready (P4 M-12): resolve_ipns unimplemented".into(),
        ))
    }

    /// 拉取 CID 对应的数据块。
    pub async fn get_block(&self, cid: &str) -> Result<Vec<u8>, MindError> {
        let _ = cid;
        Err(MindError::Federation(
            "rhizax capability not ready (P4 M-12): get_block unimplemented".into(),
        ))
    }

    /// 发布数据块到 IPFS，返回 CID。
    pub async fn publish_block(&self, data: &[u8]) -> Result<String, MindError> {
        let _ = data;
        Err(MindError::Federation(
            "rhizax capability not ready (P4 M-12): publish_block unimplemented".into(),
        ))
    }
}

impl Default for RhizaxClient {
    fn default() -> Self {
        Self::unprovisioned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unprovisioned_client_refuses_all_calls() {
        let client = RhizaxClient::unprovisioned();
        assert!(client.resolve_ipns("example.ipns").await.is_err());
        assert!(client.get_block("QmX").await.is_err());
        assert!(client.publish_block(b"data").await.is_err());
    }
}
