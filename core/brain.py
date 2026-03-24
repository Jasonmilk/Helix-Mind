import httpx
import json
import re
import logging
import asyncio
from core.memory_manager import MemoryManager
from config import settings

logger = logging.getLogger("mind.brain")

class Brain:
    def __init__(self, memory: MemoryManager):
        self.memory = memory
        self.model = settings.brain_model

    async def decompose_task(self, user_requirement: str):
        gene_lock = self.memory.read("gene_lock")
        hippocampus = self.memory.read("hippocampus")

        prompt = (
            "你现在是 Helix 的【大脑】。你需要将用户的模糊需求拆解为可由【手脚(执行单元)】一步步执行的具体动作。\n"
            f"【基因锁限制】:\n{gene_lock}\n\n"
            f"【当前短期记忆】:\n{hippocampus}\n\n"
            f"【用户需求】:\n{user_requirement}\n\n"
            "请输出 JSON 格式的数组，每个元素是一个独立且具体的动作指令。例如：\n"
            '["读取 config.yaml 文件", "修改 port 为 8080", "重启服务"]\n'
            "不要输出任何其他废话，只输出纯 JSON 数组。"
        )

        headers = {"Content-Type": "application/json"}
        if settings.tuck_api_key:
            headers["Authorization"] = f"Bearer {settings.tuck_api_key}"
            
        payload = {"model": self.model, "messages":[{"role": "user", "content": prompt}], "temperature": 0.2}

        # 重试机制
        for attempt in range(settings.max_retries):
            try:
                async with httpx.AsyncClient(timeout=120.0) as client:
                    resp = await client.post(settings.tuck_url, json=payload, headers=headers)
                    resp.raise_for_status()
                    reply = resp.json()["choices"][0]["message"]["content"]
                    
                    tasks = self._extract_json_array(reply)
                    if tasks:
                        self.memory.append_todo(tasks)
                        logger.info(f"大脑成功拆解 {len(tasks)} 个任务。")
                        return tasks
                    else:
                        raise ValueError("无法从模型回复中提取有效的 JSON 数组。")
                        
            except Exception as e:
                logger.warning(f"大脑思考失败 (尝试 {attempt + 1}/{settings.max_retries}): {e}")
                if attempt == settings.max_retries - 1:
                    # 彻底失败，写入精简错误到海马体
                    error_msg = f"【大脑拆解彻底失败】: 需求 '{user_requirement[:20]}...', 报错: {str(e)}"
                    self.memory.write("hippocampus", error_msg, append=True)
                await asyncio.sleep(2) # 退避重试
        return[]

    def _extract_json_array(self, text: str) -> list | None:
        """强健的正则 JSON 提取器"""
        try:
            # 1. 尝试直接解析
            return json.loads(text)
        except json.JSONDecodeError:
            # 2. 正则寻找方括号包裹的数组内容
            match = re.search(r'\[.*\]', text, re.DOTALL)
            if match:
                try:
                    return json.loads(match.group(0))
                except:
                    pass
        return None
