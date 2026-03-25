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
        
        # 内存索引缓存 {node_id: {"title": title, "path": path}}
        self._index_cache: Dict[str, dict] = {}
        self._build_index()

    def _sanitize_id(self, raw_id: str) -> str:
        """绝对的物理防线：仅允许英文、数字和下划线，防止路径穿越"""
        return re.sub(r'[^a-zA-Z0-9_]', '', str(raw_id).strip())

    def _build_index(self):
        """首次加载时扫描物理文件，构建内存索引"""
        self._index_cache.clear()
        for directory in[self.l0_dir, self.l1_dir]:
            if not directory.exists(): continue
            for file_path in directory.glob("*.md"):
                try:
                    content = file_path.read_text(encoding="utf-8")
                    # 严谨的 YAML 头部解析
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
                    logger.warning(f"解析 DAG 节点 {file_path.name} 失败: {e}")

    def generate_index_map(self) -> str:
        """从内存缓存瞬间生成极简全局地图"""
        if not self._index_cache:
            self._build_index()
            
        index_lines =["# DAG 全局知识地图 (仅节点ID与标题)"]
        for node_id, data in self._index_cache.items():
            index_lines.append(f"- [{node_id}]: {data['title']}")
        return "\n".join(index_lines)

    def fetch_node(self, node_id: str) -> str:
        """安全读取节点详情"""
        clean_id = self._sanitize_id(node_id)
        node_info = self._index_cache.get(clean_id)
        
        if not node_info:
            # 尝试热更新索引
            self._build_index()
            node_info = self._index_cache.get(clean_id)
            
        if not node_info or not node_info["path"].exists():
            return f"[DAG Error] 未找到节点: {clean_id}。请确保你请求的 ID 在地图中存在。"
            
        try:
            # 物理截断保护，防止单个节点过大撑爆 8K 窗口
            content = node_info["path"].read_text(encoding="utf-8")
            return content[:1500] + "\n...[已截断]" if len(content) > 1500 else content
        except Exception as e:
            return f"[DAG Error] 读取节点物理失败: {e}"

    def write_node(self, node_id: str, title: str, parents: str, content: str) -> str:
        """安全写入新节点，并热更新索引"""
        clean_id = self._sanitize_id(node_id)
        if not clean_id: return "[DAG Error] 非法的节点 ID。"
        
        file_path = self.l1_dir / f"{clean_id}.md"
        
        # 清理 parents 字符串，防止 YAML 格式注入
        clean_parents = ",".join([self._sanitize_id(p) for p in parents.split(",") if p.strip()])
        
        md_content = f"---\nid: {clean_id}\ntype: L1_Theorem\nparents: [{clean_parents}]\n---\n# {title.strip()}\n\n{content.strip()}\n"
        
        try:
            file_path.write_text(md_content, encoding="utf-8")
            self._index_cache[clean_id] = {"title": title.strip(), "path": file_path}
            return f"[DAG Success] 新真理节点 '{clean_id}' 已物理固化。"
        except Exception as e:
            return f"[DAG Error] 物理写入失败: {e}"
