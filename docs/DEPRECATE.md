# 凋亡清单

规则：每条必须有明确死期。已安葬项移入 `archive/deprecated/`。

---

## DEP-001: Forget 接口物理删除语义

- **原因**：与"记忆不可篡改"冲突
- **替代**：突触切断 + 遗忘标记（`is_recessive=true`）
- **截止**：2026-09-30
- **状态**：⏳ 迁移中

## DEP-002: 定时轮询/心跳探测

> **已移除**：移入 `docs/archive/deprecated/DEP-002.md`（2026-08-25）

## DEP-003: Anaphase 本地 YAML 工具配置

- **原因**：与 Tentacle `.manifest.json` 双源冲突
- **替代**：`.manifest.json` 唯一真理源
- **截止**：2026-09-30
- **状态**：⏳ 迁移中
