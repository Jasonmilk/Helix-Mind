//! UDS 最小原型验证（ADR-0019 P3b）：tonic 内置 UdsConnectInfo + SO_PEERCRED 白名单
//!
//! 验证目标：
//! 1. `serve_with_incoming` + `UnixListenerStream` 可启动 UDS 服务（传输层重构可行）
//! 2. tonic 内置 `UdsConnectInfo`（含 `peer_cred`）自动注入 request extensions
//! 3. interceptor 读取 `peer_cred.uid()` 做白名单校验，fail-closed（默认拒绝）
//! 4. 白名单内通过 / 白名单外拒绝

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

use helix_mind_api::health::proto::health_client::HealthClient;
use helix_mind_api::health::proto::HealthCheckRequest;
use helix_mind_api::health::HealthServiceImpl;
use helix_mind_api::health::HealthServer;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::codegen::http::Uri;
use tonic::transport::server::UdsConnectInfo;
use tonic::transport::{Endpoint, Server};
use tonic::{Request, Status};
use tower_service::Service;

/// 当前进程 UID（用于构造"白名单内"场景）
fn current_uid() -> u32 {
    unsafe { libc::getuid() }
}

/// Unix 连接器：让 tonic client 通过 UDS 连接
#[derive(Clone)]
struct UnixConnector {
    path: PathBuf,
}

impl Service<Uri> for UnixConnector {
    type Response = tokio::net::UnixStream;
    type Error = std::io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _uri: Uri) -> Self::Future {
        let path = self.path.clone();
        Box::pin(async move { tokio::net::UnixStream::connect(path).await })
    }
}

/// interceptor：读取 UdsConnectInfo.peer_cred.uid()，白名单校验，fail-closed
#[derive(Clone)]
struct PeerCredAuth {
    allowed_uids: Vec<u32>,
}

impl tonic::service::Interceptor for PeerCredAuth {
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

async fn spawn_server(path: &str, allowed_uids: Vec<u32>) {
    let _ = std::fs::remove_file(path);
    let listener = tokio::net::UnixListener::bind(path).expect("bind uds");
    let incoming = UnixListenerStream::new(listener);
    let health = HealthServer::new(HealthServiceImpl);
    let auth = PeerCredAuth { allowed_uids };
    tokio::spawn(async move {
        Server::builder()
            .layer(tonic::service::interceptor(auth))
            .add_service(health)
            .serve_with_incoming(incoming)
            .await
            .expect("serve uds");
    });
    // 等待 socket 就绪
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
}

async fn dial(path: &str) -> HealthClient<tonic::transport::Channel> {
    let channel = Endpoint::try_from("http://[::]:50051")
        .expect("endpoint")
        .connect_with_connector(UnixConnector {
            path: PathBuf::from(path),
        })
        .await
        .expect("connect uds");
    HealthClient::new(channel)
}

/// 场景 1：白名单含当前 UID → 请求通过（Serving）
#[tokio::test]
async fn uds_peercred_allowed() {
    let path = "/tmp/helix_uds_allowed.sock";
    spawn_server(path, vec![current_uid()]).await;

    let mut client = dial(path).await;
    let resp = client
        .check(HealthCheckRequest {
            service: String::new(),
        })
        .await
        .expect("check should pass for trusted uid");
    assert_eq!(resp.into_inner().status, 1); // ServingStatus::Serving
}

/// 场景 2：白名单为空（fail-closed）→ 请求被拒（permission_denied）
#[tokio::test]
async fn uds_peercred_denied_fail_closed() {
    let path = "/tmp/helix_uds_denied.sock";
    spawn_server(path, vec![]).await;

    let mut client = dial(path).await;
    let err = client
        .check(HealthCheckRequest {
            service: String::new(),
        })
        .await
        .expect_err("check should be rejected for empty whitelist");
    assert_eq!(err.code(), tonic::Code::PermissionDenied);
}
