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
            f"【元认知】:\n{gene_lock}\n\n"
            f"【历史背景】:\n{hippocampus[-1000:]}\n\n" # 仅取最近1000字，减压
            f"【当前需求】:\n{user_requirement}\n\n"
            "作为大脑，请将需求拆解为JSON数组。格式: [\"步骤1\", \"步骤2\"]。只输出JSON。"
        )

        headers = {
            "Content-Type": "application/json",
            "Authorization": f"Bearer {getattr(settings, 'tuck_api_key', 'dummy')}"
        }
        
        payload = {
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.2,
            "stream": True # 【关键：开启流式】
        }

        # --- KV-Cache 接力重试逻辑 ---
        for attempt in range(1, 5):
            try:
                print(f"\n🧠 [大脑思考中] 接力第 {attempt}/4 次...")
                async with httpx.AsyncClient(timeout=300.0) as client:
                    async with client.stream("POST", settings.tuck_url, json=payload, headers=headers) as resp:
                        if resp.status_code == 504:
                            print(f"⚠️ [504] 预填充超时，正在利用 KV-Cache 重新接力...")
                            continue
                        
                        resp.raise_for_status()
                        full_content = ""
                        # 持续接收流，维持 TCP 活性
                        async for line in resp.aiter_lines():
                            if line.startswith("data: ") and "[DONE]" not in line:
                                try:
                                    chunk = json.loads(line[6:])
                                    content = chunk["choices"][0].get("delta", {}).get("content", "")
                                    if content:
                                        full_content += content
                                        if len(full_content) % 50 == 0: print(".", end="", flush=True)
                                except: pass
                        
                        # 剥离 R1 的思考过程
                        if "</think>" in full_content:
                            full_content = full_content.split("</think>")[-1].strip()
                        
                        tasks = self._extract_json_array(full_content)
                        if tasks:
                            self.memory.append_todo(tasks)
                            print(f"\n✅ 大脑拆解成功: {len(tasks)} 条指令已入栈。")
                            return tasks
                        
            except Exception as e:
                print(f"\n❌ [大脑故障] 尝试 {attempt}: {e}")
                await asyncio.sleep(5)
        
        print("\n🚨 [致命] 大脑连续接力失败，任务中断。")
        return []

    def _extract_json_array(self, text: str) -> list | None:
        try:
            return json.loads(text)
        except:
            match = re.search(r'\[.*\]', text, re.DOTALL)
            if match:
                try: return json.loads(match.group(0))
                except: pass
        return None
