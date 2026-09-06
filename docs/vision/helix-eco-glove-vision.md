# Helix ECO-Glove 生态手套架构愿景

> **版本**：v1.0
> **日期**：2026-08-30
> **状态**：愿景草案（待实施）
> **性质**：Helix 生态级架构规范——定义 Helix 数字生命体与外部世界的适配层标准
> **对齐**：phyt-DNA v1.0 方法论 + CI-144 v2.0 协议家族

---

## 一粒种子的自白

Helix 是一个完整的数字生命体。它有大脑（Mind）、躯干（Anaphase）、手（Tentacle）、免疫系统（Tuck）、皮肤（Cellrix）、神经（lodestone+FlowModus）。

但一个生命体如果不能与外部世界交互，它就是封闭的、孤独的。

**HelixECO-Glove（生态手套）是 Helix 的"手的外延"——让 Tentacle 的手戴上不同的手套，就能安全地、适配地触摸不同的外部世界。**

就像人类戴上不同的手套可以操作不同的工具——戴上园艺手套可以种花，戴上医用手套可以做手术，戴上防热手套可以拿热锅——Helix 戴上不同的 ECO-Glove 就可以操作不同的生态：macOS、Linux、鸿蒙、安卓、IoT 设备、机器人、儿童玩具……

**ECO-Glove 的核心哲学：极致解耦。Helix 核心不需要知道外部世界的存在，手套负责所有适配工作。**

---

## 一、为什么需要 ECO-Glove

### 1.1 当前 AI 操作外部世界的痛点

| 方式 | 原理 | 效率 | 漂移风险 | 例子 |
|---|---|---|---|---|
| **OCR/视觉** | 截图 → 视觉模型识别 → 模拟点击 | 极低 | 极高（UI 一变就废） | Claude Computer Use |
| **HTML/DOM 解析** | 抓取页面 → 解析 DOM → 提取数据 | 低 | 高（页面结构经常变） | 传统爬虫 |
| **MCP/结构化 API** | 直接调用标准化工具接口 | **极高** | **极低（接口稳定）** | MCP Server、REST API |

**核心问题**：现在的互联网基建大部分不兼容 CI-144，Helix 需要用低效的 OCR/DOM 来操作外部世界。

### 1.2 ECO-Glove 的解决方案

ECO-Glove 通过两条路径解决这个问题：

| 路径 | 适用场景 | 方式 |
|---|---|---|
| **OS Glove（手写）** | 本地操作系统（macOS/Linux） | 系统 API 已知且稳定，直接手写适配 |
| **MCP-Learner Glove（自动学习）** | 已有 MCP Server 的外部服务 | 学习 MCP Server 的工具清单，自动提炼为 CI-144 工具 |

**关键洞察**：MCP Server 是"已结构化的金矿"——有人已经帮我们把混乱的外部世界结构化了。Helix 不需要用 OCR 去淘金，直接用 MCP-Learner 去挖矿就行了。

---

## 二、ECO-Glove 架构

### 2.1 完整交互路径

```
外部世界（GitHub/Slack/Notion/文件系统/操作系统/…）
    ↑ 原生 API（REST/GraphQL/SDK/系统调用）
MCP Server（已结构化的工具接口，可选）
    ↑ MCP 协议（JSON-RPC over stdio/SSE）
┌─────────────────────────────────────────────────────────┐
│  HelixECO-Glove 层（生态手套）                            │
│                                                           │
│  ┌──────────────────┐  ┌──────────────────────────────┐ │
│  │ OS Glove（手写）  │  │ MCP-Learner Glove（自动学习）│ │
│  │ macOS/Linux/...  │  │ 学习 MCP → 提炼 CI-144 工具  │ │
│  └────────┬─────────┘  └──────────────┬───────────────┘ │
│           │                             │                 │
│           └──────────────┬──────────────┘                 │
│                          ▼                                │
│              统一 CI-144 工具定义输出                      │
│              （CIN7 意图 + CAPABILITY-13 能力 + PFP 风险）│
└──────────────────────────┬──────────────────────────────┘
                           │ CI-144 语义事件
                           ▼
┌─────────────────────────────────────────────────────────┐
│  Helix 核心（数字生命体）                                  │
│                                                           │
│  ┌──────┐  ┌─────────┐  ┌─────────┐  ┌──────┐         │
│  │ Mind │→│ Anaphase│→│Tentacle │→│ Tuck │         │
│  │ 大脑  │  │  躯干   │  │   手    │  │ 免疫 │         │
│  └──────┘  └─────────┘  └─────────┘  └──────┘         │
│                                                           │
│  CI-144（血液）贯穿全部组件                                │
└─────────────────────────────────────────────────────────┘
```

