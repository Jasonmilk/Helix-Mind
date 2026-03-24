# Helix-Mind: 硅基生命的认知中枢与记忆海马体

**自主记忆管理 | 任务拆解引擎 | 人格隔离存储 | 认知闭环协同**

> 研究定位：私有边缘集群环境下，智能体的长期记忆存储、任务拆解调度与人格管理中枢

---

## 摘要

Helix-Mind 是 Anaphase-Helix 自主进化系统的**认知中枢**，承担智能体的记忆管理、任务拆解、人格隔离与认知闭环协调。它摒弃传统对话式上下文窗口的碎片化记忆模式，以 Markdown 文件为底层存储（人类可读、可 Git 追溯），通过**基因锁（Gene Lock）**、**短期记忆海马体（Hippocampus）**、**长期记忆（Long‑Term）**、**任务队列（To‑Do）** 四个核心模块，构建起可回溯、可压缩、可继承的智能体认知体系。

系统与 **Tuck（模型调度网关）** 及 **Anaphase（安全执行沙箱）** 构成三位一体分布式拓扑，实现认知‑调度‑执行的物理隔离与无缝协同，让智能体在资源受限的边缘环境中实现可持续的自主进化。

---

## 1 项目概述

### 1.1 研究背景
现有大模型智能体普遍将记忆与上下文绑定在对话窗口内，导致：
- 长程记忆丢失，无法形成持续的经验积累；
- 任务拆解与执行耦合，难以实现分工协作；
- 人格管理依赖硬编码或外部服务，缺乏统一规范。

### 1.2 核心目标
Helix-Mind 聚焦三个核心能力：
1. **长期记忆管理**：将短期对话经验提炼为结构化长期记忆，支持按需检索与自动压缩；
2. **任务拆解调度**：接收高层需求，通过大脑模型（8B）将模糊目标拆解为原子任务，写入队列供执行单元认领；
3. **人格服务化**：集中管理智能体人格（system_prompt + 参数），通过 REST API 供 Tuck 网关动态调用。

### 1.3 核心优势
- **人类可读的存储**：所有记忆以 `.md` 文件保存，支持直接用编辑器修改、Git 版本管理；
- **并发安全**：使用 `portalocker` 实现文件锁，支持多进程同时读写；
- **模型无关**：大脑调用 Tuck 网关，可无缝切换任何兼容 OpenAI 的后端模型；
- **闭环可观测**：所有需求入栈、任务拆解、执行结果均写入海马体，形成完整审计日志。

---

## 2 物理隔离架构中的定位

系统严格部署于 `10.0.0.x` 私有内网，Helix-Mind 作为**认知中枢**，与 Tuck 网关、Anaphase 执行沙箱形成三位一体分布式拓扑：

| 节点IP | 模块定位 | 核心组件 | 核心职能 |
| :--- | :--- | :--- | :--- |
| **10.0.0.2** | **认知中枢** | **Helix-Mind** | 管理智能体记忆、任务拆解、人格服务、认知闭环协调 |
| **10.0.0.54** | 模型调度网关 | Tuck 人格路由引擎 | 多模型实例调度、人格隔离、请求路由 |
| **10.0.0.206** | 安全执行沙箱 | Anaphase 内核 | 任务隔离执行、代码安全扫描、迭代成果固化 |

> 架构特性：三层物理隔离设计，实现**认知管理、模型调度、安全执行**的解耦，兼顾演化效率与系统安全性。

---

## 3 核心组件

### 3.1 记忆管理器（MemoryManager）
- **基因锁（gene_lock.md）**：不可违背的底层规则，等同于“思想钢印”，每次任务拆解时自动注入大脑 Prompt；
- **海马体（hippocampus.md）**：短期记忆，记录用户需求、任务执行汇报、系统异常等，作为上下文窗口的输入；
- **长期记忆（long_term.md）**：经史官提炼后的经验、偏好、世界观，未来可支持向量检索；
- **任务队列（todo.md）**：大脑拆解后的原子任务列表，供手脚轮询认领，支持 UUID 标识与并发弹出。

