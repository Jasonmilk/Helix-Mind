## [2026-08-28] 完成：P9 认知工艺 Phase 3 — 完整引擎（价值/突变/复盘/bm25）

### 触发条件
P8 编排器闭环验证通过（DeterministicAdapter 0 Token），用户授权 Phase 3 编码（价值评估、自适应突变、睡眠复盘、bm25 门控增强）。

### 变更性质
- **价值评估**（`value.rs`）：`ValueAssessor` 确定性评估（长度档 + 复杂关键词提升）→ `ValueGrade{Low,Medium,High}` + 建议思考深度（Skilled/Anchored/Imaginative）。0 Token，禁用 LLM 自我评分（R3，信用由他证）
- **自适应突变**（`mutation.rs`）：`AdaptiveMutation`——Bounded ε-Greedy（突变率钳制 3%-20%）+ EMA 成功率平滑（α=0.3）；`record_outcome` 确定性信号更新：成功→降探索（利用熟练路径），失败→升探索（探索新工序/模式）
- **睡眠复盘**（`review.rs`）：`SleepReview`——非熟练模式检视（强制锚定/想象力，防假复盘 R4）：检视输出实体覆盖度相对旧路径增量 ≥2 → Stale（旧路径僵化，建议降权）；否则 Viable。接入点：Deep Dream 代谢窗口（不阻塞主循环）
- **bm25 门控增强**（`gate.rs`）：`system0_gate_enhanced`——用户显式标签 > bm25 简单相似度（≥0.7 → DirectSkilled）> 价值评估（Low→Direct）> 规则兜底。相似度信号由上层经 `storage::fts::fts_search` 计算注入（cognitive 保持纯逻辑，零向量依赖）

### 兼容性
- 全为 cognitive crate 内新模块，零既有代码改动；硬冻结契约零变更；无新依赖

### 验收
`cargo test --workspace` 全绿（cognitive 单元 12→24 + 集成 4，workspace 总 98，0 warning）｜ 价值：短简 Low/带关键词 Medium/长+关键词 High ✅ ｜ 突变：界内钳制、成功降探索失败升探索、EMA 平滑 ✅ ｜ 复盘：增量≥2 Stale / 否则 Viable ✅ ｜ 门控：bm25 高相似优先、显式标签最高优先 ✅

### 状态
🧬 待提交（用户确认后 push）
