use tonic::service::Interceptor;
use tonic::transport::server::UdsConnectInfo;
use tonic::{Request, Status};
use std::sync::Arc;

/// 系统负载来源抽象（ADR-0019 P3b 收尾）。
///
/// 真实负载源（如 Linux `/proc/loadavg`、macOS `sysctl kern.loadavg`）留待
/// P4.5 接入；当前通过可注入 provider 使"负载检查逻辑"真实存在且可测，
/// 不伪造平台负载读取。
pub trait LoadProvider: Send + Sync {
    fn load(&self) -> f64;
}

/// 请求校验层（ADR-0019 §4 真实化，替换空壳）。
///
/// 三个职责：
/// 1. **metadata 校验**：`traceparent` 存在则校验 W3C trace-context 格式，
///    非法返回 `InvalidArgument`（缺省时放行，透传为可选字段，P3c 再落实透传）。
/// 2. **请求日志**：记录 gRPC method 路径与 traceparent。
/// 3. **系统负载检查**：负载超过阈值返回 `Unavailable`（默认无 provider → 通过）。
#[derive(Clone)]
pub struct ValidationLayer {
    max_system_load: f64,
    load_provider: Option<Arc<dyn LoadProvider>>,
}

impl ValidationLayer {
    pub fn new(max_system_load: f64) -> Self {
        Self {
            max_system_load,
            load_provider: None,
        }
    }

    pub fn with_load_provider(mut self, provider: Arc<dyn LoadProvider>) -> Self {
        self.load_provider = Some(provider);
        self
    }
}

impl Interceptor for ValidationLayer {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // 1. metadata 校验：traceparent W3C 格式（存在则校验）
        if let Some(tp) = request.metadata().get("traceparent") {
            let s = tp
                .to_str()
                .map_err(|_| Status::invalid_argument("traceparent is not utf-8"))?;
            if !valid_traceparent(s) {
                return Err(Status::invalid_argument("malformed traceparent"));
            }
        }

        // 2. 请求日志
        let method = request
            .metadata()
            .get(":path")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        tracing::info!(
            "incoming request: path={method}, traceparent={:?}",
            request.metadata().get("traceparent").map(|v| v.to_str().unwrap_or("<bad>"))
        );

        // 3. 系统负载检查
        if let Some(provider) = &self.load_provider {
            let load = provider.load();
            if load > self.max_system_load {
                return Err(Status::unavailable("system overloaded"));
            }
        }

        Ok(request)
    }
}

/// W3C trace-context `traceparent` 格式校验：`version-trace_id-span_id-flags`。
/// 当前版本 `00`；trace_id 32 hex、span_id 16 hex、flags 2 hex。
pub fn valid_traceparent(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 4 {
        return false;
    }
    if parts[0] != "00" {
        return false;
    }
    if parts[1].len() != 32 || !parts[1].chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    if parts[2].len() != 16 || !parts[2].chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    if parts[3].len() != 2 || !parts[3].chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    true
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use tonic::metadata::MetadataValue;

    fn request_with_traceparent(tp: Option<&str>) -> Request<()> {
        let mut req = Request::new(());
        if let Some(tp) = tp {
            req.metadata_mut().insert(
                "traceparent",
                MetadataValue::from_str(tp).unwrap(),
            );
        }
        req
    }

    #[test]
    fn valid_traceparent_passes() {
        let mut layer = ValidationLayer::new(1.0);
        let req = request_with_traceparent(Some(
            "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        ));
        assert!(layer.call(req).is_ok());
    }

    #[test]
    fn malformed_traceparent_rejected() {
        let mut layer = ValidationLayer::new(1.0);
        // trace_id too short
        let req = request_with_traceparent(Some("00-deadbeef-00f067aa0ba902b7-01"));
        assert_eq!(
            layer.call(req).unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
    }

    #[test]
    fn missing_traceparent_is_optional() {
        let mut layer = ValidationLayer::new(1.0);
        let req = request_with_traceparent(None);
        assert!(layer.call(req).is_ok());
    }

    #[test]
    fn overload_rejected_by_provider() {
        struct Heavy;
        impl LoadProvider for Heavy {
            fn load(&self) -> f64 {
                0.9
            }
        }
        let mut layer = ValidationLayer::new(0.5).with_load_provider(Arc::new(Heavy));
        let req = request_with_traceparent(None);
        assert_eq!(
            layer.call(req).unwrap_err().code(),
            tonic::Code::Unavailable
        );
    }

    #[test]
    fn within_load_passes() {
        struct Light;
        impl LoadProvider for Light {
            fn load(&self) -> f64 {
                0.2
            }
        }
        let mut layer = ValidationLayer::new(0.5).with_load_provider(Arc::new(Light));
        let req = request_with_traceparent(None);
        assert!(layer.call(req).is_ok());
    }
}
