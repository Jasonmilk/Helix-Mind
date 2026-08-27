use tonic::service::Interceptor;
use tonic::transport::server::UdsConnectInfo;
use tonic::{Request, Status};

/// 请求校验层（P3b 真实化）：metadata 校验 + 系统负载检查占位。
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

/// SO_PEERCRED 白名单鉴权（UDS 本地部署，fail-closed）—— ADR-0019 P3b。
///
/// tonic 内置 `UdsConnectInfo`（含 `peer_cred`）自动注入 request extensions，
/// 无需自定义 `Connected` trait（UDS 最小原型已验证）。
/// 连接 UID 不在白名单 → 拒绝（`UNTRUSTED_PEER_UID` 0x07DA 语义）；
/// 白名单为空 → 拒绝一切（默认 fail-closed，防误开）。
#[derive(Clone)]
pub struct PeerCredAuth {
    allowed_uids: Vec<u32>,
}

impl PeerCredAuth {
    pub fn new(allowed_uids: Vec<u32>) -> Self {
        Self { allowed_uids }
    }
}

impl Interceptor for PeerCredAuth {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let uid = request
            .extensions()
            .get::<UdsConnectInfo>()
            .and_then(|info| info.peer_cred.as_ref())
            .map(|cred| cred.uid());
        match uid {
            Some(uid) if self.allowed_uids.contains(&uid) => Ok(request),
            Some(uid) => Err(Status::permission_denied(format!(
                "untrusted peer uid: {uid}"
            ))),
            None => Err(Status::unauthenticated("no peer credential available")),
        }
    }
}
