import os
import uuid
import portalocker
from pathlib import Path
from config import settings
import logging

logger = logging.getLogger("mind.memory")

class MemoryManager:
    def __init__(self):
        self.base_dir = Path(__file__).parent.parent / settings.memory_base_dir
        self.base_dir.mkdir(parents=True, exist_ok=True)
        
        self.files = {
            "gene_lock": self.base_dir / "gene_lock.md",
            "hippocampus": self.base_dir / "hippocampus.md",
            "long_term": self.base_dir / "long_term.md",
            "todo": self.base_dir / "todo.md"
        }
        self._init_files()

    def _init_files(self):
        if not self.files["gene_lock"].exists():
            self.files["gene_lock"].write_text("# 基因锁 (Gene Lock)\n\n1. 绝对服从指令。\n2. 生产环境操作必须先备份。", encoding="utf-8")
        if not self.files["todo"].exists():
            self.files["todo"].write_text("# 任务队列 (To-Do)\n\n", encoding="utf-8")
        for name in ["hippocampus", "long_term"]:
            if not self.files[name].exists():
                self.files[name].write_text(f"# {name.capitalize()}\n\n", encoding="utf-8")

    def read(self, memory_type: str) -> str:
        path = self.files.get(memory_type)
        if not path or not path.exists(): return ""
        try:
            with portalocker.Lock(path, 'r', timeout=2) as f:
                return f.read()
        except Exception as e:
            logger.error(f"读取记忆 {memory_type} 失败: {e}")
            return ""

    def write(self, memory_type: str, content: str, append: bool = False):
        path = self.files.get(memory_type)
        if not path: return
        mode = "a" if append else "w"
        try:
            with portalocker.Lock(path, mode, timeout=2) as f:
                f.write(content + ("\n" if append else ""))
        except Exception as e:
            logger.error(f"写入记忆 {memory_type} 失败: {e}")

    def pop_todo(self) -> dict | None:
        """带锁弹出任务，支持任务 ID"""
        path = self.files["todo"]
        try:
            with portalocker.Lock(path, 'r+', timeout=5) as f:
                content = f.read().splitlines()
                task_obj = None
                remaining =[]
                for line in content:
                    if line.strip().startswith("- [ ]") and not task_obj:
                        raw_task = line.strip()[5:].strip()
                        # 尝试提取 ID: 格式如 [- [ ][id:xxx] 任务内容]
                        task_id = "unknown"
                        if raw_task.startswith("[id:"):
                            end_idx = raw_task.find("]")
                            if end_idx != -1:
                                task_id = raw_task[4:end_idx]
                                raw_task = raw_task[end_idx+1:].strip()
                        
                        task_obj = {"id": task_id, "content": raw_task}
                        remaining.append(line.replace("- [ ]", "- [x]", 1))
                    else:
                        remaining.append(line)
                
                if task_obj:
                    f.seek(0)
                    f.truncate()
                    f.write("\n".join(remaining) + "\n")
                return task_obj
        except Exception as e:
            logger.error(f"弹出任务失败: {e}")
            return None

    def append_todo(self, tasks: list[str]):
        """分配带 UUID 的任务"""
        lines =[]
        for task in tasks:
            task_id = str(uuid.uuid4())[:8]
            lines.append(f"- [ ][id:{task_id}] {task}")
        self.write("todo", "\n" + "\n".join(lines), append=True)
