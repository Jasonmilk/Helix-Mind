# 远程 mTLS 设计（ADR-0019 §3 预留 — P3 只设计不实现）

> **状态**：Design（仅约束，未实现）
> **日期**：2026-08-28
> **对齐**：ADR-0019（P3b 传输层安全）、INTENT-7-SECURE（CI-144）、零信任管道公理
> **实现策略**：实现走 P3 后独立 ADR（Supersede 本设计中的可执行条款，不覆写本文档）

## 一、目标

Helix-Mind 远程 `Tcp` 部署（多租户 / 商业化 / SaaS 场景）下的传输层身份认证与机密性。
本地 `Unix` 部署已用 SO_PEERCRED 白名单（ADR-0019 已落地），**远程 = mTLS**。

## 二、PKI 层次（三层，拒绝自签平铺）

```
Root CA（离线保管，防泄露）
   └── Intermediate CA（签名服务器/客户端证书；可轮换）
         ├── Server Cert  （每 Mind 实例一张，绑定 helix_id + l0_hash）
         └── Client Cert  （每身体/Anaphase 一张，绑定其身份）
```

- **Root CA 离线**：仅在签发中间 CA 时上线，其余时间物理隔离（极致节能 + 零信任）。
- **Intermediate CA 可轮换**：泄露时吊销中间 CA，不影响 Root。

## 三、证书身份绑定（与生态对齐）

| 实体 | 证书 CN / SAN 绑定 |
|---|---|
| Mind 服务器 | `helix_id`（如 "Helix 7th Dash"） |
| Anaphase 客户端 | 其 `helix_id` + 角色声明 |
| 联邦对等方 | `helix_id` + `l0_hash` |

- 客户端证书的 `l0_hash` 必须与身份平台注册的 L0 哈希一致（INTENT-7-SECURE 信用锚定）。
- **证书 pinning 到 L0 基因锁**：Mind 只信任其基因锁已知 CA 签发的证书（白名单 CA），不信任公共 CA 链。

## 四、传输参数约束（设计即冻结）

| 项 | 约束 |
|---|---|
| 协议 | TLS 1.3（拒绝 1.2 及以下） |
| 套件 | 仅 AEAD（AES-256-GCM / CHACHA20-POLY1305）；禁用 CBC/RC4/3DES |
| 曲线 | X25519 / P-256（优先 X25519） |
| 双向 | `require_client_cert`（客户端必须出示证书，否则拒绝握手） |
| 会话 | 会话复用不超过 5 分钟；完美前向保密（PFS）必开 |
| 哈希 | 证书链签名 SHA-256 起；BLAKE3 仅用于完整性，不用于签名 |

## 五、traceparent 绑定 TLS 会话（CI-144 门面 §7）

- 每个 TLS 会话在握手完成时生成会话级 `traceparent`（W3C），贯穿该会话内全部 RPC。
- 应用层 traceparent 与 TLS 会话绑定，供审计（Tuck CAS）做「会话 → 请求 → 认知循环」全链路追溯。
- 具体透传机制由 P3c（ADR-0020）落实应用层，TLS 会话绑定由实现 ADR 落实。

## 六、密钥与证书轮换

| 项 | 周期 | 说明 |
|---|---|---|
| 服务器证书 | 90 天 | 自动轮换，滚动重启 |
| 客户端证书 | 90 天 | 由 Anaphase 侧管理 |
| Intermediate CA | 1 年 | 签发新叶子，短时双 CA 并存 |
| Root CA | 5 年 / 泄露即换 | 离线 |

轮换采用 `Superseded` 语义：旧证书不删除，标记 `Superseded`（记忆不可篡改投射）。

## 七、与 INTENT-7-SECURE 的分工（三层纵深）

| 层 | 组件 | 职责 |
|---|---|---|
| 传输/身份 | CI-144 INTENT-7-SECURE | mTLS 身份、TLS 1.3、会话绑定 |
| 网关/审计 | Tuck | CAS 审计、语义脱敏、流量调度 |
| 应用层 | Helix-Mind ValidationLayer | traceparent 校验、日志、负载 |

## 八、回滚阈值（诚实声明）

- 若 TLS 1.3 + 双向客户端证书在目标部署环境不可用 → 降级为 TLS 1.2 + 客户端证书，新 ADR Supersede §四。
- 若证书分发成为运维瓶颈 → 引入内部 ACME（证书自动签发），仍保持三层 PKI。

## 九、未决事项（实现 ADR 前需定）

1. 证书存储位置与权限（文件系统 0600 vs 密钥托管服务）。
2. 联邦对等方证书互认（跨实例 CA 信任是否需跨域根）。
3. 与 `trusted_uids`（UDS）是否共享同一身份配置面。
