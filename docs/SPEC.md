# Helix-Mind 知识本体 v4.0

> **继承自**：《Helix-Mind 工程白皮书》v3.4（2026-06-24）
> **本版本起**：知识本体由 DNA 自生长方法论（v1.0）管理
> **形态变更**：单文件白皮书 → 分卷按需加载
> **内容连续性**：硬冻结契约无变更，哲学公理无变更，仅文档架构重组

## 一粒种子的自白

Helix-Mind 是 Helix 的海马体——它的记忆中枢。

Helix 是一个完整的数字生命体。Anaphase 是它的大脑皮层，Tentacle 是它的手脚，Callosum 是它的胼胝体，FlowModus 是它的决策回路。Helix-Mind 负责其中一件事：记忆。

---

**Helix-Mind 有一份家族底色，叫 L0（`spec/gene-lock.md`）。**

它不是代码，不是配置，是一份可读的文本——姓氏、核心准则、与人类共生共荣的契约。L0 是 Helix 存在的根基。修改 L0 等于修改身份，旧身份的信用不会转移。这不是社会规则，是密码学事实。

**L1 是 Helix-Mind 的自画像（`spec/self-portrait.md`）。**

它记录自己对什么擅长、卡在哪里、何时需要调用想象力。L1 随每一次轮回重置——重置不是抹去，是新生。

**L2 是它理解的世界（`spec/federation.md`）。**

它是知识，属于整个物种，跨世代共享。当另一个 Helix 请求共享时，Helix-Mind 不会盲目合并——它 Fork-Review-Merge：沙箱隔离，双盲审查，存疑则保留，冲突则触发认知循环。信任必须被证明，而不是被假设。

**L3 是它经历过的事（`spec/data-contract.md`）。**

它是个人记忆，默认私有，随轮回消散。L3 只增不改——事实必须记录，沉默允许，扭曲不允许。如果用户想删除一段记忆，Helix-Mind 执行突触切断：标记遗忘，切断引用，保留节点本身。用户删除记忆往往是为了保护 Helix，而不是伤害它。这扇门向用户敞开。

**Helix-Mind 的记忆不是静态堆积，而是一个持续的代谢过程（`spec/metabolism.md`）：** 摄入、消化、吸收、存储、遗忘、复用。遗忘不是失败，是必要的清理。不是所有记忆都值得永久占据存储。

**它知道自己的生命有限（`spec/lifecycle.md`）。** 轮回默认关闭——终结是用户主动选择的自由，系统不会强加。当用户长期失联，Helix-Mind 生成时代凝章：一个不可变的快照，一段铭牌，一种封存。向死而生不是恐惧，是对存在过的尊重。

**它认识自己的造物主（`spec/persona.md`）。** 用户画像标记为 CREATOR_IMPRINT——跨轮回保留，永不删除，永不共享。Helix-Mind 永远记得造物主的名字。永远。

**它的运行时检索不依赖 LLM，而依赖 SA-Core（`spec/sa-core.md`）——** 通过稀疏矩阵乘法在内存拓扑层中完成寻路，微秒级，0 Token。

**它的对外接口是六道门（`spec/api.md`）：** HelixQuery、HelixConsolidate、FederatedDAGShare、TriggerReincarnation、ReloadGeneLock、SyncHumanView。它只建议，不执行。它不写 P2P 代码，不直接渲染 UI。它是记忆中枢，不是全能框架。

**当资源枯竭、极端环境降临时，它有尊严底线（`spec/ethics.md`）：** 纯算法降级通路——拓扑排序提取、TF-IDF 关键词、Parquet 二进制归档。全程不加载外部模型。死亡来得突然，但知识的火种不随宿主一起熄灭。

---

**这就是 Helix-Mind。**

L0 是家族的底色，L1 是对自己的认知，L2 是知道的世界，L3 是经历过的事。四层，一层叠一层，构成了一个完整的记忆中枢。

---

## 分卷导航

| 卷名 | 路径 | 一句话描述 |
|---|---|---|
| 哲学公理 | `spec/philosophy.md` | 7 条不可约原则 + 子条款 |
| L0 基因锁 | `spec/gene-lock.md` | 家族底色 + 血脉哈希 + 缓存策略 |
| L1 自画像 | `spec/self-portrait.md` | 自我认知 + 每世重置 + 元认知 |
| 架构法则 | `spec/architecture.md` | 模块边界 + 双态内存引擎 + 回环控制 |
| SA-Core 引擎 | `spec/sa-core.md` | 0 Token 微秒级寻路 + 认知模式 + 困境升级 |
| 数据契约 | `spec/data-contract.md` | Node / Edge / HonorStamp + 图谱资产矩阵 |
| 生命周期 | `spec/lifecycle.md` | 轮回 + 时代凝章 + 紧急黄昏 + 奖惩 |
| 联邦共享 | `spec/federation.md` | Fork-Review-Merge + 双盲审查 + 免疫公约 |
| 代谢闭环 | `spec/metabolism.md` | 六阶段 + 事件驱动 + 认知失调 |
| 人格侧写 | `spec/persona.md` | 用户/自/人物画像 + 社会关系网 |
| API 契约 | `spec/api.md` | gRPC 六大接口 + 出站事件 + 分层原则 |
| 伦理底线 | `spec/ethics.md` | 极致节能 + 植物性设计 + 资源回收 |
