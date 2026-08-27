- **决策日期**：2026-08-28
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：P3 计划（2026-08-28 起草，待审查）+ INTENT-7-SECURE spec §3（UDS SO_PEERCRED）+ 审查③ R4（UDS 是传输层重构）
- **状态**：Active（2026-08-28 审查通过，P3 编码开工）

# ADR-0019: P3b 传输层安全（UDS + SO_PEERCRED，远程 mTLS 预留）

> 编号说明：ADR-0015 预留给 P4.5 WAL，0016-0018 已用，故本决策取 0019。

## 状态

Active（2026-08-28 审查通过，P3 编码开工）

## 问题

代码真相源显示：
- `api::server::serve(addr: SocketAddr)` **仅 TCP**，无 UDS 支持；
- `middleware::ValidationLayer` 是空壳（TODO auth/load）；
- 本地部署无任何进程级鉴权，违反 INTENT-7-SECURE §3（本地 UDS SO_PEERCRED 0ms 验证）。
- 审查③ R4 指出：UDS 不是 middleware 修改，而是**传输层重构**（tonic `serve_with_incoming` + `UnixListener`）。

## 决策

### 1. `ApiConfig.transport` 枚举（本地默认 UDS，远程 TCP 预留）
```rust
pub enum Transport {
    Tcp { addr: SocketAddr },   // 远程部署；mTLS 预留（P3 只设计不实现）
    Unix { path: PathBuf },     // 本地部署默认；SO_PEERCRED 鉴权
}
```
- 本地默认 `Unix`（0ms 鉴权，符合极致节能）；远程默认 `Tcp`（mTLS 在 P3 后落地）。

### 2. UDS + SO_PEERCRED 鉴权（fail-closed）
- tonic `serve_with_incoming` + `tokio::net::UnixListener`；accept 时取 `peer_cred`：
  - Linux：`SO_PEERCRED`（ucred: UID/GID/PID）
  - macOS：`LOCAL_PEERCRED`
- **白名单校验**：`ApiConfig.trusted_uids: Vec<u32>`，连接 UID 不在白名单 → 立即断开 + 审计日志（`UNTRUSTED_PEER_UID` 0x07DA）。
- **默认拒绝**（fail-closed）：白名单为空 → 拒绝所有 UDS 连接（防误开）。
- 技术前提：tonic + UnixListener 的 peer_cred 注入需先做**最小原型验证**（连接级身份 → interceptor/request metadata），验证后再全量实现。

### 3. 远程 TCP：mTLS 预留（设计文档）
- P3 仅落地设计约束（cipher 套件、证书 pinning 到 L0 基因锁、traceparent 绑定 TLS 会话），不实现。实现走 P3 后独立 ADR。

### 4. `ValidationLayer` 真实化
- 从空壳变为：metadata 校验（traceparent 格式 / 必要 header）+ 请求日志 + 系统负载检查（超限返回 unavailable）。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 本地 0ms 进程级鉴权，零信任落地 | 传输层重构（serve 签名 / 启动路径改动） |
| fail-closed 默认拒绝，防误开 | 需要 UDS 原型验证（peer_cred 注入是未知点） |
| 远程 mTLS 预留不阻塞 P3 | 双传输模式增加配置面 |

## 回滚阈值

- 若 UDS 原型验证失败（peer_cred 无法注入 tonic）→ 回退为 UDS 通道 + socket 文件 `0600` + `trusted_uids` 应用层校验（降级方案，仍零信任），新 ADR Supersede 第 2 条。
- 若本地 TCP 兼容性测试受阻 → 保持 TCP 默认，UDS 作为显式启用项。

## 关联

- 前置：Z3（HealthServer 注册，P0-Pre 已做）
- 对齐：INTENT-7-SECURE §3（UDS SO_PEERCRED）、CI-144 门面 §7（traceparent 绑定）
- 后续：P3c ADR-0020（CI-144 对齐）、P4.5（mTLS 实现 ADR）
- 受审决策：D4（UDS/TCP 默认）/ D5（白名单配置）
