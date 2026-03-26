import os
import re
from pathlib import Path
import logging
from typing import Optional, Dict

logger = logging.getLogger("mind.dag")

class DAGManager:
    def __init__(self, base_dir="memory_base/dag_universe"):
        self.base_dir = Path(base_dir).resolve()
        self.l0_dir = self.base_dir / "L0_Axioms"
        self.l1_dir = self.base_dir / "L1_Theorems"
        self.l0_dir.mkdir(parents=True, exist_ok=True)
        self.l1_dir.mkdir(parents=True, exist_ok=True)
        
        self._index_cache: Dict[str, dict] = {}
        self._build_index()

    def _sanitize_id(self, raw_id: str) -> str:
        return re.sub(r'[^a-zA-Z0-9_]', '', str(raw_id).strip())

    def _build_index(self):
        self._index_cache.clear()
        for directory in [self.l0_dir, self.l1_dir]:
            if not directory.exists(): continue
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
        if not self._index_cache: self._build_index()
        index_lines =["# DAG 全局地图 (仅节点ID与标题)"]
        for node_id, data in self._index_cache.items():
            index_lines.append(f"- [{node_id}]: {data['title']}")
        return "\n".join(index_lines)

    def fetch_node(self, node_id: str) -> str:
        clean_id = self._sanitize_id(node_id)
        node_info = self._index_cache.get(clean_id)
        if not node_info:
            self._build_index()
            node_info = self._index_cache.get(clean_id)
            
        if not node_info or not node_info["path"].exists():
            return f"[DAG Error] 未找到节点: {clean_id}。"
            
        try:
            content = node_info["path"].read_text(encoding="utf-8")
            return content[:1500] + "\n...[已截断]" if len(content) > 1500 else content
        except Exception as e:
            return f"[DAG Error] 读取物理失败: {e}"

    def write_node(self, node_id: str, title: str, parents: str, content: str) -> str:
        clean_id = self._sanitize_id(node_id)
        if not clean_id: return "[DAG Error] 非法节点 ID。"
        
        file_path = self.l1_dir / f"{clean_id}.md"
        clean_parents = ",".join([self._sanitize_id(p) for p in parents.split(",") if p.strip()])
        md_content = f"---\nid: {clean_id}\ntype: L1_Theorem\nparents: [{clean_parents}]\n---\n# {title.strip()}\n\n{content.strip()}\n"
        
        try:
            file_path.write_text(md_content, encoding="utf-8")
            self._index_cache[clean_id] = {"title": title.strip(), "path": file_path}
            return f"[DAG Success] 节点 '{clean_id}' 已固化。"
        except Exception as e:
            return f"[DAG Error] 写入失败: {e}"

    def get_node_path(self, node_id: str) -> str:
        """【审查吸收】：提供合法的物理路径获取接口"""
        clean_id = self._sanitize_id(node_id)
        node_info = self._index_cache.get(clean_id)
        if not node_info:
            self._build_index()
            node_info = self._index_cache.get(clean_id)
        return str(node_info["path"].resolve()) if node_info else ""
