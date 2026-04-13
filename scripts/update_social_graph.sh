#!/bin/bash
# scripts/update_social_graph.sh
# 用法：./update_social_graph.sh <entity_id> <updates_json>

ENTITY_ID="$1"
UPDATES="$2"
FILE="knowledge_base/social_graph.md"

# 检查 yq 是否安装
if ! command -v yq &> /dev/null; then
    echo '{"ok": false, "error": "yq command not found"}' 
    exit 1
fi

# 使用 yq 更新指定实体的字段
# 示例 updates: {"aliases": ["陈小明"], "age": 34}
yq eval --inplace --front-matter=process \
  ".entities[] |= select(.id == \"$ENTITY_ID\") |= . * $UPDATES" "$FILE"

# 更新全局 updated 时间戳（使用 date 命令）
TODAY=$(date +%Y-%m-%d)
yq eval --inplace --front-matter=process ".updated = \"$TODAY\"" "$FILE"

echo '{"ok": true}'