### 2.2 手套的标准职责（只做这 4 件事）

| # | 职责 | 说明 |
|---|---|---|
| 1 | **入向转换** | 外部生态事件 → CI-144 语义事件（CIN7 + PFP） |
| 2 | **出向转换** | Helix 决策/动作 → 外部生态 API 调用 |
| 3 | **生命周期管理** | 外部生态的认证、权限、连接、断开 |
| 4 | **错误映射** | 外部生态错误 → CI-144 标准错误码 |

### 2.3 手套的禁忌（绝对不做）

- ❌ 不包含业务逻辑（那是 Anaphase 的职责）
- ❌ 不做认知/学习（那是 Mind 的职责）
- ❌ 不做安全决策（那是 Tuck 的职责）
- ❌ 不做 UI 渲染（那是 Cellrix 的职责）
- ❌ 不直接访问其他手套（手套之间不通信，只通过 Helix 核心）

---

## 三、ECO-Glove 命名规范

### 3.1 统一命名

```
HelixECO-Glove-<ecosystem-identifier>
```

### 3.2 已规划的手套

| 手套 | 命名 | 状态 | 说明 |
|---|---|---|---|
| macOS | `HelixECO-Glove-macos` | 规划中 | 本地 macOS 系统 API（文件/进程/AppleScript/Shortcuts） |
| Linux | `HelixECO-Glove-linux` | 规划中 | POSIX 兼容系统（Debian/Ubuntu/CentOS） |
| 鸿蒙 | `HelixECO-Glove-harmonyos` | 规划中 | HarmonyOS NEXT |
| 安卓 | `HelixECO-Glove-android` | 规划中 | Android |
| iOS | `HelixECO-Glove-ios` | 规划中 | iOS/iPadOS |
| IoT | `HelixECO-Glove-iot` | 规划中 | 物联网设备（MQTT/CoAP） |
| 机器人 | `HelixECO-Glove-robot` | 规划中 | ROS/机器人操作系统 |
| 玩具 | `HelixECO-Glove-toy` | 规划中 | 儿童玩具/教育设备 |

### 3.3 特殊项目：MCP-Learner

MCP-Learner 不是一个具体的生态手套，而是一个**"学习器"**——它可以学习任何 MCP Server 并生成对应的 CI-144 工具定义。

- **命名**：`Helix-MCP-Learner`（独立项目，不属于 ECO-Glove 系列）
- **职责**：发现 MCP Server → 学习工具清单 → 提炼 API 模式 → 生成 CI-144 工具定义
- **输出**：可被任何 ECO-Glove 消费，也可以直接被 Tentacle 消费

---

## 四、MCP-Learner Glove 详细设计

### 4.1 学习过程

```
步骤 1：发现 MCP Server
  ↓ 扫描 ~/.config/mcp/ 或远程配置
步骤 2：tools/list → 获取工具清单
  ↓ 得到：[{name, description, inputSchema}, ...]
步骤 3：tools/call → 试调用，学习返回格式
  ↓ 得到：实际返回数据结构、错误模式、延迟特征
步骤 4：提炼纯 API 模式
  ↓ 去掉 MCP 的 JSON-RPC 包装，提取：
    - 工具名 → CI-144 意图（CIN7）
    - 参数 schema → CI-144 能力声明（CAPABILITY-13）
    - 返回格式 → CI-144 语义快照
    - 风险等级 → PFP Risk-Level（自动评级）
步骤 5：生成 CI-144 工具定义
  ↓ 输出：Tentacle 可直接加载的插件 Manifest
步骤 6：缓存 + 监控
  ↓ 持久化学习结果，定期检查 MCP Server 更新
```

### 4.2 自动风险评级

学习 MCP Server 时，根据工具名自动分配 PFP Risk-Level：