### 3.2 大脑引擎（Brain）
- 调用 Tuck 网关的 8B 模型（如 DeepSeek-R1），根据基因锁、海马体记忆将用户需求拆解为具体动作序列；
- 具备重试机制与正则 JSON 提取，抵御模型输出不稳定；
- 拆解成功后自动写入 `todo.md`，失败时记录错误至海马体。

### 3.3 人格服务（Persona API）
- 提供 `/v1/persona/{name}` 端点，返回 `system_prompt` 与 `params`；
- Tuck 网关通过 `X-Tuck-Persona` 头动态获取人格，实现人格与代码解耦；
- 人格文件以 `personas/{name}.json` 存储，支持热更新。

### 3.4 任务调度 API
- `/v1/mind/think`：接收用户需求，异步触发大脑拆解，写入海马体；
- `/v1/mind/todo/pop`：供 Anaphase 手脚轮询，弹出第一个未完成任务（原子操作，带锁）；
- `/v1/mind/report`：手脚执行完毕后汇报结果，自动追加至海马体。

---

## 4 目录结构

```
Helix-Mind/
├── memory_base/               # 记忆存储（Markdown）
│   ├── gene_lock.md           # 基因锁（思想钢印）
│   ├── hippocampus.md         # 短期记忆海马体
│   ├── long_term.md           # 长期记忆
│   └── todo.md                # 任务队列
├── personas/                  # 人格配置（JSON）
│   └── default.json
├── core/
│   ├── memory_manager.py      # 带文件锁的记忆管理器
│   └── brain.py               # 大脑拆解引擎（含重试、JSON解析）
├── config.py                  # 配置加载（Pydantic Settings）
├── mind_server.py             # FastAPI 服务入口
├── requirements.txt
└── .env                       # 环境变量（不提交）
```

---

## 5 部署与运行指南

### 5.1 环境要求
- Python 3.10+
- 可访问 Tuck 网关（10.0.0.54:8686）且 Tuck 中已注册 8B 大脑模型
- 内网节点（建议与 Anaphase 同机或认知中枢节点）

### 5.2 部署步骤
```bash
# 1. 克隆项目
git clone https://github.com/Jasonmilk/Helix-Mind.git /opt/Helix-Mind
cd /opt/Helix-Mind

# 2. 创建虚拟环境
python3 -m venv venv
source venv/bin/activate

# 3. 安装依赖
pip install -r requirements.txt

# 4. 配置环境变量
cp .env.example .env
nano .env
```

`.env` 关键配置项示例：
```env
TUCK_URL=http://10.0.0.54:8686/v1/chat/completions
TUCK_API_KEY=sk-你的密钥
BRAIN_MODEL=DeepSeek-R1-0528-Qwen3-8B-IQ4_NL.gguf
MIND_PORT=8020
```

### 5.3 启动方式
```bash
# 前台运行（调试）
python mind_server.py

# 后台运行（使用 tmux）
tmux new -s helix-mind
python mind_server.py
# Ctrl+B, D 分离
```

### 5.4 使用 systemd 管理
创建 `/etc/systemd/system/helix-mind.service`，内容参考：
```ini
[Unit]
Description=Helix-Mind Cognitive Hub
After=network.target

[Service]
User=root
WorkingDirectory=/opt/Helix-Mind
Environment="PATH=/opt/Helix-Mind/venv/bin"
ExecStart=/opt/Helix-Mind/venv/bin/python /opt/Helix-Mind/mind_server.py
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```
```bash
systemctl daemon-reload
systemctl enable helix-mind
systemctl start helix-mind
```

---

## 6 API 文档

### 6.1 人格获取
`GET /v1/persona/{name}`  
返回 `{"system_prompt": "...", "params": {...}}`  
供 Tuck 网关调用，实现人格动态注入。

