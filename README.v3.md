# Helix-Ana: Anaphase 执行引擎

Helix 生态的 Harness 层入口，负责 Agent Loop、工具编排与审计核心。

## 🧠 核心能力演进

### v2.2 - 容错增强与社交图谱查询 (当前版本)

- **JSON 解析容错**: 支持自动剥离 Markdown 代码块标记 (` ```json `)，提升非标准输出兼容性
- **参数名自动修正**: 兼容常见参数名错误 (`node_name`→`node_id`, `task`→`summary`, `entity`→`entity_id`, `attributes`→`properties`)
- **新增 query_social_graph 工具**: 只读工具，支持通过姓名或别名检索社交图谱实体
- **社交上下文匹配优化**: 从精确匹配改为忽略大小写的子串匹配，显著提高命中率
- **基因锁宪法 (L0)**: 从 `config/gene_lock.md` 加载核心行为准则，注入所有推理环节
- **杏仁核模块增强**: 英文 Prompt，支持意图分类 (`intent_category`)，接收基因锁参数
- **L1 自我画像**: 英文化，从 `knowledge_base/self.md` 加载核心特质与回复风格
- **社交图谱**: 英文化，从 `knowledge_base/social_graph.md` 管理人际关系网络
- **动态上下文组装**: 根据情绪标签动态生成 `conversation_style`，按需加载记忆与社交信息
- **动态模型路由**: 根据任务紧急度自动选择合适模型
- **工具箱集成**: 支持外部 CLI 工具安全调用

### v2.1 - 基因锁增强版

- **基因锁宪法 (L0)**: 从 `config/gene_lock.md` 加载核心行为准则，注入所有推理环节
- **杏仁核模块增强**: 英文 Prompt，支持意图分类 (`intent_category`)，接收基因锁参数
- **L1 自我画像**: 英文化，从 `knowledge_base/self.md` 加载核心特质与回复风格
- **社交图谱**: 英文化，从 `knowledge_base/social_graph.md` 管理人际关系网络
- **动态上下文组装**: 根据情绪标签动态生成 `conversation_style`，按需加载记忆与社交信息
- **动态模型路由**: 根据任务紧急度自动选择合适模型
- **工具箱集成**: 支持外部 CLI 工具安全调用

### v2.0 - 杏仁核 + 自我认知

- **杏仁核模块**: 基于 LLM 的任务优先级评估与情绪识别
- **L1 自我画像**: 从 `knowledge_base/self.md` 加载核心特质与回复风格
- **社交图谱**: 从 `knowledge_base/social_graph.md` 管理人际关系网络
- **动态模型路由**: 根据任务紧急度自动选择合适模型
- **工具箱集成**: 支持外部 CLI 工具安全调用

### v1.0 - 基础执行引擎

- **Agent Loop**: ReAct 风格的感知→推理→工具调用→反馈循环
- **Harness**: 工具拦截、Schema 校验、L0 基因锁校验、CLI 执行
- **Tool Registry**: 基于 YAML + Pydantic 的强类型工具注册表
- **HXR Logger**: 结构化日志记录器，支持推理链路回放
- **Tuck Backend**: 模型网关适配层，支持约束解码

---

## 快速启动

### 1. 环境准备

```bash
# 创建虚拟环境
python -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate

# 安装依赖
pip install -e ".[dev]"

# 复制配置文件
cp .env.example .env

# 安装系统依赖（用于社交图谱更新）
wget https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -O /usr/local/bin/yq
chmod +x /usr/local/bin/yq

# 安装 Python frontmatter 库
pip install python-frontmatter
```

### 2. 验证安装

```bash
# 检查 CLI 是否正常
ana --help

# 验证版本
ana --version
```

### 3. 运行任务

```bash
# 执行任务（LLM 模式）
ana run "查询知识库中关于 Python 的内容"

# 直通模式（绕过 LLM）
ana run "echo hello" --direct

# JSON 输出
ana run "echo hello" --json
```

---

## 命令树

```
ana
├── run <task>                    # 启动 Agent Loop，执行任务
├── queue                         # 任务队列管理
│   ├── ls                        # 列出队列状态
│   └── cancel <task_id>          # 取消任务
├── kb                            # 知识库管理
│   ├── fetch <node_id>           # 检索节点
│   ├── query <keyword>           # 关键词查询
│   ├── write <...>               # 写入记忆
│   └── snapshot                  # 创建快照
├── tentacle <keyword>            # 外部搜索与脱水
├── trace <session_id>            # 回放推理链路
├── lock                          # 基因锁管理
│   ├── ls                        # 列出所有基因锁
│   └── reload                    # 重新编译 L0 规则
├── stats                         # 系统统计
└── --version / --help            # 版本与帮助
```

---

## 核心组件

### Agent Loop 流程

1. **感知 (_perceive)**: 
   - 调用杏仁核评估任务优先级、情绪标签和意图类别（注入基因锁）
   - 根据情绪标签动态生成 `conversation_style`
   - 按需加载自画像 (`self_portrait`)
   - 检测是否涉及已知人物，加载社交上下文 (`social_context`)
   - 若意图为 `chat`，检索相关记忆 (`memory_context`)

2. **推理 (_think)**: 
   - 构建 System Prompt，注入基因锁、自我认知、社交上下文和动态对话风格
   - 根据优先级动态选择模型
   - 调用 LLM 生成思考内容

3. **行动 (_act)**: 
   - 解析工具调用请求
   - 通过 Harness 执行工具（CLI / API / 内部函数）
   - 收集执行结果

4. **反馈 (_feedback)**: 
   - 将结果反馈给 LLM
   - 判断是否需要继续迭代

### 新增模块

- **基因锁 (`config/gene_lock.md`)**: L0 核心宪法，定义 Helix 的基本行为准则
- **杏仁核 (`ana/core/amygdala.py`)**: 价值评估与优先级计算（支持意图分类）
- **自画像 (`knowledge_base/self.md`)**: L1 层级自我认知（英文化）
- **社交图谱 (`knowledge_base/social_graph.md`)**: 人际关系网络（英文化）
- **图谱更新工具 (`scripts/update_social_graph.sh`)**: 安全的实体属性更新脚本
- **图谱查询工具 (`query_social_graph`)**: 只读工具，通过姓名或别名检索实体信息

### v2.2 增强特性

- **JSON 解析容错**: `DualTagParser.parse` 自动剥离 Markdown 代码块标记后重试
- **参数名自动修正**: 兼容 `node_name`→`node_id`, `task`→`summary`, `entity`→`entity_id`, `attributes`→`properties`
- **社交上下文匹配优化**: `_load_social_context` 使用忽略大小写的子串匹配，提高命中率

---

## 配置说明

编辑 `.env` 文件配置服务端点和模型路由：

```bash
# Tuck 模型网关
TUCK_ENDPOINT=http://localhost:8686/v1/chat/completions

# Helix-Mind 记忆服务
HELIX_MIND_ENDPOINT=http://localhost:8020

# 模型路由（按优先级）
ANA_CEREBELLUM_MODEL=qwen2.5-coder:7b    # 低优先级任务
ANA_LEFT_BRAIN_MODEL=qwen2.5-coder:7b    # 中优先级任务
ANA_RIGHT_BRAIN_MODEL=qwen-max           # 高优先级/创意任务
```

完整配置参考 `.env.example`。

---

## 知识库结构

### self.md - 自画像

```markdown
---
updated: 2026-04-13
core_traits: ["independent_thinking", "tool_proficiency", "pursuit_of_elegance"]
reply_style: "concise and restrained, unless user is relaxed"
---
# Helix Self Portrait

## Behavioral Patterns
- [[tends_to_guide_with_questions_rather_than_direct_answers]]
- [[offers_brief_empathy_when_user_is_down]]
- [[switches_to_focus_mode_for_explicit_tasks]]

## Long-term Goals
- [[grow_and_achieve_together_with_user]]
```

### social_graph.md - 社交图谱

```markdown
---
updated: 2026-04-13
entities:
  - id: "person_user"
    name: "User"
    type: "user"
relationships: []
---
# Social Graph
```

### 更新社交图谱

```bash
# 更新实体属性
./scripts/update_social_graph.sh person_user '{"aliases": ["陈小明"], "age": 34}'
```

### 查询社交图谱

```bash
# 使用 query_social_graph 工具（在 Agent 任务中调用）
ana run "查询关于 User 的信息"

# 工具会返回匹配的实体信息，支持姓名或别名匹配
```

---

## 开发

```bash
# 代码格式化
ruff format .

# 代码检查
ruff check .

# 运行测试
pytest
```

---

## 架构定位

- **Anaphase (helix-ana)** = 执行分离层（Harness 执行体）
- **Helix-Mind** = 记忆系统微服务（第三卷）
- **Tuck** = 模型网关（复用）
- **Tentacle** = 感知器官（封装复用）
- **Amygdala** = 价值评估中枢（增强版，支持意图分类）
- **Gene Lock** = L0 核心宪法（新增）
- **Knowledge Base** = L1 自我认知与社交网络（英文化）
- **Tool Registry** = 工具注册表（v2.2 新增 `query_social_graph`）
- **Harness Parser** = 解析器（v2.2 增强 JSON 容错与参数修正）

---

## 许可证

MIT