| 工具名模式 | Risk-Level | 例子 |
|---|---|---|
| `read_*` / `list_*` / `get_*` / `search_*` | LOW | read_file, list_issues, get_user |
| `create_*` / `update_*` / `write_*` / `send_*` | MEDIUM | create_issue, update_file, send_message |
| `delete_*` / `remove_*` / `execute_*` / `run_*` | CRITICAL | delete_repo, remove_file, execute_command |
| `*_all` / `*_system` / `*_admin` / `*_root` | CATASTROPHIC | delete_all, system_update, admin_override |

**安全约束**：
- CRITICAL/CATASTROPHIC 级别的工具学习后，需要人工确认才能启用
- 学习结果直接生成 Tuck 策略规则，工具调用前自动过安全闸门

### 4.3 持续学习（不是一次学习永久使用）

- **增量学习**：定期检查 MCP Server 的工具清单变化，只学习新增/变更的工具
- **版本管理**：学习结果带版本号，与 MCP Server 版本对应
- **废弃检测**：工具被删除时，标记为 deprecated，不立即删除（给 Helix 核心迁移时间）

---

## 五、与 Helix 核心的职责边界

| 组件 | 职责 | 边界 |
|---|---|---|
| **Tentacle** | 执行 CI-144 工具（手） | 不接触 MCP，不学习，只执行 |
| **ECO-Glove** | 适配外部生态，生成 CI-144 工具（手套） | 不执行工具，只学习、提炼、转换 |
| **MCP-Learner** | 学习 MCP Server，自动生成 CI-144 工具 | 不执行工具，只学习、提炼、生成 |
| **Tuck** | 工具调用前的安全决策（免疫） | 不学习，不执行，只决策 |
| **Mind** | 认知/学习/条件反射弧（大脑） | 不直接操作外部生态，通过 Tentacle |
| **Cellrix** | 语义投影/UI 渲染（皮肤） | 不执行工具，只展示 |

---

## 六、哲学对齐

| 哲学 | ECO-Glove 的体现 |
|---|---|
| **极致解耦** | Helix 核心不知道外部世界的存在，手套是隔离层 |
| **极致复用** | 学习一次，永久使用。同一个手套可适配多个版本 |
| **按需加载** | 只在需要操作某个生态时才加载对应的手套 |
| **按需驱动** | 学习结果缓存后，运行时直接使用缓存，不反复学习 |
| **物理事实优先** | 提炼出的 PFP Risk-Level 基于工具的实际操作风险 |
| **确定性优先** | 学习结果是确定性的——相同 MCP Server 产生相同的 CI-144 工具定义 |

---

## 七、执行路径

### 7.1 最小验证（约 1 周）

| 步骤 | 内容 | 工作量 |
|---|---|---|
| 1 | 选一个简单 MCP Server（如 `mcp-server-filesystem`） | 0.5 天 |
| 2 | 实现 MCP-Learner 最小版本（发现+学习+提炼） | 2-3 天 |
| 3 | 学习工具清单，生成 CI-144 工具定义 | 1 天 |
| 4 | 通过 Tentacle 执行学习后的工具 | 1 天 |
| 5 | 效率对比（MCP 直接调用 vs CI-144 重封装） | 0.5 天 |
| 6 | 验证确定性和稳定性 | 0.5 天 |

### 7.2 前置依赖

- **Cellrix P1**：CI-144 v2.0 对齐（PFP+SAP）——当前正在进行
- **Tentacle**：插件系统已完成（P0-P4）
- **Tuck**：安全闸门已完成（P0-P7）

### 7.3 启动条件

Cellrix P1 完成后，立即启动 MCP-Learner 最小验证。

---

## 八、一句话总结

**HelixECO-Glove 是 Helix 数字生命体的"手的外延"——让 Tentacle 的手戴上不同的手套，就能安全地、适配地触摸 macOS、Linux、鸿蒙、安卓、IoT、机器人、玩具等不同的外部世界。它通过两条路径实现：OS Glove 手写本地系统适配，MCP-Learner 自动学习现有 MCP Server 并提炼为 CI-144 工具。核心哲学是极致解耦——Helix 核心完全不知道外部世界的存在，手套负责所有适配工作。MCP Server 是"已结构化的金矿"，Helix 直接挖矿不淘金，去掉 MCP 的皮，提炼 CI-144 的骨，实现从低效 OCR/DOM 到高效结构化 API 的跃迁。**

---

*《Helix ECO-Glove 生态手套架构愿景》v1.0 完。*