### 6.2 提交需求（异步思考）
`POST /v1/mind/think`  
请求体：`{"text": "用户需求"}`  
返回 `{"status": "Thinking..."}`，后台大脑拆解任务并写入队列。

### 6.3 获取任务（手脚认领）
`GET /v1/mind/todo/pop`  
返回 `{"has_task": true, "task": {"id": "xxx", "content": "任务内容"}}`  
原子操作，弹出队列头，已弹出任务标记为 `[x]`。

### 6.4 汇报结果
`POST /v1/mind/report`  
请求体：`{"task_id": "xxx", "status": "success/failed", "detail": "执行详情"}`  
结果追加至海马体，供后续认知参考。

### 6.5 健康检查
`GET /health`  
返回服务状态与配置的 `brain_model`。

---

## 7 标准化运行日志（示例）

```
============================================================
[认知中枢] 生命周期启动 | 大脑模型: DeepSeek-R1-8B
============================================================
[入口] 接收需求: "请帮我查看 /opt 目录下的文件列表"
[海马体] 写入: 【新需求入栈】: 请帮我查看 /opt 目录下的文件列表
[大脑] 调用 Tuck 网关进行任务拆解...
[大脑] 拆解结果: ["列出 /opt 目录下的所有文件和文件夹"]
[任务队列] 写入: - [ ][id:3a7f] 列出 /opt 目录下的所有文件和文件夹
[手脚] 认领任务 ID: 3a7f | 内容: 列出 /opt 目录下的所有文件和文件夹
[手脚] 执行结果: 成功执行，输出目录列表...
[汇报] 写入海马体: 【任务汇报 - ID:3a7f | 状态:success】:
[认知中枢] 闭环完成，等待下一轮需求。
============================================================
```

---

## 8 生态协同系统

- **Tuck**：模型调度与人格路由网关，Helix-Mind 的大脑通过 Tuck 调用 8B 模型拆解任务，同时 Tuck 通过 Helix-Mind 获取人格；
- **Anaphase**：安全执行沙箱，通过轮询 `/v1/mind/todo/pop` 获取任务，执行后通过 `/v1/mind/report` 汇报结果；
- **Helix-Tentacle**（未来集成）：触手探查器，可对超长文档进行渐进式脱水，辅助大脑处理复杂输入。

---

## 9 学术术语表

| 术语 | 标准释义 |
| :--- | :--- |
| **基因锁** | 智能体不可违背的底层规则，等同于“思想钢印” |
| **海马体** | 短期记忆存储区，记录近期对话、任务执行与异常 |
| **长期记忆** | 经过提炼、压缩的经验与知识，供后续检索 |
| **任务队列** | 大脑拆解后的原子任务列表，采用 Markdown 格式存储 |
| **人格服务** | 通过 API 提供 system_prompt 与参数，供 Tuck 动态注入 |
| **认知闭环** | 需求入栈 → 大脑拆解 → 任务执行 → 结果汇报 → 记忆归档 |

---

## 10 项目总结

Helix-Mind 并非简单的“记忆插件”，而是为硅基生命构建的**认知中枢**。它以 Markdown 文件为底，以文件锁为盾，以大脑模型为引擎，将模糊的需求转化为可执行的原子任务，并将执行结果沉淀为可回溯的记忆。与 Tuck 网关、Anaphase 执行沙箱共同构成三位一体分布式架构，让智能体在私有边缘环境中实现**自主进化、经验传承、资源最优**的长期目标。

未来将引入史官模块，实现海马体的自动压缩与长期记忆的智能检索，进一步增强认知中枢的自我进化能力。

---

**相关项目**
- [Tuck](https://github.com/Jasonmilk/Tuck) — 模型调度与安全网关
- [Anaphase-Helix](https://github.com/Jasonmilk/Anaphase-Helix) — 自主进化型硅基智能体系统
- [Helix-Tentacle](https://github.com/Jasonmilk/Helix-Tentacle) — 渐进式文档探查与脱水器官

**许可证**  
MIT © Jasonmilk
