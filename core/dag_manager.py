import os
import re
import logging
from pathlib import Path
from typing import Dict, Optional

logger = logging.getLogger("mind.dag")

class DAGManager:
    def __init__(self, base_dir="memory_base/dag_universe"):
        self.base_dir = Path(base_dir).resolve()
        self.l0_dir = self.base_dir / "L0_Axioms"
        self.l1_dir = self.base_dir / "L1_Theorems"
        self.l2_dir = self.base_dir / "L2_Engineering"   # 新增工程蓝图层
        
        self.l0_dir.mkdir(parents=True, exist_ok=True)
        self.l1_dir.mkdir(parents=True, exist_ok=True)
        self.l2_dir.mkdir(parents=True, exist_ok=True)
        
        self._index_cache: Dict[str, dict] = {}
        self._build_index()

    def _sanitize_id(self, raw_id: str) -> str:
        """仅允许字母、数字、下划线"""
        return re.sub(r'[^a-zA-Z0-9_]', '', str(raw_id).strip())

    def _build_index(self):
        """构建内存索引，包含 L0, L1, L2"""
        self._index_cache.clear()
        for directory in [self.l0_dir, self.l1_dir, self.l2_dir]:
            if not directory.exists():
                continue
            for file_path in directory.glob("*.md"):
                try:
                    content = file_path.read_text(encoding="utf-8")
                    if content.startswith("---"):
                        parts = content.split("---", 2)
                        if len(parts) >= 3:
                            yaml_header = parts[1]
                            body = parts[2]
                            id_match = re.search(r"id:\s*([a-zA-Z0-9_]+)", yaml_header)
                            title_match = re.search(r"^#\s+(.+)$", body, re.MULTILINE)
                            if id_match and title_match:
                                node_id = id_match.group(1).strip()
                                self._index_cache[node_id] = {
                                    "title": title_match.group(1).strip(),
                                    "path": file_path
                                }
                except Exception as e:
                    logger.warning(f"解析节点 {file_path.name} 失败: {e}")

    def generate_index_map(self) -> str:
        """返回全局地图（仅ID与标题）"""
        if not self._index_cache:
            self._build_index()
        index_lines = ["# DAG 全局知识地图 (仅节点ID与标题)"]
        for node_id, data in self._index_cache.items():
            index_lines.append(f"- [{node_id}]: {data['title']}")
        return "\n".join(index_lines)

    def fetch_node(self, node_id: str) -> str:
        """读取节点内容（截断至1500字符）"""
        clean_id = self._sanitize_id(node_id)
        node_info = self._index_cache.get(clean_id)
        if not node_info:
            self._build_index()
            node_info = self._index_cache.get(clean_id)
        if not node_info or not node_info["path"].exists():
            return f"[DAG Error] 未找到节点: {clean_id}。"
        try:
            content = node_info["path"].read_text(encoding="utf-8")
            if len(content) > 1500:
                return content[:1500] + "\n...[已截断]"
            return content
        except Exception as e:
            return f"[DAG Error] 读取物理失败: {e}"

    def write_node(self, node_id: str, title: str, parents: str, content: str, layer: str = "L1_Theorems") -> str:
        """
        写入新节点，支持指定层级（L1_Theorems 或 L2_Engineering）
        """
        clean_id = self._sanitize_id(node_id)
        if not clean_id:
            return "[DAG Error] 非法节点 ID。"

        if layer == "L2_Engineering":
            target_dir = self.l2_dir
        else:
            target_dir = self.l1_dir

        file_path = target_dir / f"{clean_id}.md"
        clean_parents = ",".join([self._sanitize_id(p) for p in parents.split(",") if p.strip()])
        md_content = f"---\nid: {clean_id}\ntype: {layer}\nparents: [{clean_parents}]\n---\n# {title.strip()}\n\n{content.strip()}\n"

        try:
            file_path.write_text(md_content, encoding="utf-8")
            self._index_cache[clean_id] = {"title": title.strip(), "path": file_path}
            return f"[DAG Success] 节点 '{clean_id}' 已固化至 {layer}。"
        except Exception as e:
            return f"[DAG Error] 写入失败: {e}"

    def get_node_path(self, node_id: str) -> str:
        """返回节点绝对路径，若不存在返回空字符串"""
        clean_id = self._sanitize_id(node_id)
        node_info = self._index_cache.get(clean_id)
        if not node_info:
            self._build_index()
            node_info = self._index_cache.get(clean_id)
        return str(node_info["path"].resolve()) if node_info else ""
